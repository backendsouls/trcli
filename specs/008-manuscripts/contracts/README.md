# Contracts: Manuscripts

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli manuscript` (alias: `draft`) | [cli-manuscript.md](./cli-manuscript.md) | User Stories 1 to 5; FR-001 to FR-036 |
| `trcli part` | [cli-part.md](./cli-part.md) | User Story 6; FR-037 to FR-041 |
| `trcli publication`, `trcli publications` | [cli-publication.md](./cli-publication.md) | User Story 7; FR-042 to FR-048 |

Commands in other specifications that act on a manuscript keep working under either name:
`trcli draft report`, `trcli draft evidence`, and `trcli draft refresh`
(`specs/004-experiments`), `trcli submission` (`specs/002-research-lifecycle`), and
`trcli doc new --draft` (`specs/007-templates`).
