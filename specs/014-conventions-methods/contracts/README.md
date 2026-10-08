# Contracts: Conventions, Methodologies, and the Implicit Side of Research

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli convention` | [cli-convention.md](./cli-convention.md) | User Story 1; FR-001 to FR-010 |
| `trcli method` (also `methodology`) | [cli-method.md](./cli-method.md) | User Story 2; FR-011 to FR-017 |
| `trcli applies`, `trcli departure`, `trcli read` | [cli-applies.md](./cli-applies.md) | User Story 3; FR-018 to FR-025 |
| `trcli assumption`, `trcli decision`, `trcli lesson` | [cli-assumption.md](./cli-assumption.md) | User Story 4; FR-026 to FR-032 |
| `trcli checklist` | [cli-checklist.md](./cli-checklist.md) | User Story 5; FR-033 to FR-039 |
| `trcli handbook` | [cli-handbook.md](./cli-handbook.md) | User Story 6; FR-040 to FR-046 |

A note on words: "CLI conventions" above means the rules of the `trcli` command line itself.
A `trcli convention` is something else — a convention of the researcher's lab or field,
recorded as data.

The origin of an entry is written the same way in every command: `own`, `lab`,
`institution`, `community`, `venue`, `funder`, `literature`.
