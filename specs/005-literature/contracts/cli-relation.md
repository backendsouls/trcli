# Contract: `trcli relation`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 5; FR-036 to FR-039

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli relation add <ref> <cites|extends|contradicts|replicates|reviews> <ref> [--comment <text>]
trcli relation rm <ref> <kind> <ref>
trcli relation versions <ref> <ref>... --prefer <ref>
trcli relation show <ref>
trcli relation most-cited [--limit <n>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `relation add` | Records how one reference stands to another; shown from both, each from its side | 2 when both are the same reference or the relation exists |
| `relation versions` | Records references as versions of one work and the preferred one for citing | 2 when `--prefer` is not among them |
| `relation show` | Works it relates to and works that relate to it, by kind | — |
| `relation most-cited` | References most cited by others in the library | — |

## Values

| Value | Rule |
|-------|------|
| `--comment` | up to 20,000 characters |

## Example

```console
$ trcli relation show ref-7k3f
cited by       3   ref-2j5n, ref-5p8c, ref-6m0q
extended by    1   ref-2j5n
contradicted   1   ref-8t4v  "does not hold for long sequences (sec. 5)"
version of     1   ref-9b1d (preprint) — preferred: ref-7k3f
```
