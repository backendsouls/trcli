# Contract: `trcli staff supervise`, `trcli meeting`

**Spec**: [People and Lab Management](../spec.md) — User Story 3; FR-019 to FR-025

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `mtg`

## Synopsis

```text
trcli staff supervise <supervisor-ref> <person-ref> [--co] [--from <date>]
trcli staff supervise end <supervisor-ref> <person-ref> --on <date>
trcli staff supervisees [<supervisor-ref>]
trcli staff unmet [--for <span>]

trcli meeting add <person-ref>... [--date <date>] [--notes <text>] [--private]
                  [--action "<text> @<person-ref> by <date>"]...
trcli meeting action add <meeting-ref> <text> --owner <person-ref> --by <date>
trcli meeting list [<person-ref>] [--from <date>] [--to <date>]
trcli meeting show <meeting-ref>
trcli meeting edit <meeting-ref> [--notes <text>] [--private | --not-private] | rm <meeting-ref>
trcli meeting prepare <person-ref>
```

Without `<supervisor-ref>`, the person set with `trcli staff me` is meant.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `staff supervise` | Records that one person supervises another, as main supervisor or with `--co` as co-supervisor | 2 when both are the same person or it would form a loop |
| `staff supervise end` | Ends a supervision on a date; it stays in history | 3 when no such supervision exists |
| `staff supervisees` | Current supervisees with position, expected end, and next milestone | 2 when no supervisor is given and `staff me` is not set |
| `staff unmet` | Supervised people not met for longer than the span, with the date of their last meeting | 6 `check_failed` when any |
| `meeting add` | Records a meeting that happened, with attendees, notes, and agreed actions. Each action becomes a task linked to the meeting | 2 on a date in the future, an action without an owner or a date, or an owner who is not found |
| `meeting list` | Meetings in date order with their actions and whether each was done; for one person or all | — |
| `meeting prepare` | For the next meeting with a person: actions agreed last time and their status; what they completed and what became overdue since; their next milestone and programme requirement; the date of the last meeting | 0 with a message when there has been no meeting yet |
| `meeting rm` | Removes a meeting; its actions remain as tasks | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `--date` | defaults to today; not in the future |
| `--notes` | up to 20,000 characters |
| `--private` | the notes appear in nothing produced for others |
| `--action` | `"<what> @<who> by <date>"`; `<who>` is a handle or a unique family name among the attendees |
| `--for` | default `4w` (`people.unmet_after`) |
| attendees | the person set with `staff me` is always an attendee and need not be named |

## Example

```console
$ trcli meeting add stf-4a9b --notes "Baseline reproduced. Agreed to drop the third condition." \
    --action "Rerun baseline with new split @stf-4a9b by 2026-10-20" \
    --action "Send reading list on Bayesian models @me by 2026-10-12"
Recorded mtg-6h3d with Silva, Ana (2026-10-08) · 2 actions → tsk-5w0h, tsk-7x2k

$ trcli meeting prepare stf-4a9b
Silva, Ana · doctoral student · last met 2026-10-08 (14 days ago)

Agreed last time
  ✔ Rerun baseline with new split        tsk-5w0h  done 2026-10-17
  ✘ Send reading list on Bayesian models tsk-7x2k  yours · overdue since 2026-10-12

Since then      3 tasks done · 1 run succeeded · 1 result recorded · 2 tasks overdue
Next milestone  Qualifying exam · 2028-02-29 · in 130 days
Programme       Paper submitted — awaiting confirmation
```
