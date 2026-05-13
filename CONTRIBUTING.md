# Contributing to Bento

Thanks for your interest. Bento is in early walking-shell territory — the layout
and tooling are deliberately minimal but the bar for what lands is high.

## Toolchain

The toolchain is pinned via [`rust-toolchain.toml`](./rust-toolchain.toml). If
you have `rustup` installed, the right toolchain (stable + `rustfmt` + `clippy`)
is fetched automatically the first time you run a `cargo` command in this repo.
No manual `rustup install` needed.

Minimum supported Rust version: see `rust-version` in [`Cargo.toml`](./Cargo.toml).

## Local checks (what CI runs)

Three commands, in order:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

CI runs the same commands plus a `cargo build --release --locked` to catch
`Cargo.lock` drift. Run them locally before pushing.

## Crate layout

```
src/
  main.rs     # thin binary entry point
  lib.rs      # library root — everything testable lives behind here
tests/
  smoke.rs    # binary smoke tests
docs/
  adr/        # architecture decision records — read these for the "why"
  plan/       # milestone plans
```

### `unsafe` policy

Per [ADR 0001](./docs/adr/0001-implementation-language.md), all `unsafe` code
must live inside a forthcoming `src/sys/` module behind a typed API. The crate
root declares `unsafe_code = "deny"` in `Cargo.toml`; `sys::` will opt back in
locally with `#[allow(unsafe_code)]`. Keep the dangerous parts small and
auditable.

## Design context

Before proposing larger changes, skim:

- [`docs/adr/`](./docs/adr/) — language choice and other accepted decisions
- [`docs/plan/`](./docs/plan/) — the milestone roadmap (issues are derived from
  these files)
