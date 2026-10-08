# Contracts: Experiments

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli experiment` | [cli-experiment.md](./cli-experiment.md) | User Stories 1 and 7; FR-001 to FR-011, FR-048 to FR-055 |
| `trcli step` | [cli-step.md](./cli-step.md) | User Story 2; FR-012 to FR-016 |
| `trcli run, sweep` | [cli-run.md](./cli-run.md) | User Stories 3 and 4; FR-017 to FR-037 |
| `trcli param, metric` | [cli-param.md](./cli-param.md) | User Story 4; FR-030, FR-031, FR-033 |
| `trcli result, figure, table` | [cli-result.md](./cli-result.md) | User Story 5; FR-038 to FR-044 |
| `trcli software` | [cli-software.md](./cli-software.md) | User Story 6; FR-045 to FR-047 |
