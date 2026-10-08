# Contracts: Research Projects, Milestones, and Tasks

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli project` | [cli-project.md](./cli-project.md) | User Stories 1 and 2; FR-001 to FR-018 |
| `trcli milestone` | [cli-milestone.md](./cli-milestone.md) | User Story 3; FR-019 to FR-027 |
| `trcli task` | [cli-task.md](./cli-task.md) | User Story 4; FR-028 to FR-039 |
| `trcli due, progress` | [cli-due.md](./cli-due.md) | User Story 5; FR-040 to FR-043 |
| `trcli project-type` | [cli-project-type.md](./cli-project-type.md) | User Story 6; FR-045 to FR-048 |
| `trcli todo` | [cli-todo.md](./cli-todo.md) | User Story 7; FR-053 to FR-072 |

## Shared by these commands

| Contract | Content |
|----------|---------|
| [json-output.md](./json-output.md) | The shape of `--output json` for projects, milestones, tasks, what is due, progress, and the to-do list |
