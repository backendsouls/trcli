# Contracts: People and Lab Management

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli staff` (alias: `person`) | [cli-staff.md](./cli-staff.md) | User Stories 1 and 2; FR-001 to FR-018 |
| `trcli staff supervise`, `trcli meeting` | [cli-meeting.md](./cli-meeting.md) | User Story 3; FR-019 to FR-025 |
| `trcli handover`, `trcli staff return` | [cli-handover.md](./cli-handover.md) | User Story 4; FR-026 to FR-032 |

Assigning a person to most kinds of work is done from the work itself, in the contract of
the specification that owns it: `trcli manuscript author` (`specs/008-manuscripts`),
`trcli experiment edit --responsible` and `trcli step edit --performer`
(`specs/004-experiments`), `trcli task add --responsible` and `trcli project add
--supervisor` (`specs/003-research-projects`). This specification adds project membership
(`trcli staff assign`) and gathers all of them per person.
