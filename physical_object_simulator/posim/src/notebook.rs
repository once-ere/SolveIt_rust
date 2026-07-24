//! Notebook-style REPL: numbered `In[n]`/`Out[n]` cells like a
//! Jupyter/Mathematica notebook. Enter executes the current line
//! (the terminal's equivalent of shift-enter); previous cells can be
//! revisited and edited with magics:
//!
//! - `%history`          — show all cells
//! - `%edit n <text>`    — replace cell n's input and re-execute it
//! - `%rerun n`          — execute cell n's input again
//! - `%save <file>`      — save all inputs as a replayable script
//! - `%load <file>`      — replay a script file
//! - `%reset`            — clear the simulator state
//! - `%quit` / `%exit`   — leave
//!
//! (Pure `std` has no raw terminal mode, so cursor-key cell navigation
//! is delegated to the JupyterLab front end via `posim --machine`.)

use std::io::{BufRead, Write};

use crate::vm::{execute_line, SimState, Value};

pub struct Cell {
    pub input: String,
    pub output: String,
    pub ok: bool,
}

pub struct Notebook {
    pub cells: Vec<Cell>,
    pub state: SimState,
}

impl Default for Notebook {
    fn default() -> Self {
        Self { cells: Vec::new(), state: SimState::default() }
    }
}

impl Notebook {
    /// Executes one input line as a new numbered cell; returns the
    /// rendered output lines to display.
    pub fn execute_cell(&mut self, input: &str) -> String {
        let n = self.cells.len() + 1;
        let (output, ok) = match execute_line(input, &mut self.state) {
            Ok(Value::Unit) => (String::new(), true),
            Ok(v) => (v.to_string(), true),
            Err(e) => (e, false),
        };
        let rendered = if output.is_empty() {
            String::new()
        } else if ok {
            format!("Out[{n}]= {output}")
        } else {
            format!("Err[{n}]: {output}")
        };
        self.cells.push(Cell { input: input.to_string(), output, ok });
        rendered
    }

    /// Handles a `%magic` line; returns the text to display, or `None`
    /// if the notebook should quit.
    pub fn magic(&mut self, line: &str) -> Option<String> {
        let mut parts = line.splitn(2, char::is_whitespace);
        let cmd = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim().to_string();
        match cmd {
            "%quit" | "%exit" => None,
            "%history" => {
                let mut out = String::new();
                for (i, c) in self.cells.iter().enumerate() {
                    let marker = if c.ok { " " } else { "!" };
                    out.push_str(&format!("{marker}In[{}]:= {}\n", i + 1, c.input));
                    if !c.output.is_empty() {
                        let label = if c.ok { "Out" } else { "Err" };
                        out.push_str(&format!("  {label}[{}]= {}\n", i + 1, c.output));
                    }
                }
                if out.is_empty() {
                    out.push_str("(no history)\n");
                }
                out.pop();
                Some(out)
            }
            "%rerun" => match rest.parse::<usize>() {
                Ok(n) if n >= 1 && n <= self.cells.len() => {
                    let input = self.cells[n - 1].input.clone();
                    let echo = format!("In[{}]:= {}", self.cells.len() + 1, input);
                    let out = self.execute_cell(&input);
                    Some(if out.is_empty() { echo } else { format!("{echo}\n{out}") })
                }
                _ => Some(format!("%rerun: no cell {rest}")),
            },
            "%edit" => {
                let mut p = rest.splitn(2, char::is_whitespace);
                let idx = p.next().unwrap_or("").parse::<usize>();
                let new_text = p.next().unwrap_or("").trim().to_string();
                match idx {
                    Ok(n) if n >= 1 && n <= self.cells.len() && !new_text.is_empty() => {
                        self.cells[n - 1].input = new_text.clone();
                        let echo = format!("In[{}]:= {}", self.cells.len() + 1, new_text);
                        let out = self.execute_cell(&new_text);
                        Some(if out.is_empty() { echo } else { format!("{echo}\n{out}") })
                    }
                    Ok(n) if n >= 1 && n <= self.cells.len() => {
                        Some(format!("current In[{n}]:= {}\nusage: %edit {n} <new text>", self.cells[n - 1].input))
                    }
                    _ => Some("usage: %edit <cell number> <new text>".to_string()),
                }
            }
            "%save" => {
                if rest.is_empty() {
                    return Some("usage: %save <file>".to_string());
                }
                let mut body = String::new();
                for c in &self.cells {
                    if c.ok && !c.input.trim_start().starts_with('%') {
                        body.push_str(&c.input);
                        body.push('\n');
                    }
                }
                match std::fs::write(&rest, body) {
                    Ok(()) => Some(format!("saved {} cell(s) to {rest}", self.cells.len())),
                    Err(e) => Some(format!("%save failed: {e}")),
                }
            }
            "%load" => match std::fs::read_to_string(&rest) {
                Ok(text) => {
                    let mut shown = Vec::new();
                    for line in text.lines() {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            continue;
                        }
                        shown.push(format!("In[{}]:= {}", self.cells.len() + 1, line));
                        let out = self.execute_cell(line);
                        if !out.is_empty() {
                            shown.push(out);
                        }
                    }
                    Some(shown.join("\n"))
                }
                Err(e) => Some(format!("%load {rest} failed: {e}")),
            },
            "%reset" => {
                self.state = SimState::default();
                Some("system reset".to_string())
            }
            other => Some(format!("unknown magic `{other}` — see HELP")),
        }
    }
}

