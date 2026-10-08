# Contracts: Ideas, Topics, Questions, and Hypotheses

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli idea`, `trcli inbox` | [cli-idea.md](./cli-idea.md) | User Stories 1, 2, and 6; FR-001 to FR-025 |
| `trcli question`, `trcli hypothesis` | [cli-question.md](./cli-question.md) | User Story 3; FR-026 to FR-034 |
| `trcli topic` | [cli-topic.md](./cli-topic.md) | User Story 4; FR-035 to FR-042 |
| `trcli review-ideas` (also `revisit`) | [cli-review.md](./cli-review.md) | User Story 5; FR-043 to FR-050 |

## The one command that matters most

```console
$ trcli idea "whatever just crossed your mind"
```

It asks nothing, takes seconds, works from scripts, and never fails for a reason other than
an empty text or a missing workspace. Everything else here exists to make that capture
worth something later.

This specification also adds a `--topic <topic>` option to `add` and `list` of every record
noun, and lines to `trcli status` and to activity reports for the inbox and for what is due
for review.
