# Contract: `trcli ref (alias: paper)`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Stories 1 and 2; FR-001 to FR-020

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `ref`

## Synopsis

```text
trcli ref add --title <text> [--kind <kind>] [--author "<Family>, <Given>"]... [--year <year>]
              [--venue <text>] [--abstract <text>] [--doi <doi>] [--arxiv <id>] [--isbn <isbn>]
              [--url <address>] [--language <code>] [--keyword <text>]... [--file <path>]
              [--field <name>=<value>]... [--on-duplicate <ask|add|merge|cancel>]
trcli ref add (--doi <doi> | --arxiv <id> | --isbn <isbn>)
trcli ref complete <ref>... | --all-incomplete [--overwrite]
trcli ref merge <keep-ref> <other-ref>
trcli ref duplicates
trcli ref by-author <name>
trcli ref same-author <name> <name>
trcli ref import <file> [--format <bibtex|ris|csl-json>] [--on-duplicate <skip|add|merge>] [--dry-run]
trcli ref export [filters] [--format <format>] [--to <file>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `ref add --title …` | Adds a reference by hand at reading status `to read` | 2 on any invalid value, all reported together |
| `ref add --doi …` (no title) | Looks the identifier up, shows the details, stores on acceptance | 7 `lookup_unavailable` with cause `no_connection`, `catalogue_unavailable`, `unknown_identifier`, or `disabled`; nothing is stored |
| (likely duplicate) | Reported; `--on-duplicate ask` prompts for add / merge / cancel | 5 `confirmation_required` when it cannot ask |
| `ref complete` | Fetches missing details; shows stored and fetched side by side; never overwrites without `--overwrite` and confirmation | 7 as above |
| `ref merge` | Keeps the first, fills its empty details from the second, moves citations, annotations, tags, links, relations, and review entries | 5 `confirmation_required` |
| `ref duplicates` | Groups of likely duplicates across the library | — |
| `ref import` | Imports every valid entry; reports each invalid one with its reason, then counts | 2 when the file cannot be read as any format or every entry is invalid; nothing changes with `--dry-run` |
| `ref export` | Writes the selected references | 5 when `--to` exists and `--yes` is not given |
| `ref rm` | Lists bibliographies, reviews, and drafts that use it | 5 `confirmation_required`; 5 `blocked_by_dependents` when included in a completed review |

## Values

| Value | Rule |
|-------|------|
| `--kind` | `article` (default), `conference-paper`, `preprint`, `book`, `chapter`, `thesis`, `report`, `web-page`, `dataset`, `software`, `standard`, `patent`, `other` |
| `--year` | 1000 to next year |
| `--doi` | starts with `10.` and contains `/`; a resolver prefix is removed |
| `--field` | details particular to the kind: `volume`, `issue`, `pages`, `publisher`, `edition`, `editor`, `institution`, `degree`, `accessed` |
| list filters | `--kind`, `--author`, `--year`, `--year-from`, `--year-to`, `--venue`, `--language`, `--status`, `--rating-min`, `--in <bibliography-ref>`, `--tag`, `--search` |

## Example

```console
$ trcli ref import library.bib --on-duplicate skip
Imported 987 of 1000 entries (11 invalid, 2 duplicates skipped)
  entry 14  (key: smith2020)   year "20x0" is not a year
  entry 203 (no key)           title is missing
  … 9 more (use --verbose to list all)
```
