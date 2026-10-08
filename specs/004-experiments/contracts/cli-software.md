# Contract: `trcli software`

**Spec**: [Experiments](../spec.md) — User Story 6; FR-045 to FR-047

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `sw`

## Synopsis

```text
trcli software add --name <text> --location <path-or-url> [--license <text>] (--own | --third-party)
trcli experiment edit <ref> --software <ref>...
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `software add` | Registers code or a tool an experiment depends on | 2 when a local location does not exist |
| (effect on `run start`) | The exact version of each linked piece of software is recorded. Own code with unrecorded changes marks the run and warns; code with no change history is fingerprinted instead | 0 with a warning |
| (effect on `run compare`, `repro check`) | Differences in software versions are listed | — |
| `software rm` | Shared verb | 5 `blocked_by_dependents` when a run recorded it |

## Values

| Value | Rule |
|-------|------|
| `--own` / `--third-party` | exactly one is required |

## Example

```console
$ trcli run start exp-2b6r
warning: sw-1k5j "trainer" has changes not yet recorded in its history; this run is marked accordingly
```
