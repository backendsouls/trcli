# Contract: `trcli concept`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 13; FR-054

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `cpt`

## Synopsis

```text
trcli concept add --term <text> --definition <text> [--synonym <text>]...
trcli concept relate <ref> <broader|narrower|related> <ref> [--remove]
trcli concept find <text>
trcli concept link <ref> <reference-ref>... [--remove]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `concept add` | Adds a term with its definition and synonyms | 2 when the term already exists, naming the existing concept |
| `concept relate` | Relates two concepts; visible from both | 2 when relating a concept to itself |
| `concept find` | Finds a concept by term or synonym | 3 when none matches |

## Values

| Value | Rule |
|-------|------|
| `--term` | 1–200 characters, unique ignoring case and accents |

## Example

```console
$ trcli concept find "self attention"
cpt-6r1n "Self-attention" (found by synonym "self attention")
```
