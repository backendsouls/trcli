# Contracts: Simulations

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli model` | [cli-model.md](./cli-model.md) | User Stories 1 and 5; FR-001 to FR-008, FR-038 to FR-044 |
| `trcli scenario` | [cli-scenario.md](./cli-scenario.md) | User Story 2; FR-009 to FR-017 |
| `trcli sim` | [cli-sim.md](./cli-sim.md) | User Stories 3, 4, and 6; FR-018 to FR-037, FR-045 to FR-049 |

A simulation is an experiment, and a replication is a run: every command of
`specs/004-experiments` (`trcli experiment`, `step`, `run`, `param`, `metric`, `result`,
`software`) works on them unchanged. This specification adds one option to a command it
does not own, `trcli experiment edit --model`, described in [cli-model.md](./cli-model.md).
