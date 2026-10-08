# Contract: Configuration and Workspace Layout

**Plan**: [../plan.md](../plan.md) | **Commands**: [cli-conventions.md](./cli-conventions.md)

## Where settings come from

Later sources override earlier ones:

1. Built-in defaults
2. User file — `<config dir>/trcli/config.toml`
3. Workspace file — `<workspace>/.trcli/config.toml`
4. Environment variables — `TRCLI_<SECTION>_<KEY>` (for example `TRCLI_OUTPUT_COLOR=never`)
5. Command-line options

| Platform | `<config dir>` |
|----------|----------------|
| Linux | `$XDG_CONFIG_HOME` or `~/.config` |
| macOS | `~/Library/Application Support` |
| Windows | `%APPDATA%` |

`trcli config path` prints the files in use. `trcli config set <key> <value>` writes to the
workspace file; with `--user`, to the user file. `trcli config list` shows every effective
value and which source it came from.

## Workspace layout

```text
<workspace>/
└── .trcli/
    ├── config.toml       # workspace settings
    ├── trcli.db          # the repository (SQLite); location set by storage.path
    ├── audit.head        # sequence and hash of the last audit entry (local to this copy)
    ├── backups/          # copy of trcli.db taken before each upgrade
    └── …                 # directories features add, such as runs/ for step logs
```

A workspace is found by, in order: `--workspace`, `TRCLI_WORKSPACE`, the nearest `.trcli/`
at or above the current directory, then `default_workspace` in the user file.

The `.trcli` directory must be on a local disk. Placing it in a folder synchronized by a
cloud-storage client is not supported and can damage the database.

## Settings

| Key | Type | Default | Scope | Meaning |
|-----|------|---------|-------|---------|
| `default_workspace` | path | unset | user | Workspace used when none is found from the current directory |
| `storage.path` | path | `.trcli/trcli.db` | workspace | Location of the repository; relative paths are relative to the workspace root |
| `storage.busy_timeout_ms` | integer 0–60000 | `5000` | both | How long to wait when another `trcli` process is writing |
| `output.format` | `human` \| `json` | `human` | both | Default output form |
| `output.color` | `auto` \| `always` \| `never` | `auto` | both | Colored output |
| `output.page_size` | integer 1–1000 | `50` | both | Default `--limit` for lists |
| `output.symbols` | `unicode` \| `ascii` | `unicode` | both | Special symbols, or plain characters only |
| `output.date_format` | `iso` | `iso` | both | Dates are shown as `YYYY-MM-DD` |
| `theme.success` / `theme.warning` / `theme.error` / `theme.handle` / `theme.heading` / `theme.muted` | style | green / yellow / red bold / cyan / bold / dim | both | Styles; a style is a color name plus optional `bold`, `dim`, `underline` |
| `researcher.name` | text 1–200 | OS user name | both | Name recorded as the actor |
| `citation.style` | style name or path to a `.csl` file | `apa` | both | Default citation style |
| `citation.key_pattern` | pattern | `{family}{year}{word}` | both | How citation keys are generated |
| `bibliography.format` | `bibtex` \| `ris` \| `csl-json` | `bibtex` | both | Default import/export format |
| `lookup.enabled` | boolean | `true` | both | Allows online lookup of paper details |
| `lookup.timeout_seconds` | integer 1–9 | `8` | both | Total time allowed for a lookup |
| `lookup.contact_email` | email | unset | user | Sent to catalogues that ask for a contact; nothing is sent when unset |
| `telemetry.enabled` | boolean | `true` | workspace | Records local telemetry (FR-052). Nothing is ever transmitted. |
| `environment.tools` | list of program names | `[]` | both | Programs whose versions are captured in snapshots |
| `environment.variables` | list of variable names | `[]` | both | The only environment variables captured |
| `run.shell` | `auto` \| path | `auto` | both | Shell used for steps defined with `--shell`: `sh` on Linux and macOS, `cmd` on Windows |
| `run.step_timeout_seconds` | integer ≥ 0 | `0` (none) | workspace | Stops an automated step that runs longer |

### Whose settings these are

The foundation registers `default_workspace`, `storage.*`, `output.*`, `theme.*`,
`researcher.name`, and `telemetry.enabled`. Every other setting in the table above was
listed here when this file belonged to the first specification and is **registered by the
feature that owns it** (`citation.*`, `bibliography.*`, `lookup.*` by literature;
`environment.*` and `run.*` by reproducibility and experiments). Each feature's own
contracts are the authority for its settings; the rules below apply to all of them.

## Rules

- Every value is validated when a file is loaded and when `config set` is run. An invalid
  value or an unknown key is an error naming the file, the key, and what is expected; the
  tool does not start with a partly valid configuration.
- `environment.variables` refuses names that contain `KEY`, `TOKEN`, `SECRET`, `PASSWORD`,
  `PASSWD`, or `CREDENTIAL` (case-insensitive).
- `lookup.timeout_seconds` is capped at 9 so that a failed lookup is always reported in
  under 10 seconds (a success criterion of `specs/005-literature`).
- Scope "workspace" settings are ignored, with a warning, when found in the user file;
  scope "user" settings likewise in a workspace file.
- No setting holds a secret. The tool stores no credentials.
- `NO_COLOR` (any value) and `CLICOLOR=0` turn color off regardless of `output.color`,
  except when `--color always` is given on the command line.
