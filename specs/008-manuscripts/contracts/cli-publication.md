# Contract: `trcli publication`

**Spec**: [Manuscripts](../spec.md) — User Story 7; FR-042 to FR-048

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `pub`

## Synopsis

```text
trcli publication add <manuscript-ref> --venue <text|venue-ref> --date <date> [--as <article|preprint|other>]
                      [--version <n|label>] [--volume <text>] [--issue <text>] [--pages <text>]
                      [--doi <doi>] [--url <address>] [--license <text>] [--open-access | --closed-access]
trcli publication add <manuscript-ref> --doi <doi> --lookup
trcli publication list [--manuscript <ref>] [--from <date>] [--to <date>]
trcli publication show <ref> | edit <ref> [fields] | rm <ref>
trcli publication notice <ref> (--correction | --retraction) --date <date> --note <text>
trcli publications [--style <style>] [--group <year|kind>] [--from <date>] [--to <date>] [--kind <kind>]
                   [--project <ref>] [--grant <ref>] [--include-in-progress] [--to-file <file>]
                   [--format <document|bibtex|ris|csl-json>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `publication add` | Records the public form of a manuscript and sets its stage to `published`. A reference for the work is created in the library and linked; an existing reference for the same work is linked instead | 2 on a date in the future, a malformed identifier, or an abandoned manuscript |
| `publication add --lookup` | Fetches the details from the identifier, shows them, saves on acceptance — the same rules as `trcli ref add --doi` | 7 `lookup_unavailable`; nothing is stored |
| `publication add --as preprint` | Records a preprint; the stage is not changed. A later article is recorded as another version of the same work | — |
| `publication notice` | Records a correction or retraction, shown with the publication and in publication lists | 2 without `--note` |
| `publication rm` | Removes the record; the manuscript returns to `accepted`. The library reference is kept | 5 `confirmation_required` |
| `publications` | The researcher's publication list in a citation style, newest first, grouped by year or kind, with their own name marked. `--include-in-progress` adds submitted and accepted manuscripts under their own headings | 2 on unknown style or group |
| `publications --to-file` | Exports the list as a document or a bibliography file | 5 when the file exists and `--yes` is not given |

## Values

| Value | Rule |
|-------|------|
| `--date` | `YYYY-MM-DD`, not in the future |
| `--doi` | starts with `10.` and contains `/`; a resolver prefix is removed |
| `--version` | the manuscript version that was published; defaults to the latest |
| `--license` | up to 100 characters, for example `CC-BY-4.0` |
| the researcher's own name | taken from `researcher.name`; marked in `publications` wherever it appears among the authors |

## Example

```console
$ trcli publication add ms-5t1q --venue "Journal of Examples" --date 2027-06-02 --doi 10.1234/joe.2027.42 --open-access
Recorded pub-7d2x for ms-5t1q "Our Paper" (published 2027-06-02, open access)
Linked to the library as ref-4s9m (created)

$ trcli publications --group year --include-in-progress
2027
  **Silva, A.**, Costa, B., & Lima, C. (2027). Our paper. Journal of Examples, 12(3), 45–67.
  https://doi.org/10.1234/joe.2027.42

Accepted
  Costa, B., & **Silva, A.** Another paper. Conference on Things.

Submitted
  **Silva, A.** A third paper. Journal of Further Examples.
```
