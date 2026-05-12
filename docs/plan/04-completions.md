# 0.4 — Completions

Schema-driven completions with menu UI. Every issue in this file belongs to milestone `0.4 — Completions`.

---

### Schema format design and validator

**Labels:** `area:completions`, `kind:design`
**Milestone:** `0.4 — Completions`

TOML schema for command specs: subcommands, flags, positional args, value types. Decide:

- how to express "flag takes value of type X"
- how subcommand inheritance works
- how to express mutually exclusive flag groups
- how to reference dynamic providers

---

### Provider API and plugin registration

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Provider = function returning `[(value, description, metadata)]`. Plugins register new providers under a name; schemas reference providers by name. Built-ins use the same API.

---

### Built-in providers: filesystem

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

`file`, `directory`, `executable`. Honor hidden defaults; configurable. Smart sorting (recent dirs higher, then alphabetical).

---

### Built-in providers: git

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

`git_branch` (with metadata: current marker, ahead/behind, last commit), `git_remote`, `git_remote_branch`, `git_tag`, `git_ref`, `git_stash`.

---

### Built-in providers: system

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

`process`, `signal`, `env_var`, `user`, `group`, `host_from_ssh_config`, `port`, `systemd_unit`.

---

### Cache layer with invalidation

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Per-provider TTL and invalidation rules:

- `executable` — invalidate on PATH change
- `git_branch` — invalidate on `cd` or git operation
- `process` — short TTL (~2s)
- `signal` — forever

Stale-while-revalidate by default: serve cache, refresh in background.

---

### Async provider execution

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Providers run on a worker pool. Menu opens immediately with whatever's in cache; results stream in as providers return.

---

### Fuzzy matcher

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Prefix → substring → fuzzy fallback. Returns match positions for UI highlight. fzf-style algorithm. Make per-provider matching configurable.

---

### Frecency ranking

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Track recency + frequency of accepted completions. Boost score in the matcher. Per-command (so `cd ~/projects` doesn't influence `vim ~/projects`).

---

### Completion menu UI

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Render rows with icon, name (with match highlight), description, metadata. Footer with key hints and result count. Inline below the prompt, height-limited with scrolling.

---

### Keyboard navigation

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Arrow keys to move, Tab to cycle/accept, Ctrl-N/P alternates, Esc dismisses. Mouse click to select. Configurable via the same keybindings system as the line editor.

---

### Autosuggest (ghost text)

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Fish-style inline suggestion from history as the user types. Right arrow accepts whole; Alt-Right accepts a word. Separate subsystem from menu completion but shares the matcher.

---

### Carapace importer

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

Parse Carapace YAML specs into Bento's native schema format. Unlocks ~1000 commands at once. Document the mapping for cases where Carapace concepts don't translate cleanly.

---

### Completions profiling

**Labels:** `area:completions`, `kind:feat`
**Milestone:** `0.4 — Completions`

`bento completions profile` shows per-provider timings across recent completions. Helps users find slow plugins in their setup.

---
