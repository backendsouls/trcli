# Contracts: Literature, References, and Bibliography

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli ref (alias: paper)` | [cli-ref.md](./cli-ref.md) | User Stories 1 and 2; FR-001 to FR-020 |
| `trcli cite` | [cli-cite.md](./cli-cite.md) | User Story 3; FR-021 to FR-024 |
| `trcli bib` | [cli-bib.md](./cli-bib.md) | User Story 3; FR-025 to FR-028 |
| `trcli read, annotate, summary` | [cli-read.md](./cli-read.md) | User Story 4; FR-029 to FR-035 |
| `trcli relation` | [cli-relation.md](./cli-relation.md) | User Story 5; FR-036 to FR-039 |
| `trcli review` | [cli-review.md](./cli-review.md) | User Story 6; FR-040 to FR-048 |
| `trcli extract, theme` | [cli-synthesis.md](./cli-synthesis.md) | User Story 7; FR-049 to FR-052 |
