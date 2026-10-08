# Contracts: Courses, Curricula, and Roadmaps

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli course`, `trcli term`, `trcli grading` | [cli-course.md](./cli-course.md) | User Story 1; FR-001 to FR-010 |
| `trcli programme`, `trcli curriculum` | [cli-curriculum.md](./cli-curriculum.md) | User Stories 2 and 3; FR-011 to FR-029 |
| `trcli plan` | [cli-plan.md](./cli-plan.md) | User Story 4; FR-030 to FR-037 |
| `trcli requirement`, `trcli programme extend` | [cli-requirement.md](./cli-requirement.md) | User Story 5; FR-038 to FR-049 |
| `trcli roadmap` | [cli-roadmap.md](./cli-roadmap.md) | User Story 6; FR-050 to FR-057 |

Two words, two commands: the **graduate roadmap** of a programme is managed with
`trcli requirement`; a personal **learning roadmap** with `trcli roadmap`.

This specification adds sources to a command it does not own: assessments, graduate
requirements, and plan terms appear in `trcli due` (`specs/003-research-projects`).
