# 🏗️ Building `SolveIt_rust` Together

### A Complete, Beginner-Proof Guide to Git & GitHub for Our Class

**Repository:** `https://github.com/once-ere/SolveIt_rust.git`
NOTE: nsh (A.K.A., Teacher) has already created
https://github.com/once-ere/SolveIt_rust.git
**Audience:** High-school students with *zero* Git experience, and their teacher.
**What lives in the repo:** HTML, CSS, JavaScript, Rust, C, C++, Makefiles, CMake files, Markdown — plus datasets, Fortran code, and libraries.

> 🗝️ **The one-sentence summary:** One person — the owner of the GitHub account
> `once-ere` (your teacher) — creates the repository **once**; every student then
> **clones** it, works on a **personal branch**, and submits each day's homework as a
> **Pull Request** that the class reviews and the teacher merges.

---

## ⌨️ How to read every command box in this guide (read this first!)

1. **Type ONE line, press Enter, wait, and read what the computer says** before typing the next line.
2. Lines that begin with `#` are **explanations for humans — do not type them.**
3. Commands are always given **one at a time, one per line**. Where a shortcut exists that
   chains several commands together, it appears **after** the one-at-a-time version, in a
   clearly labeled block: **🚀 In one command (optional — only once the long way makes sense to you)**.
4. In those shortcut blocks, `&&` means *"and then, but only if the previous command succeeded."*
5. To **paste** into a terminal: Linux `Ctrl+Shift+V` · macOS `Cmd+V` · Windows PowerShell `Ctrl+V` or right-click · Git Bash right-click → Paste.

---

## Table of Contents

