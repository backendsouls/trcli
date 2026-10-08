# Settings: `trcli config`

## Purpose

Settings adjust how TRCLI behaves. Your own preferences live in one file and apply in
every workspace; choices that belong to a workspace live with that workspace; a variable
or an option overrides both for a session or a single command. At any time you can see
every setting, its value, and where that value comes from.

## Commands

| Command | What it does |
|---------|--------------|
| `trcli config list` | Every setting, its value in effect, and the source of that value |
| `trcli config get <key>` | One setting: meaning, allowed values, default, value in effect |
| `trcli config set <key> <value> [--user]` | Sets it for this workspace, or for yourself with `--user` |
| `trcli config unset <key> [--user]` | Removes your value; the next source applies again |
| `trcli config path` | Where the settings files are |

### Where values come from

Later sources win:

1. the built-in default;
2. your own file — `~/.config/trcli/config.toml` on Linux (or `$XDG_CONFIG_HOME`),
   `~/Library/Application Support/trcli/config.toml` on macOS,
   `%APPDATA%\trcli\config.toml` on Windows;
3. the workspace's file — `<workspace>/.trcli/config.toml`;
4. variables of the session — `TRCLI_<SECTION>_<KEY>`, for example `TRCLI_OUTPUT_COLOR=never`;
5. options of the command — `--output`, `--color`.

### The settings of the foundation

| Key | Allowed | Default | Where | Meaning |
|-----|---------|---------|-------|---------|
| `default_workspace` | a path | unset | yours | Workspace used when none is found from the current directory |
| `storage.path` | a path | `.trcli/trcli.db` | workspace | Where the workspace's database is |
| `storage.busy_timeout_ms` | 0–60000 | `5000` | both | How long to wait for another command that is changing the workspace |
| `output.format` | `human`, `json` | `human` | both | Default output form |
| `output.color` | `auto`, `always`, `never` | `auto` | both | Coloured output |
| `output.symbols` | `unicode`, `ascii` | `unicode` | both | Special symbols, or plain characters only |
| `output.page_size` | 1–1000 | `50` | both | Rows a list shows unless `--limit` says otherwise |
| `output.date_format` | `iso` | `iso` | both | Dates are shown as `YYYY-MM-DD` |
| `researcher.name` | 1–200 characters | your system user name | both | Name recorded as the actor in the audit trail |
| `telemetry.enabled` | `true`, `false` | `true` | workspace | Records local telemetry; nothing is ever transmitted |
| `theme.success`, `.warning`, `.error`, `.handle`, `.heading`, `.muted` | a colour, optionally `bold`, `dim`, `underline` | green, yellow, red bold, cyan, bold, dim | both | Styles of coloured output |

`NO_COLOR` (any value) and `CLICOLOR=0` turn colour off whatever `output.color` says,
unless `--color always` is given on the command line. No setting holds a secret.

## Examples

```console
$ trcli init --name "Doctorate"
Created workspace "Doctorate" in [..]/work/.trcli
$ trcli config get output.color
output.color
  Meaning  Coloured output.
  Allowed  one of: auto, always, never
  Default  auto
  Scope    both
  Value    auto
  Source   default
$ trcli config set output.page_size 20
Set output.page_size = 20 in this workspace
$ trcli config get output.page_size
output.page_size
  Meaning  How many rows a list shows unless --limit says otherwise.
  Allowed  a whole number from 1 to 1000
  Default  50
  Scope    both
  Value    20
  Source   workspace file [..]/work/.trcli/config.toml
$ trcli config unset output.page_size
Removed output.page_size from this workspace
$ trcli config set theme.handle "magenta bold"
Set theme.handle = magenta bold in this workspace
```

Set a preference for yourself, valid in every workspace:

```sh
trcli config set --user output.symbols ascii
```

Override for one command or one session, without storing anything:

```sh
trcli workspace show --output json
TRCLI_OUTPUT_COLOR=never trcli workspace show
```

## When it fails

An invalid value, or a setting that does not exist, is refused with what is allowed
(exit code 2), and nothing is changed:

```console
$ trcli config set output.page_size 0
error: 1 value is invalid
  <value> "0"  is out of range; expected a whole number from 1 to 1000 (for example: 1)
Nothing was changed.
[exit 2]
$ trcli config set output.colour never
error: 1 value is invalid
  <key> "output.colour"  is not a setting; expected a setting listed by `trcli config list` (one of: output.format, output.color, output.symbols, output.page_size, output.date_format)
Nothing was changed.
[exit 2]
$ trcli config set default_workspace /somewhere
error: 1 value is invalid
  <key> "default_workspace"  can only be set for a user; expected a setting that can be stored in this workspace; set it with --user
Nothing was changed.
[exit 2]
```

| What happened | Exit code | What the tool says and what to do |
|---------------|-----------|-----------------------------------|
| An invalid value or an unknown setting given to `config set` | 2 | What is allowed; nothing is changed |
| A settings file holds an invalid value or an unknown setting | 2, for every command | The file, the setting, and what is expected; correct or remove the line — the tool does not run on a partly valid configuration |
| A settings file cannot be read or is not valid TOML | 2 | The file and why |
| A setting is found where it does not belong (a workspace-only setting in your own file) | 0 | It is ignored, with a warning |
| The settings file cannot be written | 7 | Why; nothing is recorded |
