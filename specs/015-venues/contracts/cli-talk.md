# Contract: `trcli event attend`, `trcli talk`

**Spec**: [Venues — Places to Publish](../spec.md) — User Story 5; FR-037 to FR-041

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `tlk`

`trcli presentation` is accepted for `trcli talk`.

## Synopsis

```text
trcli event attend <event-ref> <planning|attended|not-attending>
trcli event date <event-ref> <label> <date>
trcli event cost <event-ref> --kind <registration|travel|lodging|other> --amount <n> --currency <code> [--note <text>]
trcli event follow-up <event-ref> <text>

trcli talk add --title <text> --kind <kind> --date <date> (--event <event-ref> | --host <text> [--place <text>])
               [--with <person>]... [--file <path>] [--invited] [--award <text>]
trcli talk link <ref> <manuscript|result|project|grant-ref>... [--remove]
trcli talk same <ref> <talk-ref>
trcli talks [--style <style>] [--group <year|kind>] [--from <date>] [--to <date>] [--kind <kind>]...
            [--project <ref>] [--grant <ref>] [--to-file <file>]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply to `talk` and are not
repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `event attend` | Records whether the researcher plans to attend, attended, or will not | 2 on unknown state |
| `event date` | Adds a practical date for attending — end of early registration, a travel-grant deadline; it appears in `trcli due` | 2 on a malformed date |
| `event cost` | Records a cost of attending, in its currency; never converted or added across currencies | 2 on a negative amount |
| `event follow-up` | Captures a follow-up from the event as a thought in the inbox, linked to the event | 2 on empty text |
| `talk add` | Records a presentation at an event, or — without one — with its host and place | 2 on a missing title, or neither `--event` nor `--host`; 0 with a warning when the date is outside the event's dates; 2 when `--file` does not exist |
| `talk link` | Links what the presentation was based on and the grant that paid for it | 3 when a record is not found |
| `talk same` | Records that two presentations used the same material | 2 when both are the same presentation |
| `talks` | The researcher's presentations, newest first, grouped by year or kind, in a citation style; invited ones and awards are marked. `--to-file` writes a document | 2 on unknown style or group; 5 when the file exists and `--yes` is not given |

## Values

| Value | Rule |
|-------|------|
| `<kind>` | `talk`, `poster`, `tutorial`, `demonstration`, `panel`, `seminar`, `other` |
| `--with` | co-presenters: people from the register (`specs/012-people`) or names |
| `--file` | the location of the slides or poster; recorded by location only |
| what is out of scope | booking, expense claims, and session schedules |

## Example

```console
$ trcli talk add --title "Disagreement as signal in speech annotation" --kind talk \
    --date 2026-05-14 --event evt-4k9s --with stf-4a9b --file talks/icsp2026.pdf
Added tlk-8n3q (talk at ICSP 2026)

$ trcli talks --group year --from 2026-01-01
2026
  Silva, A., & Costa, B. (2026, May 14). Disagreement as signal in speech annotation [Talk].
    ICSP 2026, Lisbon.
  Silva, A. (2026, March 3). What annotators disagree about [Invited seminar]. University of
    Example, Department of Linguistics.
```