1. [The Cast of Characters — Who Does What](#1-the-cast-of-characters--who-does-what)
2. [The Big Question Answered: Who Creates the Repo, and From Where?](#2-the-big-question-answered)
3. [Fixing the Scrambled Sources (So Nobody Gets Confused)](#3-fixing-the-scrambled-sources)
4. [Git in Five Minutes: Words You Must Know](#4-git-in-five-minutes-words-you-must-know)
5. [Part I — One-Time Machine Setup (Everyone, Every Computer)](#5-part-i--one-time-machine-setup)
6. [Part II — Creating & Populating the Repository (Teacher Only, Once)](#6-part-ii--creating--populating-the-repository)
7. [Part III — Student First-Time Setup: Clone, Never Init](#7-part-iii--student-first-time-setup-clone-never-init)
8. [Part IV — The Daily Homework Loop](#8-part-iv--the-daily-homework-loop)
9. [Part V — The Teacher's Class-Day Routine](#9-part-v--the-teachers-class-day-routine)
10. [Keeping Your Branch Fresh & Fixing Conflicts](#10-keeping-your-branch-fresh--fixing-conflicts)
11. [Recovery Playbook: Common Mistakes and Exact Fixes](#11-recovery-playbook)
12. [Command Cheat Sheet](#12-command-cheat-sheet)
13. [The Golden Rules](#13-the-golden-rules)

---

## 1. The Cast of Characters — Who Does What

| Who | GitHub role | Responsibilities |
|---|---|---|
| **The teacher** (owns the GitHub account `once-ere`) | Repository **owner** | Creates the repo **once**, pushes the starter code, invites every student as a **collaborator**, reviews and merges Pull Requests in class |
| **Each student** | **Collaborator** (invited by the owner) | Clones the repo, pulls + builds daily, contributes one improvement per day on a personal branch, opens a Pull Request |

**"Should one designated person start this process, and the others somehow follow?" — YES, exactly.**
One person (whoever can log in to the `once-ere` GitHub account — the teacher) creates and
populates the repository **one time**. Everyone else "follows" by doing exactly two things:

1. **Create a personal GitHub account** (each student needs their own — free at <https://github.com/signup>) and **accept the collaborator invitation** GitHub emails them.
2. **Clone** the repository (Part III).

A student **never** creates the repo, **never** runs `git init`, and **never** pushes directly to `main`.

---

## 2. The Big Question Answered

> ❓ *Can `https://github.com/once-ere/SolveIt_rust.git` be created from the local
> school computer using the command line, or must `once-ere` log in at
> `https://github.com/once-ere` and create it on the website?*

**Answer: Both are possible — but only by someone authenticated as `once-ere`.**
A repository under `github.com/once-ere/...` lives *inside that GitHub account*, so nothing
a student's account does can ever create it. The two legitimate routes:

### Route A — Website (recommended for beginners)

1. Go to <https://github.com> and **sign in as `once-ere`**.
2. Click the **+** (top-right) → **New repository**.
3. Repository name: `SolveIt_rust`. Visibility: **Private** (recommended for a class) or Public.
4. ✅ Check **"Add a README file"** (this gives the repo its first commit and a `main` branch).
5. Optionally pick a `.gitignore` template (choose **Rust** — we extend it later).
6. Click **Create repository**. Done — `https://github.com/once-ere/SolveIt_rust.git` now exists.

### Route B — Local command line, using the official GitHub CLI (`gh`)

Plain `git` **cannot** create a repository on GitHub — `git` only talks to repos that already
exist. But GitHub's official command-line tool `gh` can, from any school computer, **after
logging in as `once-ere`**. One command at a time:

**Step 1 of 4 — install the GitHub CLI** (pick the lines for YOUR system):

```bash
# Ubuntu, line 1: refresh the list of installable software
sudo apt update

# Ubuntu, line 2: install the GitHub CLI program (called "gh")
sudo apt install -y gh
```

```bash
# macOS (needs Homebrew from https://brew.sh) — one line:
brew install gh
```

```powershell
# Windows 11 PowerShell — one line:
winget install --id GitHub.cli
```

**Step 2 of 4 — log in AS `once-ere`** (one time; it opens a browser window):

```bash
gh auth login
# When it asks, choose: GitHub.com  →  HTTPS  →  Login with a web browser
# Then follow the one-time code it shows you.
```

**Step 3 of 4 — create the repository** under the `once-ere` account:

```bash
gh repo create once-ere/SolveIt_rust --private --add-readme
```

**Step 4 of 4 — verify it now exists:**

```bash
gh repo view once-ere/SolveIt_rust
```

> ⚠️ Route B should be done **only by the teacher**, and on a shared school computer the
> teacher must afterward run this one command so students can't act as `once-ere`:
>
> ```bash
> gh auth logout
> ```

Either route ends in the same place: the repo exists, and the teacher continues with **Part II**
to populate it.

---

## 3. Fixing the Scrambled Sources

This guide reconciles two earlier, partially contradictory documents
(`github_local_dev_guide.md` and the forwarded Gmail thread *"git config and dev
Pre-Universe"*). Every contradiction, and the decision this guide adopts:

| # | The scrambled/conflicting information | ✅ The decision in this guide |
|---|---|---|
| 1 | One source says **clone** the repo ("codex"); the other says `git init` + `git remote add origin` + `git pull` ("gemini") | **Students always `clone`.** One command downloads history, creates the `origin` remote, and checks out `main` — nothing to get wrong. `init` + `remote add` is used **only** by the teacher, and only if the starter code already exists in a local folder (Part II, Option B). |
| 2 | The email sets `user.name "once-ere"` / `user.email "Patrick299Nash@gmail.com"` as if everyone should copy it | That identity belongs to **the repo owner only**. **Each student configures their *own* name and email** — commits must show who really wrote them. |
| 3 | Folder paths are mashed together: `/home/Users/uname/...` and `C;\Users\uname\...` | Correct, per OS: **Linux** `/home/uname/Developer/Projects/SolveIt_rust` · **macOS** `/Users/uname/Developer/Projects/SolveIt_rust` (both are `~/Developer/Projects/SolveIt_rust`) · **Windows 11** `C:\Users\uname\Developer\Projects\SolveIt_rust` (colon after `C`, not semicolon). |
| 4 | Sources mention a *Pre-Universe* project, URLs like `github.com/43d168f3e/...`, and Wolframscript `.wl` files | Those belong to an older project. This class uses **`once-ere/SolveIt_rust`** and a mix of **HTML/CSS/JS, Rust, C, C++, Make, CMake, Markdown, Fortran, datasets**. |
| 5 | The email shows an account password / token pasted in plain text | **Never do this.** GitHub does not accept account passwords for pushing; you use a **Personal Access Token (PAT)** — and a token must *never* be written into a file, an email, or the repository. Any token that has been seen is leaked: revoke it (§ 11 tells how). |
| 6 | Branch naming differs: `my-new-feature` vs `dev/<yourname>/<short-task>` | **`dev/<yourname>/<short-task>`** — with a whole class contributing, names must show *whose* branch it is at a glance. |
| 7 | Both "merge" and "squash-merge" appear in the merge procedure | The teacher uses **Squash and merge**: one clean commit per approved homework, so history reads like a class diary. |
| 8 | Earlier drafts led with combined commands like `gh repo create ... --source=. --remote=origin --push` | **Beginners get one command per line, each explained.** Combined shortcuts appear only afterward, clearly labeled optional (see the "How to read" box at the top). |

---

## 4. Git in Five Minutes: Words You Must Know

- **Git** — a program on *your* computer that records snapshots (commits) of a project.
- **GitHub** — a *website* that hosts a shared copy of the project so the whole class can collaborate.
- **Repository (repo)** — the project: all files **plus** their entire history.
- **Clone** — copy a GitHub repo to your computer (history included). Done **once** per machine.
- **Commit** — a saved snapshot with a message. Lives on *your* machine until you push.
- **Push** — upload your commits to GitHub. **Pull** — download the newest commits from GitHub.
- **Branch** — a parallel line of work. **`main`** is the official class branch; you work on your own branch so `main` stays stable.
- **`origin`** — the nickname Git saves for the GitHub URL you cloned from. Just a label in a config file, not a live connection.
- **Pull Request (PR)** — a *web page* on GitHub where you propose: "please merge my branch into `main`." The class reviews it; the teacher merges it.
- ⚠️ **`git pull` ≠ Pull Request.** `git pull` is a terminal command that downloads updates. A Pull Request is a review page on the website. Confusing the two breaks collaboration.

### The four places your code exists

```
 ┌──────────────┐  git add   ┌──────────────┐ git commit ┌──────────────┐  git push  ┌──────────────┐
 │  1. Working  │───────────▶│  2. Staging  │───────────▶│   3. Local   │───────────▶│  4. GitHub   │
 │   directory  │            │     area     │            │  repository  │            │   (origin)   │
 │ (your edits) │            │ (next commit)│            │(your commits)│◀───────────│ (everyone's) │
 └──────────────┘            └──────────────┘            └──────────────┘  git pull  └──────────────┘
```

Understand these four boxes and every Git command becomes predictable.

---

## 5. Part I — One-Time Machine Setup

> Everyone — teacher and students — does this **once on every computer they use**
> (school Linux account, school macOS account, personal laptop).

### 5.1 Install Git and the build tools

Our repo contains Rust, C, C++, and CMake projects, so each machine needs Git **and** compilers.
Remember: one line, Enter, read the output, next line.

#### 🐧 Linux (Ubuntu — school accounts)

```bash
# 1. Refresh Ubuntu's list of installable software:
sudo apt update

# 2. Install Git, the C/C++ compilers, make, cmake, and the Fortran compiler:
sudo apt install -y git build-essential cmake gfortran

# 3. Install the Rust toolchain (cargo). This single line downloads the official
#    installer and runs it — press Enter to accept the standard choices:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 4. Make cargo usable in THIS terminal window (new windows get it automatically):
source "$HOME/.cargo/env"
```

Now verify — each command should print a version number, not an error:

```bash
git --version
gcc --version
cmake --version
cargo --version
```

#### 🍎 macOS (school accounts and MacBooks)

```bash
# 1. Install Xcode Command Line Tools (gives you git + the C/C++ compiler).
#    A window pops up — click "Install" and wait for it to finish:
xcode-select --install

# 2. Install CMake (needs Homebrew from https://brew.sh):
brew install cmake

# 3. Install the Rust toolchain (cargo) — press Enter to accept the standard choices:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 4. Make cargo usable in THIS terminal window:
source "$HOME/.cargo/env"
```

Verify — each command should print a version number:

```bash
git --version
clang --version
cmake --version
cargo --version
```

#### 🪟 Windows 11 (personal laptops) — PowerShell

```powershell
# 1. Install Git:
winget install --id Git.Git -e --source winget

# 2. Install CMake:
winget install --id Kitware.CMake -e

# 3. Install the Rust toolchain:
winget install --id Rustlang.Rustup -e

# 4. Install the C/C++ compiler for Windows (Rust uses it too):
winget install --id Microsoft.VisualStudio.2022.BuildTools -e
```

Now **close PowerShell, open a new one**, and verify — one line at a time:

```powershell
git --version
cmake --version
cargo --version
```

> 💡 On Windows, prefer **Git Bash** (installed with Git) so the same Linux-style
> commands in this guide work unchanged.

### 5.2 Tell Git who *you* are (your OWN name — see Fix #2!)

```bash
# 1. Your real name, in quotes (appears on every commit you make):
git config --global user.name "Your Real Name"

# 2. The email address of YOUR GitHub account:
git config --global user.email "the-email-on-YOUR-github-account@example.com"

# 3. Make new repositories call their first branch "main":
git config --global init.defaultBranch main

# 4. Look at what you just saved — check for typos:
git config --global --list
```

*(Only the teacher, on the teacher's own machine, uses the `once-ere` identity.)*

### 5.3 Create your GitHub login credential (PAT)

GitHub refuses account passwords on the command line. When Git asks for a *password*
during your first `push` (or first `clone` of a private repo), you paste a
**Personal Access Token**:

1. Sign in to **your own** GitHub account → click your photo (top-right) → **Settings**.
2. Left sidebar, bottom: **Developer settings** → **Personal access tokens** → **Tokens (classic)**.
3. **Generate new token (classic)** → name it (e.g. `school-laptop`), set an expiration, check the **`repo`** scope → **Generate**.
4. **Copy the token now** (you will never see it again) and store it in a password manager.
5. First time Git prompts:
   - **Username** = your GitHub username (letters, like `once-ere`) — **never the token**
   - **Password** = **paste the token** — this is the *only* place a token ever goes

Your OS keychain (or Git Credential Manager on Windows) remembers it after the first use.

> 🚫 **Never** type a token into a file, a chat, an email, the username field, or the
> repository itself. A token that has been seen is a leaked token — revoke it immediately
> on the same settings page. (What happens if the token lands in the username field, and
> the exact cure, is Mistake ⑨ in § 11.)

### 5.4 Create the class folder (correct path per OS — see Fix #3!)

```bash
# Linux and macOS (Terminal):

# 1. Create the folder (and any missing parents — that is what -p does):
mkdir -p ~/Developer/Projects

# 2. Go into it:
cd ~/Developer/Projects
```

```powershell
# Windows 11 (PowerShell):

# 1. Create the folder:
New-Item -ItemType Directory -Path "$HOME\Developer\Projects" -Force

# 2. Go into it:
Set-Location "$HOME\Developer\Projects"
```

---

## 6. Part II — Creating & Populating the Repository

> 🧑‍🏫 **Teacher only. Done once for the whole class.** Students: skip to Part III.

### Option A — You created the repo on GitHub first (Route A/B above) ✅ recommended

**Step 1 — get the (nearly empty) repo onto your computer:**

```bash
# 1. Go to the class folder:
cd ~/Developer/Projects

# 2. Clone the repository (downloads it and connects it to GitHub):
git clone https://github.com/once-ere/SolveIt_rust.git

# 3. Go into the repository folder:
cd SolveIt_rust
```

**Step 2 — create one folder per sub-project, one at a time:**

```bash
mkdir web         # HTML, CSS, JavaScript
mkdir rust-app    # the cargo (Rust) project
mkdir c-app       # the C project with its Makefile
mkdir cpp-app     # the C++ project with its CMakeLists.txt
mkdir data        # datasets
mkdir docs        # documentation, Markdown files
```

> 🚀 **In one command** (optional): `mkdir -p web rust-app c-app cpp-app data docs`

Now copy your starter files into those folders with the file manager
(HTML/CSS/JS into `web/`, the cargo project into `rust-app/`, and so on).

**Step 3 — tell Git to ignore build junk.** Open a text editor
(Ubuntu: `gedit .gitignore` · macOS: `open -e .gitignore` · Windows: `notepad .gitignore`),
type exactly these lines, and save:

```text
# Rust
target/
# C / C++ / CMake
build/
*.o
*.obj
*.exe
*.out
# Editors / OS
.vscode/
.DS_Store
```

> 🚀 **In one command** (optional, Linux/macOS — writes the same lines for you):
>
> ```bash
> printf '# Rust\ntarget/\n# C / C++ / CMake\nbuild/\n*.o\n*.obj\n*.exe\n*.out\n# Editors / OS\n.vscode/\n.DS_Store\n' >> .gitignore
> ```

**Step 4 — snapshot everything and send it to GitHub, one command at a time:**

```bash
# 1. Stage every new file (the "." means: everything in this folder):
git add .

# 2. Look before you leap — the files listed in green are what will be saved:
git status

# 3. Save the snapshot locally, with a message:
git commit -m "Add starter project structure: web, rust-app, c-app, cpp-app, data, docs"

# 4. Upload the snapshot to GitHub:
git push origin main
```

### Option B — The starter code already exists in a local folder (the ONLY place `init` belongs)

```bash
# 1. Go to the existing folder of starter code:
cd ~/Developer/Projects/SolveIt_rust

# 2. Turn this plain folder into a Git repository whose branch is called "main":
git init -b main

# 3. Stage every file:
git add .

# 4. Check what is about to be saved:
git status

# 5. Save the first snapshot:
git commit -m "Initial commit: starter class project"

# 6. Tell Git where the GitHub repository lives, and call that address "origin":
git remote add origin https://github.com/once-ere/SolveIt_rust.git

# 7. Upload, and link your local "main" to GitHub's "main" (that is what -u does):
git push -u origin main
```

*(If the GitHub repo was created **with** a README, run
`git pull origin main --allow-unrelated-histories` between steps 6 and 7, resolve, then push.)*

> 🚀 **In one command** (optional, experts only — this is the shortcut some of you saw
> in an earlier draft; it creates the GitHub repo *and* uploads the current folder in
> one go, and only works after `gh auth login` as `once-ere` and steps 1–5 above):
>
> ```bash
> gh repo create SolveIt_rust --private --source=. --remote=origin --push
> ```

### Invite the students (required — otherwise they cannot push!)

1. On GitHub: **once-ere/SolveIt_rust** → **Settings** → **Collaborators** → **Add people**.
2. Add each student's GitHub username. Each student accepts the emailed invitation.

### Protect `main` (recommended)

**Settings** → **Branches** → **Add branch protection rule** → pattern `main` →
check **"Require a pull request before merging."** Now nobody — not even by accident —
can push straight to `main`; everything flows through class review.

---

## 7. Part III — Student First-Time Setup: Clone, Never Init

> 🧑‍🎓 Every student, **once per computer** (after accepting the collaborator invite).

```bash
# 1. Go to the class folder you created in § 5.4:
cd ~/Developer/Projects

# 2. Copy the whole repository (files + history) onto this computer:
git clone https://github.com/once-ere/SolveIt_rust.git

# 3. Go into your new local copy:
cd SolveIt_rust
```

That single `clone` did four things: downloaded **all files and all history**, created the
hidden `.git` database (your *local repository*), saved the remote nickname
`origin → https://github.com/once-ere/SolveIt_rust.git`, and checked out branch `main`.

**Prove it worked** — run these four, one at a time; each should answer without an error:

```bash
git rev-parse --is-inside-work-tree    # should print: true
git remote -v                          # should print the once-ere GitHub URL, twice
git branch -a                          # should print: * main, and remotes/origin/main
git status                             # should print: "nothing to commit, working tree clean"
```

> ❌ **Why not `git init` + `git remote add`, like one of the old emails said?**
> That two-step dance creates an *empty, unrelated* repo and then bolts the remote on.
> Beginners then hit "refusing to merge unrelated histories" errors and diverging
> branches. `clone` does everything in one correct step. (See Fix #1.)

---

## 8. Part IV — The Daily Homework Loop

> 🧑‍🎓 Every student, every day. Six steps: **Sync → Build → Branch → Improve → Commit → PR.**

### Step 1 — Sync: get the latest class code

```bash
# 1. Go to your local copy:
cd ~/Developer/Projects/SolveIt_rust

# 2. Make sure you are standing on the main branch:
git checkout main

# 3. Download and merge whatever the class merged since yesterday:
git pull origin main
```

### Step 2 — Build everything that is out of date

**Rust app** (three commands — go in, build, come back out):

```bash
cd rust-app
cargo build
cd ..
```

**Makefile project:**

```bash
cd c-app
make
cd ..
```

**CMake project** (two commands, both run from the repository's top folder):

```bash
# 1. Configure: read cpp-app/CMakeLists.txt and prepare a build/ folder:
cmake -S cpp-app -B cpp-app/build

# 2. Compile and link everything that is out of date:
cmake --build cpp-app/build
```

**Web code** (`web/` — HTML/CSS/JS) has no compile step: open `web/index.html` in a browser.

> 🚀 **In one command** (optional): `cd rust-app && cargo build && cd ..` — remember,
> `&&` means "and then, only if that worked."

If a build fails on freshly-pulled `main`, tell the class — that is a real bug, and
fixing it can *be* your contribution for the day.

### Step 3 — Create *your* branch for today's idea (never work on `main`)

```bash
# 1. Create a new branch named after you and today's task, and switch onto it:
git checkout -b dev/<yourname>/<short-task>
#    examples:  dev/maria/faster-sort      dev/jamal/fix-cmake-warnings

# 2. Verify — this must print YOUR branch name, not "main":
git branch --show-current
```

### Step 4 — Make your improvement

One new feature, optimization, refactor, bug fix, dataset, or documentation improvement.
Then inspect exactly what you changed:

```bash
# Which files did I touch?
git status

# What exactly changed inside them, line by line?
git diff
```

Re-run the Step 2 build for anything you touched — **never submit code you didn't build.**

### Step 5 — Commit (snapshot) your work

```bash
# 1. Stage exactly the files you meant to change (list them by name):
git add path/to/file1 path/to/file2

# 2. Check the list one last time:
git status

# 3. Save the snapshot with a clear message:
git commit -m "Optimize: replace bubble sort with quicksort in rust-app"
```

Message rule: **start with a verb, say what and why** — `Fix:`, `Add:`, `Optimize:`, `Refactor:`.

### Step 6 — Push your branch and open the Pull Request

```bash
# Upload your branch to GitHub; -u links it so next time plain "git push" is enough:
git push -u origin dev/<yourname>/<short-task>
```

*(First push ever? Git asks for username + **token** — § 5.3. Username is your GitHub
name; the token goes in the password field only.)*

Then in the browser:

1. Open <https://github.com/once-ere/SolveIt_rust> — a yellow banner shows your freshly-pushed branch → click **Compare & pull request** (or use **Pull requests** → **New pull request**).
2. **Base:** `main` ← **Compare:** your branch.
3. Title = your commit summary. Description = **what** you changed, **why** it is better, **how** you built/tested it.
4. Click **Create pull request**. 🎉 Homework submitted — your PR is now on the class list.

---

## 9. Part V — The Teacher's Class-Day Routine

> 🧑‍🏫 Next class: show every contribution, moderate discussion, merge the worthy ones.

### 9.1 Show and discuss each contribution

1. Open the **Pulls page**: <https://github.com/once-ere/SolveIt_rust/pulls> — one row per student submission.
2. For each PR, project it on screen and open the **Files changed** tab — green = added, red = removed. Perfect for discussion.
3. Moderate: *Is it correct? Is it clear? Is it worthy?* Classmates can leave comments — even line-by-line (hover a line, click the blue **+**).

### 9.2 Merge the approved contributions

For each PR the class approves:

1. Click **Squash and merge** (one clean commit per homework — Fix #7) → **Confirm**.
2. Click **Delete branch** (tidies the repo; the work is already in `main`'s history).

If a PR needs work: leave a **Request changes** review. The student fixes it, commits, and
pushes to the *same branch* — the PR updates automatically. No PR is ever wasted.

If two PRs collide (both edited the same lines), merge the first; GitHub marks the second
"has conflicts" — that student resolves it with § 10 and the class merges it next time.

### 9.3 Everyone re-syncs (including the teacher)

```bash
# 1. Stand on main:
git checkout main

# 2. Download today's newly merged homework:
git pull origin main

# 3. Delete your merged branch — its work now lives in main:
git branch -d dev/<yourname>/<short-task>
```

The loop is closed: today's merged `main` is the starting line for tomorrow's homework.

---

## 10. Keeping Your Branch Fresh & Fixing Conflicts

If `main` moved while you were working (someone's PR got merged), update your branch
**before** pushing:

```bash
# 1. Stand on main:
git checkout main

# 2. Get the newest main:
git pull origin main

# 3. Go back to your own branch:
git checkout dev/<yourname>/<short-task>

# 4. Weave main's new commits into your branch:
git merge main
```

If Git reports a **conflict**, it wrote both versions into the file:

```text
<<<<<<< HEAD
your version of the line(s)
=======
the version that came from main
>>>>>>> main
```

Fix it in three moves:

1. Open the file; replace the whole block (including the `<<<<<<<`, `=======`, `>>>>>>>` markers) with the correct final code — yours, theirs, or a blend.
2. Rebuild (Step 2 of the daily loop) to prove the blend works.
3. Conclude, one command at a time:
   ```bash
   git add .
   git commit -m "Merge main and resolve conflicts"
   git push
   ```

---

## 11. Recovery Playbook

| 😱 Mistake | ✅ Exact fix (one command per line) |
|---|---|
| ① "I edited files while on `main`" (not committed) | `git checkout -b dev/<you>/<task>` — your edits ride along to the new branch. |
| ② "I *committed* on `main`" (not pushed) | First `git checkout -b dev/<you>/<task>` (saves your work on a branch), then `git checkout main`, then `git reset --hard origin/main` (makes `main` match GitHub again). |
| ③ "I staged a file I didn't mean to" | `git restore --staged path/to/wrong.file` |
| ④ "I want to throw away my edits to one file" | `git restore path/to/file` ⚠️ permanently discards those edits. |
| ⑤ "My commit message is wrong" (not pushed) | `git commit --amend -m "Better message"` |
| ⑥ "Git rejected my push" (branch is behind) | First `git pull`, resolve any conflict (§ 10), then `git push`. |
| ⑦ "I cloned/configured the wrong URL" | `git remote set-url origin https://github.com/once-ere/SolveIt_rust.git` |
| ⑧ "Everything is hopelessly tangled" | Rename the broken folder, re-`clone` fresh (Part III), copy your changed files in, then branch-commit-push normally. |
| ⑨ **"The password popup shows my TOKEN as the username"** — e.g. `Password for 'https://d3E432q89…@github.com'` | The token got into the URL's username slot (pasted into the URL, or typed at the *Username* prompt and saved). Three commands/actions, in order: **(a)** repair the URL: `git remote set-url origin https://YOUR_GITHUB_USERNAME@github.com/once-ere/SolveIt_rust.git`, then verify: `git remote -v`; **(b)** erase the bad saved credential: `printf "protocol=https\nhost=github.com\n" \| git credential reject` — if the popup persists, delete the `github.com` entry in your keyring (Ubuntu: *Passwords and Keys*; macOS: *Keychain Access*; Windows: *Credential Manager*); **(c)** the token has been displayed on screen, so it is leaked — **revoke it** (GitHub → Settings → Developer settings → Tokens) and generate a new one. Then push again: username = your GitHub name, password = the **new** token. |

---

## 12. Command Cheat Sheet

| Command | Meaning |
|---|---|
| `git clone <url>` | Copy a GitHub repo (files + history + remote) to your machine — once per machine |
| `git status` | What changed? What is staged? What branch am I on? |
| `git diff` | Show exact line-by-line edits not yet staged |
| `git add <file>` | Put a file's changes into the staging area |
| `git commit -m "msg"` | Save a snapshot of staged changes locally |
| `git push` / `git push -u origin <branch>` | Upload commits (`-u` links the branch the first time) |
| `git pull origin main` | Download + merge the newest `main` from GitHub |
| `git checkout main` | Switch to branch `main` |
| `git checkout -b <branch>` | Create a new branch and switch to it |
| `git branch --show-current` | Print the branch you are on |
| `git branch -d <branch>` | Delete a (merged) local branch |
| `git merge main` | Bring `main`'s new commits into your current branch |
| `git log --oneline --graph -15` | Compact picture of recent history |
| `git remote -v` | Show the saved `origin` URL |
| `git remote set-url origin <url>` | Repair/replace the saved `origin` URL |
| `git restore <file>` / `git restore --staged <file>` | Discard edits / unstage a file |
| `cargo build` | Build the Rust project in the current folder |
| `make` | Build a Makefile project |
| `cmake -S . -B build` | CMake step 1 of 2: configure (prepare the build folder) |
| `cmake --build build` | CMake step 2 of 2: compile and link |
| `gh repo create once-ere/SolveIt_rust --private --add-readme` | (Owner only) create the GitHub repo from the terminal |

---

## 13. The Golden Rules

1. 🌿 **Never work on `main`.** Branch first: `dev/<yourname>/<short-task>`.
2. 🔁 **Always `git pull origin main` before starting** — begin from today's truth.
3. 🔨 **Build before you push.** Broken code stops the whole class.
4. 📦 **One idea = one branch = one Pull Request.** Small PRs get reviewed; huge ones get skipped.
5. ✍️ **Commit messages start with a verb** and explain *why*, not just *what*.
6. 🔐 **Your identity is yours**: your own `user.name`/`user.email`, your own GitHub account, your own token. **Tokens go only in the password prompt — never in files, emails, URLs, or the username field.**
7. ⌨️ **One command per line.** Type it, press Enter, read the answer, then continue. Shortcut chains with `&&` are for later, once the long way is boring.
8. 🧯 **Don't panic-delete.** Every mistake in § 11 has a short, exact fix. Ask first; `rm -rf` never.
9. 🤝 **Review kindly, specifically, in class.** Comment on code, not on people.

---

*Generated 2026-07-19 by reconciling `github_local_dev_guide.md` and
"Gmail — Fwd: git config and dev Pre-Universe" into a single, consistent workflow.
Companion files: `New_Repo.pdf` (compiled from `New_Repo.tex`) and `New_Repo.xlsx`.*
