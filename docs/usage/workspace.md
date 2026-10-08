# Workspace: `trcli init`, `trcli workspace`

## Purpose

A **workspace** is where your records are kept: a directory of your choice that holds a
`.trcli/` directory. Create one once; from then on TRCLI finds it from wherever you stand
inside that directory or beneath it. You can keep several workspaces, entirely separate.

```text
<workspace>/
└── .trcli/
    ├── trcli.db      your records (one file; copy the workspace by copying the directory)
    ├── config.toml   the workspace's settings
    ├── audit.head    the end of the audit trail, used to detect tampering
    └── backups/      a copy of trcli.db taken before each upgrade
```

Nothing inside refers to where the workspace is: move or rename the directory and it keeps
working. Keep it on a local disk, not in a folder synchronized by a cloud-storage client.

## Commands

| Command | What it does |
|---------|--------------|
| `trcli init [<dir>] --name <name> [--description <text>]` | Creates a workspace in `<dir>` (default: the current directory) |
| `trcli workspace show` | Name, description, location, format, and the number of records of each kind |
| `trcli workspace edit [--name <name>] [--description <text>] [--researcher <name>]` | Changes only what you name |
| `trcli workspace upgrade [--check]` | Brings an older workspace to the current format, keeping a copy first; `--check` only says whether one is needed |
| `trcli workspace check` | Verifies that the stored data is consistent and the audit trail is intact |

| Value | Rule |
|-------|------|
| `--name` | 1 to 200 characters; required by `init` |
| `--description` | up to 20,000 characters |
| `--researcher` | 1 to 200 characters; the name your actions are recorded under in the audit trail |

### How the workspace is found

In this order: `--workspace <dir>`; the `TRCLI_WORKSPACE` variable; the nearest `.trcli/`
at or above the current directory; the `default_workspace` setting. When the default is
used, the tool says so.

## Examples

Create a workspace and look at it:

```console
$ trcli init --name "Doctorate" --description "Thesis on soil microbes"
Created workspace "Doctorate" in [..]/work/.trcli
$ trcli workspace show
Workspace "Doctorate"
  Description  Thesis on soil microbes
  Location     [..]/work
  Format       1
  Created      2026-10-08 14:00 UTC
Records
  specimen     0
  sample-note  0
```

Change its details. Only what you name is changed, and the change is recorded:

```console
$ trcli workspace edit --name "Doctorate (2026)" --researcher "Ana Souza"
Updated workspace "Doctorate (2026)": name, researcher.name
$ trcli workspace show --output json
{
  "data": {
    "created_at": "2026-10-08T14:00:00Z",
    "description": "Thesis on soil microbes",
    "format_version": 1,
    "is_default": false,
    "location": "[..]/work",
    "name": "Doctorate (2026)",
    "records": [
      {
        "count": 0,
        "kind": "specimen"
      },
      {
        "count": 0,
        "kind": "sample-note"
      }
    ]
  },
  "ok": true,
  "warnings": []
}
```

Check that everything is sound:

```console
$ trcli workspace check
✓ The stored data is consistent.
✓ The audit trail is intact: 2 entries verified.
$ trcli workspace upgrade --check
No upgrade is needed: the workspace is in format 1.
```

Make one workspace reachable from anywhere:

```sh
trcli config set --user default_workspace ~/research
```

## When it fails

A workspace is created only where there is none (exit code 4):

```console
$ trcli init --name "Again"
error: a workspace already exists in [..]/work/.trcli
Nothing was changed.
[exit 4]
```

The name is required and checked, with every problem reported at once (exit code 2):

```console
$ trcli workspace edit --name "" --researcher ""
error: 2 values are invalid
  --name ""        must not be empty; expected 1 to 200 characters
  --researcher ""  must not be empty; expected 1 to 200 characters
Nothing was changed.
[exit 2]
```

| What happened | Exit code | What the tool says and what to do |
|---------------|-----------|-----------------------------------|
| No workspace here | 4 | How to create one (`trcli init`) or point to one (`--workspace`) |
| A workspace already exists | 4 | Nothing is changed |
| The workspace was made by a newer version | 4 | Nothing is changed; install a newer `trcli` |
| The workspace is in an older format | 4 for commands that would change something | Reading still works; run `trcli workspace upgrade` — a copy is kept in `.trcli/backups/` first, and restored if the upgrade fails |
| The stored data is damaged | 4 | What is wrong and where; nothing is overwritten; restore from `.trcli/backups/` |
| Another command is changing the workspace | 4 | The second command waits (`storage.busy_timeout_ms`), then says the workspace is busy; run it again |
| `workspace check` finds the audit trail altered | 6 | Which entry; see [audit.md](./audit.md) |
