# Bento — issue generation plan

This directory contains the plan for bootstrapping GitHub issues in the Bento repo. Each file describes a discrete set of objects to create.

**Repo:** `HundredAcreStudio/bento`

## Instructions for Claude Code

Work through the files in order. For each one:

1. **`00-labels-and-milestones.md`** — create the labels and milestones listed. Idempotent: skip any that already exist (check first with `gh label list` and `gh api repos/{owner}/{repo}/milestones`).
2. **`01-walking-shell.md` through `04-completions.md`** — for every issue block in each file, create a GitHub issue with:
   - the `### ` heading as the title
   - the `**Labels:**` line parsed as a comma-separated label list
   - the `**Milestone:**` line set as the issue's milestone (every issue in `0N-*.md` belongs to milestone `0.N`)
   - everything from the blank line after `**Milestone:**` up to the next `---` divider as the issue body (preserve markdown)
3. After each file, report how many issues were created and whether any failed.

**Tool:** use the `gh` CLI. Examples:

```bash
gh label create "area:core" --color "0E8A16" --description "..." --force
gh api repos/HundredAcreStudio/bento/milestones -f title="..." -f description="..."
gh issue create --title "..." --milestone "..." --label "lab1,lab2" --body "..."
```

**Safety:**
- Issues are not idempotent. Before creating any, run `gh issue list --limit 100 --json title --jq '.[].title'` and skip titles that already exist.
- If any single issue fails, log the failure and continue with the rest. Don't abort the run.
- Do not modify or close existing issues unless I explicitly ask.

**Order of operations:** finish the setup file before any milestone file, because issues reference milestones. Within milestone files, order doesn't matter.

## Inventory

| File | Contains | Count |
|---|---|---|
| `00-labels-and-milestones.md` | label + milestone definitions | 9 labels, 4 milestones |
| `01-walking-shell.md` | core REPL, parser, exec, pipes | 13 issues |
| `02-plugin-architecture.md` | config, hooks, plugin manager | 12 issues |
| `03-prompt-system.md` | segment renderer, themes, conditions | 14 issues |
| `04-completions.md` | schema, providers, menu UI, autosuggest | 14 issues |

Total: 53 issues across 4 milestones.
