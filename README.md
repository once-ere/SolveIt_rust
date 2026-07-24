MIT License

Copyright (c) 2026 once-ere

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.


# physical_object_simulator

Pure-Rust physics simulator built on the local
[`sundials_rs`](sundials_rs) workspace — **all** numerical
integration runs through the pure-Rust SUNDIALS 7.7.0 translation
(CVODE Adams/BDF, ARKODE symplectic SPRK). Zero `unsafe`, zero external
crate dependencies, zero warnings.

Latest release: a programmable notebook and a compound rigid body —
user-defined functions (`DEF name(param = default, ...) { body }`,
every body line syntax-checked at definition, `FUNCS`/`SHOW` to list
and edit), named objects (`NEW ... AS name`, plus `LET` variables and
string literals), and the rigid `DUMBBELL` (two solid spheres plus a
rod as ONE rigid body, exact part-wise collisions conserving E, P and
L through real solver events); the scene window gains a permanent
Reset button (with `SCENE RESET` — bit-identical re-initialization,
Start re-runs) and a live labeled conserved-quantities readout (E, P
and L); 99 tests green (39 lib + 16 collision + 9 conservation +
35 posim).

- `physical_object/` — library: `pub struct physical_object`, the
  unique union of the legacy `PointParticle`, `RigidBody` and
  `RigidBody3D`, with get/set for every field; `PhysicalObjectSystem`;
  the sundials integration drivers; validated examples.
- `posim/` — the simulator front end: lexer → grammar compiler → stack
  machine, a notebook REPL (`In[n]`/`Out[n]` cells), script batch mode,
  and a JSON machine mode.
- `jupyter/` — JupyterLab wrapper kernel so notebooks can get/set the
  simulator's data (see `jupyter/README.md`).
- `PLAN.md` — the integration plan / design record (union mapping,
  grammar, solver mapping, verification results).

## Documentation

- [grammar.md](physical_object_simulator/grammar.md) /
  [grammar.pdf](physical_object_simulator/grammar.pdf) — the complete
  command-language and notebook specification, with fourteen worked
  examples.
- [physical_object_simulator.md](physical_object_simulator/physical_object_simulator.md) /
  [physical_object_simulator.pdf](physical_object_simulator/physical_object_simulator.pdf) — the
  full solution guide for new users, with fourteen more worked examples.
- [ARCHITECTURE.md](physical_object_simulator/ARCHITECTURE.md) — module
  responsibilities and pinned cross-module contracts.
- [CLAUDE.md](physical_object_simulator/CLAUDE.md) — working rules for
  contributors and agents.

## Quick start

The simulator lives in the
[`physical_object_simulator/`](physical_object_simulator/) workspace;
the repository root holds the legacy `realtime_orbit` reference app,
the C reference `sundials-7.7.0/`, and the `sundials_rs` submodule.

```bash
cd physical_object_simulator
cargo run                 # notebook REPL (type HELP)
cargo test --workspace    # all tests
cargo run -p physical_object --release --example kepler_orbit
cargo run -p physical_object --release --example outer_solar_system
cargo run -p physical_object --release --example tumbling_body
cargo run -p physical_object --release --example charged_in_b_field
cargo run -p posim -- --script my_session.posim
cargo run -p posim -- --machine   # JSON protocol for front ends
```

Example session:

```
In[1]:= new sphere { mass = 2, radius = 0.5, position = [0, 10, 0], velocity = [1, 0, 0] }
Out[1]= obj0
In[2]:= set system.gravity = [0, -9.81, 0]
In[3]:= step 1
Out[3]= t = 1 (advanced by 1, 12 solver steps)
In[4]:= get obj0.position
Out[4]= [1, 5.095000000000006, 0]
In[5]:= method sprk leapfrog_2_2 0.001
In[6]:= help
```
