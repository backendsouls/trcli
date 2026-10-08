# Contract: `trcli bib`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 3; FR-025 to FR-028

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `bib`

## Synopsis

```text
trcli bib add --name <text> [--purpose <text>] [--style <style>] [--order <author|year|added|manual>]
trcli bib put <ref> <reference-ref>... [--remove]
trcli bib move <ref> <reference-ref> --to <position>
trcli bib check <ref>
trcli bib export <ref> [--format <bibtex|ris|csl-json> | --formatted] [--to <file>]
trcli bib attach <ref> <draft-ref> [--remove]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `bib put` | Adds references to a bibliography, creating citations for those without one | 3 when a reference is not found |
| `bib move` | Sets a position by hand | 2 unless the order is `manual` |
| `bib check` | Reports missing required details, missing keys, likely duplicates, and two versions of one work | 6 `check_failed` when anything is reported |
| `bib export` | A file for a writing tool, or with `--formatted` the reference list in the bibliography's style | 0 with a notice when empty |
| `bib attach` | Makes a draft's citations those of the bibliography | — |

## Values

| Value | Rule |
|-------|------|
| `--name` | 1–200 characters, unique |

## Example

```console
$ trcli bib check bib-3x9a
bib-3x9a "Chapter 2": 2 problems
  ref-2j5n  missing "pages" (required by apa for articles)
  ref-7k3f, ref-9b1d  are versions of the same work
$ echo $?
6
```
