# ADR 0001 — Implementation language: Rust

- **Status:** Accepted
- **Date:** 2026-05-12
- **Issue:** [#1](https://github.com/HundredAcreStudio/bento/issues/1)
- **Milestone:** 0.1 — Walking Shell

## Context

Bento is a new POSIX-ish shell. Milestone 0.1 (the "Walking Shell") needs fork/exec with PATH lookup, pipe wiring with `pipe()`+`dup2()`, redirection, signal handling, and job control with `tcsetpgrp` and process groups. Later milestones add an embedded scripting runtime for plugins (Lua and/or Wasm), a prompt/segment renderer, and a completion menu with an `fzf`-style picker.

The host language has to be good at four things at once:

1. **Low-level POSIX syscalls** — running arbitrary code in the child between `fork` and `exec`, owning signal disposition, manipulating process groups and the controlling terminal.
2. **Line editing and TUI** — a real readline replacement (history, reverse-search, multi-line, vi/emacs modes) plus an immediate-mode TUI for completion menus.
3. **Embedded scripting** — mature, idiomatic bindings for Lua and Wasm.
4. **Async without fighting the kernel** — slow hooks must not block the prompt; the runtime must coexist with raw syscalls.

Candidates: Rust, Go, Zig, OCaml.

## Decision

**Rust**, stable channel via `rustup`, 2024 edition.

## Rationale (by axis)

### Syscalls — the deciding factor

Writing a Unix shell needs arbitrary code between `fork()` and `exec()`: wire `pipe()` fds with `dup2()`, `setpgid()`, install signal dispositions, close inherited fds. Rust exposes this directly through `std::os::unix::process::CommandExt::pre_exec`, with the full POSIX surface available via the `nix` crate.

Go's `syscall.SysProcAttr` covers the common shapes (`Setpgid`, `Foreground`, `Ctty`, `ExtraFiles`) but **forbids arbitrary code in the child** because Go's M:N scheduler makes `fork()` in a multi-threaded process unsafe in general — the runtime works around this with a stop-the-world `ForkExec`, and you cannot run user code in that window. Go's runtime also owns signal handlers; `signal.Notify` is a notification channel, not `sigaction`. These are documented friction points for shell authors and exactly what a job-control shell needs to do.

Zig has clean syscall access but the standard library is unstable and `async` is currently removed. OCaml's syscall coverage is fine, but its FFI dominates any non-trivial systems work.

### Line editing and TUI

- **Rust:** `reedline` is the line editor Nushell extracted into a reusable crate — history, multi-line, reverse search, vi/emacs modes, completion menus, hints. It is *purpose-built for shells*. `crossterm` covers raw-mode/terminal control; `ratatui` is a mature immediate-mode TUI for any extra surface (completion popups, picker UI).
- **Go:** `bubbletea` is excellent for TUI apps but is not a line editor. There are several Go readline-likes (`chzyer/readline`, `peterh/liner`) but none are at reedline's level. We would end up writing the line editor ourselves.

Reedline alone shifts months of work off the critical path.

### Embedded scripting

- **Rust:** `mlua` is the canonical Lua binding — async-aware, sandboxable, well-maintained, supports Lua 5.4/LuaJIT. `wasmtime` is first-party Rust.
- **Go:** `gopher-lua` is pure Go but slower and less idiomatic; `wazero` is a strong pure-Go Wasm runtime but not better than Wasmtime. CGO to native Lua is possible but throws away Go's deployment story.

mlua's quality and the wasmtime native fit make this a meaningful Rust win.

### Async runtime

Tokio is opt-in. The hot REPL path can stay synchronous; async is reserved for the hooks subsystem (0.2) where slow plugins must not block the prompt. This is the inverse of Go, where every binary ships with a runtime whether you want it or not — fine for servers, awkward for a shell that wants tight syscall control.

### Contributor on-ramp — the real cost

Rust's learning curve is the genuine downside vs. Go. Mitigations:

- Confine `unsafe` to a small `sys::` module behind a typed API.
- Lean on `clippy` and `rustfmt` in CI from day one.
- Write `CONTRIBUTING.md` early; document the crate layout and where to start.
- Prefer ergonomic crates (`anyhow`/`thiserror`, `clap`, `nix`) over hand-rolled equivalents.

### Ecosystem maturity / platform support

Rust on macOS and Linux is first-class. Apple Silicon has stable native targets. Windows is a Tier-1 target if we ever want it (current focus is Unix). Crate ecosystem covers everything Bento needs.

### Precedent

Nushell, Atuin, Starship, Zellij, Helix, Alacritty, Wezterm — modern terminal tooling has converged on Rust. The library ecosystem (`reedline`, `crossterm`, `ratatui`, `mlua`, `nix`) was largely built by and for that cohort.

## Alternatives considered

- **Go** — rejected. Syscall friction is real (no code between fork/exec, runtime owns signals), and the shell-specific library story (line editor, Lua) is weaker. Elvish proves Go can ship a shell, but we would fight the runtime for job control and write our own reedline.
- **Zig** — rejected. Excellent syscall ergonomics, but pre-1.0, std library churn, async story in flux, small TUI/Lua ecosystem.
- **OCaml** — rejected. Capable language, small contributor pool, weak TUI and embedded-scripting story.
- **C / C++** — not seriously considered. We want memory safety by default.

## Consequences

**Positive**
- Direct POSIX access for the parts that need it; no fighting a managed runtime.
- Reedline, mlua, wasmtime, ratatui shave months off the roadmap.
- Strong static guarantees and tooling (`clippy`, `rustfmt`, `rust-analyzer`).

**Negative**
- Steeper contributor on-ramp. Mitigated by CI quality bars and `CONTRIBUTING.md`.
- Compile times will be longer than Go. Acceptable for a shell binary that ships infrequently; incremental builds are fast enough in dev.
- The owner is new to Rust. Acceptable; the language is well-documented and the relevant crates are well-trodden.

## Follow-ups

- Issue #2 — project scaffolding and CI in Rust.
- Milestone 0.2 — separate ADR for the plugin scripting language (mlua vs. wasmtime vs. hybrid).
