# 0.3 — Prompt System

Segment-based prompts with themes. Every issue in this file belongs to milestone `0.3 — Prompt System`.

---

### Segment renderer core

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Walk the segment list, run each segment, assemble the rendered prompt. Support left, right, and multi-line layouts. Output: a string with ANSI escapes ready for the terminal.

---

### Named styles and theme palette

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

`[styles.NAME] fg = "#..." bg = "#..." bold = true italic = true`. Segments reference styles by name. Palette swap recolors the entire prompt in one place.

---

### `when` condition DSL

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Parse and evaluate condition expressions in segment configs. Supports negation, AND/OR composition, parameterized conditions (`cwd_matches:~/work/*`, `env_set:FOO`).

---

### Built-in conditions library

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Ship: `in_git_repo`, `has_venv`, `is_ssh`, `last_command_failed`, `last_command_took_ms:N`, `cwd_matches:GLOB`, `env_set:VAR`, `has_file:NAME`. Plugins can register more.

---

### Built-in segments: basics

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

`cwd`, `time`, `exit_code`, `duration`, `user`, `hostname`, `os_icon`. Each with its standard config knobs.

---

### Built-in segments: git

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

`git` segment with branch, dirty status, ahead/behind, stash count, untracked files. Optional sub-controls via config flags.

---

### Built-in segments: language environments

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

`python_venv`, `node_version`, `ruby_version`, `rust_toolchain`, `go_version`, `java_version`. Auto-detect from cwd; configurable visibility (always / only-in-project / never).

---

### Built-in segments: cloud and remote

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

`kubectl_context`, `kubectl_namespace`, `aws_profile`, `gcloud_project`, `terraform_workspace`, `ssh_host`.

---

### Separator strategies

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Pluggable joiners: `powerline`, `plain`, `brackets`, custom. Decoupled from segments — adding a visual style is one new strategy, not edits to every segment.

---

### Async segment runtime

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Slow segments (git status over a slow filesystem) run on a background task with timeout. The prompt shows a placeholder and updates in place when the result arrives. This is the engine that makes the shell feel fast.

---

### Transient prompts

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

After a command runs, the previous prompt re-renders with a stripped-down theme (e.g. just `❯`) to save screen space. The transient theme is its own segment list, configurable like any other theme.

---

### Nerd font and ASCII fallback

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Every built-in segment declares a glyph and an ASCII fallback. A config switch toggles globally. Makes SSH to plain-font boxes work without changes.

---

### Right prompt and multi-line layouts

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Right-aligned segments. Two-line prompts (info above, input below). Graceful narrow-terminal degradation — right segments drop to a second line rather than clip.

---

### `bento theme preview`

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

Renders the current prompt in a grid of states: clean / dirty git / venv active / SSH / after failure / deep path. Lets users iterate on themes without setting up real state.

---

### Theme install and share format

**Labels:** `area:prompt`, `kind:feat`
**Milestone:** `0.3 — Prompt System`

A theme is a single shareable file (palette + segment list + layout). `bento theme install <user/repo>`, `bento theme use <name>`, `bento theme list`, `bento theme show <name>`.

---
