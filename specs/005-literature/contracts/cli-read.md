# Contract: `trcli read, annotate, summary`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 4; FR-029 to FR-035

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `ann`

## Synopsis

```text
trcli read status <ref> <to_read|reading|read|discarded>
trcli read priority <ref> <low|normal|high>
trcli read rate <ref> --rating <1-5> [--relevance <low|medium|high>]
trcli read queue [--limit <n>]
trcli read activity [--from <date>] [--to <date>]
trcli annotate quote <ref> <text> --page <page> [--comment <text>]
trcli annotate highlight <ref> <text> --kind <kind> [--page <page>]
trcli annotate list [<ref>] [--kind <kind>] [--search <text>] [--tag <tag>]
trcli annotate edit <annotation-ref> ... | rm <annotation-ref>
trcli annotate export <annotation-ref> [--style <style>]
trcli summary set <ref> [--problem <text>] [--method <text>] [--findings <text>] [--limitations <text>] [--assessment <text>]
trcli summary show <ref>
trcli read notes <ref> [--to <file>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `read status` | Changes reading status; the date is recorded | — |
| `read queue` | Unread references by priority, then by date added | — |
| `annotate quote` | Stores a quote with its page | 2 on empty text or a page that is not a positive number or range |
| `annotate highlight` | Stores a highlight with a kind | 2 on unknown kind |
| `annotate list` | Annotations across the library, with reference and page | — |
| `annotate export` | The quote, its page, and the reference's citation in a style | — |
| `summary set` | Sets the parts given of the one structured summary of a reference | — |
| `read notes` | The summary and all annotations of a reference in page order | — |

## Values

| Value | Rule |
|-------|------|
| `--page` | `12` or `12-14` |
| `--kind` | `key-claim`, `method`, `finding`, `limitation`, `question`, `idea` |

## Example

```console
$ trcli annotate quote ref-7k3f "we propose a new simple network architecture" --page 1
Added ann-4g6w to ref-7k3f, p. 1
```
