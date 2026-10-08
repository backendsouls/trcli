# Contract: `trcli manuscript venue`

**Spec**: [Venues — Places to Publish](../spec.md) — User Story 3; FR-023 to FR-031

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

These commands extend `trcli manuscript` of `specs/008-manuscripts`
([cli-manuscript.md](../../008-manuscripts/contracts/cli-manuscript.md)); `trcli draft venue …`
works the same way.

## Synopsis

```text
trcli manuscript venue add <manuscript-ref> <venue-ref | name>... [--fit <text>] [--at <position>]
trcli manuscript venue list <manuscript-ref>
trcli manuscript venue move <manuscript-ref> <venue-ref> --to <position>
trcli manuscript venue drop <manuscript-ref> <venue-ref> [--because <text>]
trcli manuscript venue compare <manuscript-ref> [--scheme <text>]
trcli manuscript venue fits <manuscript-ref> [<venue-ref | call-ref>]
trcli manuscript venue aim <manuscript-ref> <venue-ref | call-ref>
trcli manuscript venue next <manuscript-ref>
trcli manuscript venue suggest <manuscript-ref> [--limit <n>]

trcli venue counts add <name> --scheme <text> --at-least <value> [--order <v1,v2,...>] [--for <programme-ref | grant-ref>]
trcli venue counts list | rm <name>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `manuscript venue add` | Adds venues to the manuscript's shortlist in order of preference, each with a note on its fit. A name not in the register is offered for creation | 2 when a venue is already on the shortlist; 5 when a new venue would be created and it cannot ask |
| `manuscript venue list` | The shortlist in order, with each candidate's state: considering, tried, chosen, or dropped | 0 with a message when empty |
| `manuscript venue compare` | One table: kind, next deadline and time remaining, standing in the chosen scheme, costs, review model, open-access policy, the researcher's own time to decision and record there, and their interest. Candidates that fail a counting rule are marked | 2 with fewer than two candidates |
| `manuscript venue fits` | Compares the manuscript with the venue's or call's requirements — kind, length, language, anonymity — each as met, not met, or not checkable. Without a venue: against the current target | 6 `check_failed` when a requirement is not met |
| `manuscript venue aim` | Sets the manuscript's target venue and deadline from the venue or call, lists the manuscript under the call, and marks the candidate chosen. The deadline follows later changes to the call. Offers the venue's template for the manuscript's document | 5 for a venue marked to avoid, showing the recorded reason; 0 with a warning when the call's submission date has passed or the manuscript is under review elsewhere |
| `manuscript venue next` | After the manuscript was not accepted: marks the current target as tried and aims at the next candidate still being considered | 2 when no candidate remains |
| `manuscript venue suggest` | Venues from the register whose topics overlap with the manuscript's, that take its kind, and that are active and not marked to avoid; those with an upcoming deadline first | 0 with a message when the register has none that fit |
| `venue counts add` | Records which venues count for a programme, a funder, or the researcher: a ranking scheme and a minimum standing | 2 when `--at-least` is not among the values of `--order`, when given |

## Values

| Value | Rule |
|-------|------|
| candidate state | `considering` (default), `tried`, `chosen`, `dropped` |
| `--fit` | why this venue suits this manuscript; up to 2,000 characters |
| `--scheme` | the ranking scheme used for the comparison; defaults to `venues.preferred_scheme` |
| `--order` | the values of a scheme from best to worst, so the tool can tell what "at least" means; without it only equality is checked |
| what is compared in `fits` | only what the workspace knows: the manuscript's kind, word count, language, and whether its document hides the authors; everything else is listed as "check yourself" |
| suggestions | come only from the researcher's own register; the tool recommends nothing it has not been told about |
| settings | `venues.preferred_scheme` |

## Example

```console
$ trcli manuscript venue compare ms-3e8k --scheme Qualis
                   LREC 2027            JSpeech                 WSLR 2026
Kind               conference           journal                 workshop
Next deadline      2026-11-21 · 44 d    open                    2026-10-20 · 12 d
Qualis             A3 (2024)            A2 (2024)               B1 (2024)  ✘ below "PhD requirement" (A4)
Cost               registration 650 EUR publication 1,800 EUR   registration 200 EUR
Review             double-blind         double-blind            single-blind
Open access        full                 hybrid                  full
My time / record   —                    103 d · 1 of 2          41 d · 1 of 1
Interest           target               target                  watching
Fit                "main venue for      "room for the full      "fast feedback"
                    the resource"        analysis"

$ trcli manuscript venue aim ms-3e8k cfp-5d1r
ms-3e8k "Study B" is now aimed at LREC 2027 · Main (submission 2026-11-21, anywhere on earth)
Template recorded for this venue: lrec-paper — use it with: trcli doc new lrec-paper --draft ms-3e8k
```
