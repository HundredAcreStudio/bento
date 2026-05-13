# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

Bento is a POSIX-ish shell written in Rust (2024 edition, MSRV pinned in `Cargo.toml`). The roadmap is the four milestone files under `docs/plan/`; design rationale lives in `docs/adr/`. Read those before proposing architectural changes — the language choice (Rust over Go/Zig/OCaml) is decided in `docs/adr/0001-implementation-language.md` and is load-bearing for the syscall, line-editor, and embedded-scripting strategy.

The current `README.md` is a transient artifact about *generating GitHub issues* from the plan files — it is not project documentation. Treat `CONTRIBUTING.md`, `docs/adr/`, and `docs/plan/` as the real sources of truth.

## Common commands

The three commands CI gates on, in order:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI additionally runs `cargo build --release --locked` to catch `Cargo.lock` drift. Run a single test with `cargo test <name>` (e.g. `cargo test repl_echoes_until_eof`). The toolchain is pinned via `rust-toolchain.toml`; `rustup` fetches the right one automatically.

## Crate layout

```
src/main.rs     # thin entry point — calls bento::run()
src/lib.rs      # library root; everything testable lives here
tests/smoke.rs  # binary integration tests via assert_cmd
```

Keep the binary thin and put new logic behind `lib.rs` so it stays testable.

## Lint and safety policy

`Cargo.toml` is the authority. Three things matter:

- `unsafe_code = "deny"` at the crate root. Per ADR 0001, all `unsafe` will live in a forthcoming `src/sys/` module behind a typed API that locally opts back in with `#[allow(unsafe_code)]`. Don't introduce `unsafe` elsewhere.
- `clippy::pedantic` is warned (group-level, priority -1). Expect to satisfy pedantic lints in new code.
- `unwrap_used`, `expect_used`, and `panic` are warned. In production code, bubble errors via `io::Result` / `?` and have the top-level function print them. The smoke test file opts out with `#![allow(clippy::expect_used, clippy::unwrap_used)]` — that allowance is for tests only.

## Milestone context

Work currently lives in milestone **0.1 — Walking Shell** (`docs/plan/01-walking-shell.md`). Issues in that file are deliberately small and sequential (REPL → tokenizer → parser → exec → built-ins → expansions → redirection → pipes → signals → line editor). When picking up an issue, check the plan file for its position in that chain — later items often have non-obvious dependencies on the framework an earlier item is meant to establish, and scope creep across issue boundaries is the wrong call.
