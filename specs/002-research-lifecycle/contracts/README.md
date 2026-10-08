# Contracts: Research Lifecycle Extensions

The interfaces this specification exposes to users and scripts. Each file is the contract
for one group of commands; changing one after release is a breaking change.

Shared rules — grammar, global options, verbs every record has, output forms, and exit
codes — are in the [CLI conventions](../../000-foundation/contracts/cli-conventions.md) and are not repeated in each file.

| Commands | Contract | Covers |
|----------|----------|--------|
| `trcli backup, restore, export, upgrade` | [cli-backup.md](./cli-backup.md) | User Story 3; FR-013 to FR-019 |
| `trcli notebook` | [cli-notebook.md](./cli-notebook.md) | User Story 4; FR-020 to FR-023 |
| `trcli prereg` | [cli-prereg.md](./cli-prereg.md) | User Story 7; FR-033 to FR-037 |
| `trcli submission, review-comment` | [cli-submission.md](./cli-submission.md) | User Story 8; FR-038 to FR-041 |
| `trcli ethics, dmp` | [cli-ethics.md](./cli-ethics.md) | User Story 9; FR-042 to FR-046 |
| `trcli funder, grant` | [cli-grant.md](./cli-grant.md) | User Story 10; FR-047 to FR-050 |
| `trcli status` | [cli-status.md](./cli-status.md) | User Story 11; FR-051 |
| `trcli concept` | [cli-concept.md](./cli-concept.md) | User Story 13; FR-054 |
| `trcli instrument, sample, material` | [cli-lab.md](./cli-lab.md) | User Story 14; FR-055, FR-056 |
| `trcli publish` | [cli-publish.md](./cli-publish.md) | User Story 15; FR-057 to FR-059 |
| `trcli sync, type, extension` | [cli-integration.md](./cli-integration.md) | User Story 16; FR-060 to FR-063 |
| `trcli member, sync` | [cli-collab.md](./cli-collab.md) | User Story 17; FR-064 to FR-069 |
| `trcli due (extended)` | [cli-due-extended.md](./cli-due-extended.md) | User Story 2 (remaining part); FR-012 |
