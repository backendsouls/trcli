# Contracts: Foundation and Architecture

The interfaces every part of TRCLI shares. The first three files are the rules **all**
commands of **all** specifications follow; the others are the contracts of the commands
this specification owns. Changing any of them after release is a breaking change.

## Rules shared by every command

| Contract | Content |
|----------|---------|
| [cli-conventions.md](./cli-conventions.md) | Grammar, how a record is named, global options, the verbs every record has, what every command guarantees, and where each command group is specified |
| [output-and-exit-codes.md](./output-and-exit-codes.md) | Streams, the form for people and the structured form, the names of problems, exit codes, confirmations |
| [configuration.md](./configuration.md) | Settings, the places they come from and their order, and the layout of a workspace |
| [feature-contract.md](./feature-contract.md) | What a feature built on the foundation receives and must provide; the gates it passes before being accepted |

## Commands owned by this specification

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli init`, `workspace`, `config`, `completions` | [cli-workspace.md](./cli-workspace.md) | User Stories 1 and 5; FR-001 to FR-009, FR-039 to FR-045 |
| `trcli link`, `tag`, `note` | [cli-link.md](./cli-link.md) | User Story 2; FR-015 to FR-017 |
| `trcli audit`, `telemetry` | [cli-audit.md](./cli-audit.md) | User Story 6; FR-046 to FR-054 |

These files were written while the foundation was still part of
`specs/001-research-workspace` and were moved here on 2026-10-08. Requirement numbers cited
inside them were updated in their headers only; where a file's body cites an "FR-" number,
it is one of that earlier numbering and is to be re-checked when this specification is
planned.
