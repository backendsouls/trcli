# Contracts: Templates

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli template` | [cli-template.md](./cli-template.md) | User Stories 1, 3, 4, and 5; FR-001 to FR-005, FR-024 to FR-044 |
| `trcli doc` | [cli-doc.md](./cli-doc.md) | User Stories 1, 2, and 6; FR-006 to FR-023, FR-045 to FR-048 |

This specification also adds one option to commands owned by other specifications:
`--template <name>` on every command that produces a document. It is described in
[cli-template.md](./cli-template.md) and does not change those commands' other behavior.
