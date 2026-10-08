# Contract: `trcli event`, `trcli call`

**Spec**: [Venues — Places to Publish](../spec.md) — User Story 2; FR-012 to FR-022

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `evt` (event), `cfp` (call)

## Synopsis

```text
trcli event add <venue-ref> --name <text> [--from <date>] [--to <date>] [--place <text>]
                [--mode <in-person|online|hybrid>] [--part-of <event-ref>]
trcli event state <ref> <announced|confirmed|cancelled|held>
trcli event next <ref> [--name <text>]
trcli event list [<venue-ref>] [--from <date>] [--to <date>] [--attending]

trcli call add <venue-ref | event-ref> --title <text> [--open]
               [--date <kind>=<when>]... [--expected]
               [--max-words <n>] [--max-pages <n>] [--anonymous | --not-anonymous] [--topic <text>]... [--note <text>]
trcli call date <ref> <kind> <when> [--expected | --confirmed] [--because <text>]
trcli call date rm <ref> <kind>
trcli call confirm <ref>
trcli call history <ref>
trcli call list [<venue-ref>] [--upcoming | --all]

trcli deadlines [--within <span>] [--kind <kind>]... [--interest <interest>]... [--mine]
trcli venue overdue-calls
```

The shared verbs `show`, `edit`, `rm`, `tag`, and `note` apply to `event` and `call`.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `event add` | Adds an occurrence of a venue that meets | 2 when `--to` is before `--from`, or the venue is a journal or preprint server |
| `event state` | Records that an event is confirmed, cancelled, or held. On `cancelled`, lists manuscripts aimed at its calls | — |
| `event next` | Creates the next edition from this one: same call structure, dates moved by a year and marked `expected` | — |
| `call add` | Adds a call to a venue or event with its dates. `--open` makes a call with no deadline, for a journal that always accepts | 2 when dates are out of order (for example notification before submission), a date is malformed, or `--open` is combined with `--date` |
| `call date` | Sets or changes one date. The earlier value is kept in the call's history; manuscripts aimed at the call follow | 2 when the change puts dates out of order |
| `call confirm` | Marks all of a call's expected dates as confirmed | — |
| `call history` | Every earlier value of every date, with when and why it changed | — |
| `deadlines` | Every upcoming date of every call, in order, with venue, kind, time remaining, and the manuscripts aimed at it; expected dates are marked. `--mine` keeps only calls a manuscript is aimed at | 2 on an invalid span |
| (in `trcli due`) | Every call date appears as kind `call`; after a call's submission date has passed, its later dates remain only for manuscripts sent to it | — |
| `venue overdue-calls` | Target and watched venues with no upcoming call recorded whose last edition was about a year ago or more — their next call is probably out | 0 with a message when none |

## Dates

| Form of `<when>` | Meaning |
|------------------|---------|
| `2027-01-15` | end of that day **anywhere on earth**; the tool says so wherever it shows the date |
| `2027-01-15T23:59 aoe` | that moment, anywhere on earth |
| `2027-01-15T17:00 America/Sao_Paulo` | that moment in a named time zone |
| `2027-01-15T17:00+01:00` | that moment at an offset |

Wherever a date is shown, the stated moment is shown, and the researcher's local moment as
well when it falls on a different calendar day.

## Values

| Value | Rule |
|-------|------|
| `<kind>` (of a date) | `abstract`, `submission`, `notification`, `final`, `other:<label>` |
| expected order | abstract ≤ submission < notification ≤ final |
| `--expected` | the date is a guess (for example copied from last year); shown as such until confirmed |
| a call's requirements | take the place of the venue's general ones for that call |
| several calls | a venue or event may have several at once: tracks, rounds, a special issue |
| settings | `venues.deadline_window` (default `90d`) for `deadlines` without `--within` |

## Example

```console
$ trcli event next evt-4k9s
Created evt-7b2d "ICSP 2027" from "ICSP 2026" · dates moved by a year and marked expected
  call "Main track": abstract 2027-01-08? · submission 2027-01-15? · notification 2027-03-20? · final 2027-04-10?

$ trcli deadlines --within 60d
IN        WHEN (anywhere on earth)       VENUE      CALL            KIND         AIMED HERE
 12 days  2026-10-20                     WSLR 2026  Workshop        submission   ms-9a1f
 37 days  2026-11-14 (local: Nov 15)     LREC 2027  Main            abstract     —
 44 days  2026-11-21                     LREC 2027  Main            submission   ms-3e8k
 99 days  2027-01-15 expected            ICSP 2027  Main track      submission   —
```
