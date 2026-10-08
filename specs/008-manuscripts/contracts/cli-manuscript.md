# Contract: `trcli manuscript` (alias: `draft`)

**Spec**: [Manuscripts](../spec.md) — User Stories 1 to 5; FR-001 to FR-036

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `ms` (manuscript), `ver` (version)

`trcli draft …` is accepted everywhere `trcli manuscript …` is, with identical behavior.

## Synopsis

```text
trcli manuscript add --title <text> [--kind <kind>] [--type <text>] [--abstract <text>] [--keyword <text>]...
                     [--language <code>] [--venue <text|venue-ref>] [--deadline <date>]
                     [--target-length <n>] [--file <path>]... [--detail <name>=<value>]...
trcli manuscript duplicate <ref> [--title <text>]
trcli manuscript retype <ref> --kind <kind> [--type <text>]

trcli manuscript stage <ref> <stage> [--reason <text>]
trcli manuscript resume <ref>
trcli manuscript history <ref>
trcli manuscript overview [--include-closed]
trcli manuscript due [--within <span>]
trcli manuscript stalled [--after <span>]

trcli manuscript author set <ref> <staff-ref | "Family, Given; Affiliation">...
trcli manuscript author move <ref> <author> --to <position>
trcli manuscript author mark <ref> <author> [--corresponding] [--equal] [--remove]
trcli manuscript author role <ref> <author> <role>... [--remove]
trcli manuscript acknowledge <ref> <name> --for <text> [--remove]
trcli manuscript byline <ref>
trcli manuscript contributions <ref>
trcli manuscript acknowledgements <ref>
trcli manuscript by <staff-ref>

trcli manuscript version add <ref> --note <text> [--label <text>] [--force]
trcli manuscript version list <ref>
trcli manuscript version label <ref> <n> <text> | note <ref> <n> <text>
trcli manuscript version match <ref> [<n|label>]
trcli manuscript version compare <ref> <n|label> <n|label>
trcli manuscript version rm <ref> <n>

trcli manuscript cite <ref> <citation-ref|bibliography-ref>... [--remove]
trcli manuscript report <ref> <result|figure|table-ref>... [--remove]
trcli manuscript address <ref> <question|hypothesis-ref>... [--remove]
trcli manuscript rests-on <ref>
trcli manuscript using <record-ref>
trcli manuscript bib <ref> [--format <format> | --formatted] [--style <style>] [--to <file>]
trcli manuscript ready <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `manuscript add` | Creates a manuscript at stage `idea`; kind defaults to `paper`. Mentions any manuscript with the same title | 2 on any invalid value, all reported together |
| `manuscript duplicate` | Copies details, authors, and links; the copy starts at `idea` with no versions and is recorded as derived from the original | — |
| `manuscript retype` | Changes kind; shows which details no longer apply and which become available; keeps authors, versions, parts, and links | 5 `confirmation_required` |
| `manuscript stage` | Moves to any stage, forwards or backwards; dated and kept. `abandoned` requires `--reason`. Moving to `published` asks for publication details (see [cli-publication.md](./cli-publication.md)) | 2 on unknown stage, listing valid ones; 2 for `abandoned` without `--reason`; 0 with a warning when moved to `submitted` without a corresponding author |
| `manuscript resume` | Returns an abandoned manuscript to the stage it had before | 2 when it is not abandoned |
| `manuscript history` | Stage changes with how long each lasted, versions, and changes of venue, deadline, and author order, in time order | — |
| `manuscript overview` | Manuscripts grouped by stage with counts; stalled ones marked; abandoned and published left out unless `--include-closed` | — |
| `manuscript due` | Manuscripts due within the span, overdue first | 2 on an invalid span |
| `manuscript author set` | Sets the ordered authors from the staff register or by name and affiliation; offers to add unknown people to the register | 2 when a person appears twice; 3 when a staff reference is not found |
| `manuscript author role` | Records contribution roles for an author | 2 on unknown role, or when the person is not an author (offers `acknowledge`) |
| `manuscript byline` / `contributions` / `acknowledgements` | The author list with affiliations; the contribution statement; acknowledged people and supporting funders | — |
| `manuscript by` | Manuscripts a person is an author of, with their position | — |
| `manuscript version add` | Records the next numbered version with the date, stage, authors, parts, citations and findings, and — when files are set — their fingerprint, size, and word count | 5 when files are unchanged since the latest version and `--force` is not given; 0 with a notice when there are no files or they cannot be read as text |
| `manuscript version match` | Says whether the current files match a version (the latest by default) | 6 `check_failed` when they differ or are missing |
| `manuscript version compare` | Differences in stage, authors, word count, parts, citations, and reported results | 3 when a version is not found |
| `manuscript version rm` | Removes a version | 5 `blocked_by_dependents` when a submission or publication refers to it |
| `manuscript cite` / `report` / `address` | Links citations or bibliographies, reported findings, and questions or hypotheses | 3 when a record is not found |
| `manuscript rests-on` | Questions, citations, findings with their runs, experiments, datasets, methodologies, and grants, grouped by kind | — |
| `manuscript using` | Manuscripts that use a given record | — |
| `manuscript bib` | Exports the manuscript's reference list | 5 when `--to` exists and `--yes` is not given |
| `manuscript ready` | Reports replaced results, changed figure or table files, incomplete references, authors without affiliation, missing abstract, corresponding author, or files, a near or passed deadline, and the findings of the document's own check. Changes nothing | 6 `check_failed` when anything is found; 0 otherwise |
| `manuscript rm` | Lists versions, parts, links, and what refers to it; asks twice for a published manuscript. Files are never deleted | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `--title` | 1–500 characters, required for `add` |
| `<kind>` | `paper` (default), `thesis`, `report`, `proposal`, `book`, `presentation`, `other` |
| `--type` | more precise type within the kind, 1–100 characters (for example `journal-article`, `technical-report`, `doctoral-thesis`) |
| `<stage>` | `idea`, `outlining`, `drafting`, `revising`, `submitted`, `under_review`, `accepted`, `published`, `abandoned`; for a thesis `defended` and `deposited` are accepted for `accepted` and `published` |
| `--keyword` | 1–100 characters each, at most 30 |
| `--language` | a language code such as `en` or `pt-BR` |
| `--target-length` | words; a positive whole number |
| `--file` | a file or directory the manuscript is written in; must exist; recorded by location only |
| `--detail` | details particular to the kind: `institution`, `programme`, `degree`, `supervisor`, `funder`, `call`, `event` |
| `<author>` | a staff reference, a position in the list, or a family name that is unique among the authors |
| `<role>` | `conceptualization`, `methodology`, `software`, `validation`, `formal-analysis`, `investigation`, `resources`, `data-curation`, `writing-original-draft`, `writing-review-editing`, `visualization`, `supervision`, `project-administration`, `funding-acquisition` |
| list filters | `--kind`, `--type`, `--stage`, `--author <staff-ref>`, `--venue`, `--due-before <date>`, `--tag`, `--search`; `--sort deadline\|title\|stage\|updated` |
| settings | `manuscript.stalled_after` (default `60d`) |

## Example

```console
$ trcli manuscript add --title "Our Paper" --type journal-article --deadline 2027-01-15 --file paper/
Added ms-5t1q "Our Paper" (paper · journal-article, idea, due 2027-01-15)

$ trcli manuscript stage ms-5t1q submitted
warning: ms-5t1q has no corresponding author
ms-5t1q "Our Paper": drafting → submitted (drafting lasted 41 days)

$ trcli manuscript ready ms-5t1q
ms-5t1q "Our Paper" is not ready: 3 findings
  result     res-8u2m accuracy = 0.89 was replaced by run-9c3d (0.91)
  figure     fig-1n4s file changed since it was recorded
  author     "Costa, Bruno" has no affiliation
$ echo $?
6
```
