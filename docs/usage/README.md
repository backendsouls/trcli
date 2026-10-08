# TRCLI usage guides

One guide per group of commands. Every example in these guides is run against the tool by
`crates/trcli-cli/tests/usage.rs`, so what you read here is what the tool does.

| Guide | Commands |
|-------|----------|
| [workspace.md](./workspace.md) | `trcli init`, `trcli workspace show / edit / upgrade / check` |
| [config.md](./config.md) | `trcli config list / get / set / unset / path` |
| [records.md](./records.md) | what every kind of record shares: `list`, `show`, `rm`, `tag`, `note`; `trcli link`, `trcli tag list` |
| [audit.md](./audit.md) | `trcli audit list / verify / export`, `trcli telemetry show / on / off / status` |

## The words TRCLI uses

The same words are used in commands, help text, messages, and these guides.

| Word | Meaning |
|------|---------|
| **workspace** | The place your records are kept: a directory that contains `.trcli/`. Found from wherever you stand inside it. |
| **record** | One thing you keep: a reference, an experiment, a task. Each kind of record has its own commands. |
| **kind** | What a record is. A kind has a name (`reference`) and a prefix (`ref`). |
| **short name** (or **handle**) | What you type to name a record: the kind's prefix, a hyphen, and a short code, such as `ref-7k3f`. A unique beginning is enough. It never changes and is never reused. |
| **tag** | A label any record can carry: 1 to 50 characters of `a-z 0-9 - _`. |
| **note** | A dated remark attached to a record. |
| **link** | A connection between any two records, with a word for how they relate. Seen from both. |
| **setting** | Something you can adjust, such as `output.color`. See [config.md](./config.md). |
| **audit trail** | The record of every change: what was done, to what, by whom, when. It can be verified and cannot be edited. |
| **telemetry** | What the tool notes about its own use, for you: which commands ran, how long they took. It stays in the workspace and is never sent anywhere. |

## What every command does the same way

- **Checks first.** Every value you give is checked before anything changes. If something
  is wrong, nothing is changed and every problem is listed at once.
- **Two forms of output.** For people by default; `--output json` gives one JSON document
  with the same content, for success and for failure alike.
- **Results and messages are kept apart.** Results go to standard output; errors,
  warnings, and remarks go to standard error.
- **The exit code means something.**

  | Code | Meaning |
  |------|---------|
  | 0 | It worked |
  | 1 | Something unexpected went wrong |
  | 2 | Invalid input, or the command was not written correctly |
  | 3 | Nothing matched, or what you typed matches several records |
  | 4 | A problem with the workspace: none found, exists already, too new, needs an upgrade, damaged, busy |
  | 5 | A confirmation was needed and not given, or the action is not allowed |
  | 6 | A check did not pass |
  | 7 | Something outside the tool could not be done |
  | 130 | You interrupted the command |

- **Never waits for an answer nobody can give.** A command that needs confirmation and
  cannot ask (no terminal, or `--no-input`) fails with exit code 5 unless you give `--yes`.

## Options every command accepts

| Option | Meaning |
|--------|---------|
| `--workspace <dir>` | Use this workspace instead of the one found from the current directory (also: `TRCLI_WORKSPACE`) |
| `--output human\|json` | Output form |
| `--color auto\|always\|never` | Coloured output; `NO_COLOR` and `CLICOLOR=0` are respected |
| `-y`, `--yes` | Answer yes to confirmations |
| `--no-input` | Never ask a question; fail instead |
| `-q`, `--quiet` | Only results and errors |
| `-v`, `--verbose` | Diagnostic detail on standard error |

## Help, version, and completion

```console
$ trcli --version
trcli 0.1.0 (workspace format 1)
```

`trcli --help` lists the groups of commands; `trcli <command> --help` shows a command's
options and an example; `trcli help <command>` is the same.

`trcli completions <bash|zsh|fish|powershell>` prints a completion script for your shell:

```sh
trcli completions bash > ~/.local/share/bash-completion/completions/trcli
trcli completions zsh  > ~/.zfunc/_trcli
```
