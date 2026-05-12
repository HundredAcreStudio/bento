# 0.1 — Walking Shell

Core REPL with pipes, redirects, and built-ins. Every issue in this file belongs to milestone `0.1 — Walking Shell`.

---

### Decide implementation language

**Labels:** `kind:design`
**Milestone:** `0.1 — Walking Shell`

Pick the host language (Rust / Go / Zig / OCaml / …). Write a one-page ADR with tradeoffs:

- ecosystem maturity and platform support
- embedded scripting bindings (Lua via mlua, Wasm via wasmtime)
- terminal-UI library quality (ratatui, bubbletea, …)
- async runtime story
- contributor on-ramp

**Blocks:** project scaffolding.

---

### Project scaffolding and CI

**Labels:** `kind:infra`
**Milestone:** `0.1 — Walking Shell`

Build system, source layout, test runner, linter/formatter, GitHub Actions CI running build + tests + lint on every PR.

**Done when:**
- the build command produces a working binary
- a smoke test passes in CI
- lint and format checks gate PRs

---

### REPL loop and prompt input

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Read a line of input, echo it back, loop until EOF. Handle Ctrl-D cleanly. No parsing yet — this is the spine that everything else hangs off.

---

### Tokenizer

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Tokenize a command line into words, respecting:

- single and double quotes
- backslash escapes
- metacharacters: `|`, `>`, `<`, `>>`, `&`, `;`, `&&`, `||`, `(`, `)`

Output: a stream of typed tokens for the parser to consume.

---

### Parser and command AST

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Parse the token stream into commands, pipelines, redirections, and short-circuit lists (`&&`, `||`). Define the AST types other subsystems will consume.

**Done when:** `echo hi | grep h && echo ok` parses to the right tree.

---

### External command execution

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Implement fork + exec with PATH lookup. `ls` and other simple commands run end-to-end and return the correct exit code.

**Notes:** use `execvp` so PATH lookup is free.

---

### Built-in framework and core built-ins

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Registry pattern for built-ins. Implement: `cd`, `exit`, `export`, `unset`, `alias`, `unalias`, `pwd`, `:`, `true`, `false`.

Built-ins run in-process; the dispatch step checks the registry before going down the fork/exec path.

---

### Variable storage and expansion

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Shell variables, environment variables, and expansion rules:

- `$VAR` and `${VAR}`
- `${VAR:-default}`, `${VAR:=default}`, `${VAR:?msg}`, `${VAR:+alt}`
- `$?`, `$$`, `$!`, `$#`, `$@`, `$*`

---

### Tilde, brace, command sub, arithmetic, and glob expansion

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

- Tilde: `~`, `~user`
- Brace: `{a,b,c}`, `{1..10}`
- Command substitution: `$(cmd)` and legacy backticks
- Arithmetic: `$((expr))`
- Globs: `*`, `?`, `[abc]`, `**` (recursive)

Expansion order matters — follow POSIX rules, then document our extensions.

---

### I/O redirection

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

`>`, `<`, `>>`, `2>`, `&>`, `2>&1`, `<<<` (here-string), `<<` (heredoc).

Implementation: in the child between fork and exec, `open()` the target file and `dup2()` onto the appropriate fd. The exec'd program inherits the redirected fd.

---

### Pipes

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

Single (`a | b`) and multi-stage (`a | b | c | …`). Each stage forks; pipes wired with `pipe()` + `dup2()`. Exit status is the last stage's by default; support `pipefail` option.

---

### Signal handling and basic job control

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

- SIGINT (Ctrl-C) kills the foreground job, not the shell
- SIGTSTP (Ctrl-Z) suspends the foreground job
- `&` runs in the background
- `jobs`, `fg`, `bg` built-ins
- Process groups and `tcsetpgrp` for terminal control
- SIGCHLD handler updates job table

---

### Persistent history and line editor

**Labels:** `area:core`, `kind:feat`
**Milestone:** `0.1 — Walking Shell`

- Up/down arrow recall
- Ctrl-R reverse incremental search
- Configurable key bindings (start with emacs defaults)
- History persisted to `~/.local/share/bento/history`
- Cursor movement: Ctrl-A, Ctrl-E, Alt-B/F (word-wise)
- Multi-line editing for unterminated input

---
