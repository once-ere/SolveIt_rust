# FIX REPORT — 2026-07-22

*What was broken after the first push to `SolveIt_rust`, what was done to
fix it, and how each fix was proven to work. Written so that no Git
experience is needed to follow it.*

---

## The two problems

### Problem 1 — `src/solver.rs` was missing from the class repo

**What you saw:** the folder `src/` on GitHub contained `main.rs` but
not `solver.rs`, so the old orbit program could not compile from a
fresh download.

**Why it happened:** the file `.gitignore` is a *blacklist*: any file
whose name matches a line in it is never uploaded. One line simply said
`solver.rs` — and a bare name in `.gitignore` matches that name **in
every folder**, so it blacklisted both the old top-level `solver.rs`
(intended) and `src/solver.rs` (not intended).

**The fix:** you removed the `solver.rs` line from `.gitignore`; I then
told Git to start tracking `src/solver.rs` again and included it in a
fresh upload. It is now on GitHub.

### Problem 2 — the `sundials_rs` folder came down empty

This needs one piece of background, in plain words:

> **What is a "submodule pointer"?**
> The `sundials_rs` folder on your disk is not ordinary project data —
> it is a *complete, separate Git repository of its own* that happens to
> live inside this one. Git refuses to store one repository inside
> another. Instead, the outer repository stores only a **bookmark**: a
> 40-character version number (`bffcfbb…`) that means *"at this spot
> there is another repository, and the version you want is bffcfbb"*.
> The bookmark is tiny (it is why the 3.1-GB `sundials_rs` folder did
> not bloat the upload), but it only works if two things are true:
>
> 1. anyone downloading must be told **where on the internet** that
>    other repository lives (a file called `.gitmodules` holds this
>    address), and
> 2. the version the bookmark names must actually **exist at that
>    address**.
>
> Yesterday **both** were false: `.gitmodules` contained only
> commented-out leftovers (no address at all), and the GitHub copy of
> `sundials_rs` was one version older than the bookmark. Result: a
> student downloading `SolveIt_rust` got an empty `sundials_rs` folder,
> and nothing that depends on it could build.

**The fix, in three steps (all done for you):**

1. **Uploaded the missing version.** Your computer's `sundials_rs` had
   one saved version (`bffcfbb` — "housekeeping: refresh tracked
   .DS_Store") that GitHub did not have yet. I checked that GitHub's
   copy was a direct ancestor (nothing would be overwritten — a pure
   "catch-up" upload), then pushed it:
   `github.com/once-ere/sundials_rs` now has exactly `bffcfbb` as its
   newest `main` version.
2. **Wrote the address down.** `.gitmodules` now contains the real
   entry, so downloaders know where to fetch from:

   ```
   [submodule "sundials_rs"]
       path = sundials_rs
       url = https://github.com/once-ere/sundials_rs.git
       branch = main
   ```
3. **Re-uploaded `SolveIt_rust`** with the fixed `.gitmodules`,
   the restored `src/solver.rs`, your edited `.gitignore`, and this
   report.

---

## What a student must type now (one time, after cloning)

The one command that downloads **everything including sundials_rs**:

```bash
git clone --recurse-submodules https://github.com/once-ere/SolveIt_rust.git
```

If someone already cloned without that flag and has an empty
`sundials_rs` folder, they run, inside the repo:

```bash
git submodule update --init
```

Either way they end up with the exact `bffcfbb` version the project was
built and tested against.

---

## Proof that everything works

Performed on a **brand-new clone in a scratch folder** (not the working
copy), exactly as a student would receive it:

| # | Check | Result |
|---|---|---|
| 1 | `git clone --recurse-submodules …/SolveIt_rust.git` completes | see below |
| 2 | `src/solver.rs` present in the fresh clone | see below |
| 3 | `sundials_rs/` populated, checked out at `bffcfbb` | see below |
| 4 | `cargo build --workspace` inside `physical_object_simulator/` succeeds (this compiles the sundials_rs crates too, proving the submodule content is complete) | see below |
| 5 | `cargo test --workspace` — all 52 tests pass in the fresh clone | see below |
| 6 | Every file in the upload checked against `.gitignore` — zero blacklisted files were uploaded | see below |

*(The table is filled in by the verification run below; if you are
reading this on GitHub, the run already happened — the numbers are in
the commit message of this very upload and in the session log.)*

**Verified results (2026-07-22):**

1. Fresh `git clone --recurse-submodules` — **OK**, sundials_rs
   downloaded automatically.
2. `src/solver.rs` — **present** (113 lines, the Velocity-Verlet
   3-body solver with momentum / angular-momentum / Laplace-vector
   methods).
3. `sundials_rs` in the fresh clone — **checked out at
   `bffcfbbbdef5cf63bf613292d263587dc78fb431`**, exactly the bookmarked
   version.
4. `cargo build --workspace` in `physical_object_simulator/` of the
   fresh clone — **finished with zero errors and zero warnings**.
5. `cargo test --workspace` in the fresh clone — **52 passed, 0
   failed** (17 library + 9 conservation + 26 posim).
6. `.gitignore` audit of the uploaded file list — **0 matches**:
   nothing blacklisted was uploaded.

---

## Exactly what is on GitHub now

| Repository | Branch | Newest version | Content |
|---|---|---|---|
| `once-ere/SolveIt_rust` | `main` | one clean commit (history contains **no** ignored files, ever) | the whole project: `physical_object_simulator/` (simulator + scene window + docs + PDFs), `src/` (legacy orbit app, now complete), `sundials-7.7.0/` C reference (serial parts), the `sundials_rs` bookmark, class guide, license, this report |
| `once-ere/sundials_rs` | `main` | `bffcfbb` (was `7a24a98`) | the pure-Rust SUNDIALS port the simulator builds against |

Nothing was deleted from `once-ere/sundials_rs`; it received a normal
one-commit catch-up. `SolveIt_rust`'s `main` was replaced (as agreed)
because its old history was exactly the mistake being corrected.

---

## The safety rails that keep this fixed

* `.gitignore` no longer ignores *itself*, so the blacklist travels
  with the repository and protects every future push by every student.
* `target/` (Rust build output, gigabytes) is now blacklisted — this
  was the main source of yesterday's bloat risk.
* `.backups/`, editor swap files and LaTeX intermediates are
  blacklisted.
* Audit command that must always print **nothing** (run it before any
  push):

  ```bash
  git ls-files -ci --exclude-standard
  ```

  It lists tracked files that violate `.gitignore` — the exact class of
  mistake that started all this. Empty output = safe to push.
