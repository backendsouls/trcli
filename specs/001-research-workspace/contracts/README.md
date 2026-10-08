# Contracts: Research Workspace (datasets and reproducibility)

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli dataset` | [cli-dataset.md](./cli-dataset.md) | User Story 7; FR-040 to FR-042 |
| `trcli env, repro` | [cli-env.md](./cli-env.md) | User Story 8; FR-043 to FR-047 |

The contracts that used to live here and applied to every command — CLI conventions,
output and exit codes, configuration, and the workspace, link, and audit commands — moved
to `specs/000-foundation/contracts` on 2026-10-08.