/// Interactive notebook loop over stdin/stdout.
pub fn repl() {
    let mut nb = Notebook::default();
    println!("posim — physical_object simulator notebook (sundials_rs backend)");
    println!("type HELP for the command language, %quit to leave\n");
    let stdin = std::io::stdin();
    loop {
        print!("In[{}]:= ", nb.cells.len() + 1);
        let _ = std::io::stdout().flush();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) => break, /* EOF */
            Ok(_) => {}
            Err(_) => break,
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('%') {
            match nb.magic(line) {
                Some(msg) => {
                    if !msg.is_empty() {
                        println!("{msg}");
                    }
                }
                None => break,
            }
        } else {
            let out = nb.execute_cell(line);
            if !out.is_empty() {
                println!("{out}");
            }
        }
    }
    println!("goodbye");
}

/// Batch mode: execute a script file, echoing cells.
pub fn run_script(path: &str) -> Result<(), String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let mut nb = Notebook::default();
    let mut failed = false;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        println!("In[{}]:= {}", nb.cells.len() + 1, line);
        if line.starts_with('%') {
            match nb.magic(line) {
                Some(msg) => {
                    if !msg.is_empty() {
                        println!("{msg}");
                    }
                }
                None => break,
            }
        } else {
            let out = nb.execute_cell(line);
            if !out.is_empty() {
                println!("{out}");
            }
            if let Some(c) = nb.cells.last() {
                if !c.ok {
                    failed = true;
                }
            }
        }
    }
    if failed {
        Err("script had failing cells".to_string())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells_number_and_capture_output() {
        let mut nb = Notebook::default();
        let out = nb.execute_cell("1 + 1");
        assert_eq!(out, "Out[1]= 2");
        let out = nb.execute_cell("new point { mass = 2 }");
        assert_eq!(out, "Out[2]= obj0");
        let out = nb.execute_cell("bogus syntax !!");
        assert!(out.starts_with("Err[3]:"), "{out}");
        assert_eq!(nb.cells.len(), 3);
        assert!(!nb.cells[2].ok);
    }

    #[test]
    fn magics_edit_and_rerun() {
        let mut nb = Notebook::default();
        nb.execute_cell("new point { mass = 2 }");
        nb.execute_cell("get obj0.mass");
        assert_eq!(nb.cells[1].output, "2");
        let out = nb.magic("%edit 2 get obj0.inverse_mass").unwrap();
        assert!(out.contains("Out[3]= 0.5"), "{out}");
        let out = nb.magic("%rerun 1").unwrap();
        assert!(out.contains("obj1"), "{out}");
        let hist = nb.magic("%history").unwrap();
        assert!(hist.contains("In[1]:="), "{hist}");
        assert!(nb.magic("%quit").is_none());
    }
}
