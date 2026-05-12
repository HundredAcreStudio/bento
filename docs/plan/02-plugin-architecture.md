# 0.2 — Plugin Architecture

Config, hooks, and plugin manager. Every issue in this file belongs to milestone `0.2 — Plugin Architecture`.

---

### Design: plugin scripting language

**Labels:** `kind:design`
**Milestone:** `0.2 — Plugin Architecture`

Decide between:

- embedded Lua (via mlua) — proven, small, fast
- native Bento script — best DX inside Bento, more work
- WASM modules — sandboxed, language-agnostic, heavier
- hybrid (declarative TOML for config + scripting language for logic)

Write the ADR. Blocks the plugin loader.

---

### XDG config discovery and TOML loader

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Resolve per XDG Base Directory spec:

- `~/.config/bento/config.toml` — main settings
- `~/.config/bento/plugins/` — installed plugins
- `~/.config/bento/themes/` — installed themes
- `~/.local/share/bento/` — state (history, caches)

Parse the config file and validate against the schema.

---

### Plugin loader and lifecycle

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Discover, load, initialize, and tear down plugins. Error isolation — one broken plugin must not crash the shell. Surface load errors via a `bento doctor` command.

---

### Hook registration and dispatch

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Plugins register callbacks; the shell invokes them at well-defined points with structured context (cwd, exit code, command, duration). Support ordering / priority for cases where order matters.

---

### Standard hook set

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Implement and document:

- `on_startup` — once at shell launch
- `before_prompt` — before each prompt is drawn
- `after_command` — after each command, with exit code + duration
- `on_cd` — directory change
- `on_keypress` — for live features (autosuggest, syntax highlighting)
- `on_exit` — cleanup

---

### Plugin stdlib

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Built-in helpers plugins can call:

- `bento.git.branch()`, `bento.git.is_dirty()`, `bento.git.ahead_behind()`
- `bento.color.rgb()`, `bento.color.named()`
- `bento.path.shorten(cwd, n)`, `bento.path.contains_file()`
- `bento.exec(cmd)`, `bento.exec_capture(cmd)`
- `bento.fs.read()`, `bento.fs.exists()`

Highest-leverage thing for plugin author DX. Document everything.

---

### Async hook execution

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Hooks return values or promises. Slow plugins don't block the prompt. Define the timeout and cancellation model.

---

### Lazy plugin loading

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Plugins declare triggers (commands, hooks they care about) in their manifest. Bento defers loading until a trigger fires. Critical for startup time.

---

### Config hot reload

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

Watch the config file (an mtime poll in `before_prompt` is enough). On change, reload affected subsystems without an `exec $SHELL`.

---

### Plugin manifest format

**Labels:** `area:plugins`, `kind:design`
**Milestone:** `0.2 — Plugin Architecture`

Spec the manifest schema: name, version, description, dependencies, triggers, declared hooks, declared completions, declared segments. TOML, validated on install.

---

### Plugin manager: install and update

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

- `bento plugin install <user/repo>` — clones from GitHub, validates manifest, enables
- `bento plugin update` — pulls all enabled plugins
- `bento plugin update <name>` — single plugin

Support tagged releases, not just main branch.

---

### Plugin manager: enable, disable, pin, list

**Labels:** `area:plugins`, `kind:feat`
**Milestone:** `0.2 — Plugin Architecture`

- `bento plugin enable/disable <name>`
- `bento plugin pin <name> <version>`
- `bento plugin list` — with status indicators
- `bento plugin info <name>` — show manifest + status

---
