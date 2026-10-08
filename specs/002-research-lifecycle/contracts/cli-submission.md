# Contract: `trcli submission, review-comment`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 8; FR-038 to FR-041

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `sub`, `cmt`

## Synopsis

```text
trcli submission add <draft-ref> --venue <text|venue-ref> --version <n> --date <date>
trcli submission decide <ref> <accepted|minor_revision|major_revision|rejected|withdrawn> --date <date>
trcli submission comment add <ref> --reviewer <label> <text>
trcli submission comment respond <comment-ref> <text> [--version <n>] [--decline]
trcli submission comment list <ref> [--status <open|addressed|declined>]
trcli submission letter <ref> [--to <file>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `submission add` | Records sending a draft version to a venue; the draft becomes `submitted` | 2 when the version does not exist |
| `submission decide` | Records the decision; the draft's stage follows | 2 when the decision date is before the submission date |
| `submission comment add` | Records one reviewer comment with the status `open` | — |
| `submission comment respond` | Records the response and the version that addresses it | — |
| `submission letter` | Produces the response letter; unanswered comments are flagged | 0, with a warning on stderr when comments are unanswered |

## Values

| Value | Rule |
|-------|------|
| `--reviewer` | a label such as `R1`; 1–20 characters |
| list filters | `submission list --draft <ref> --venue <text> --decision <decision>` |

## Example

```console
$ trcli submission letter sub-4f8c --to response.md
Wrote response.md: 30 comments, 28 answered
warning: 2 comments have no response (cmt-1a9x, cmt-7q2e)
```
