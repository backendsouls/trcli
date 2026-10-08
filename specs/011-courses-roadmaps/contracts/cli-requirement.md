# Contract: `trcli requirement` (graduate roadmap)

**Spec**: [Courses, Curricula, and Roadmaps](../spec.md) — User Story 5; FR-038 to FR-049

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `req`

A programme's graduate roadmap is the set of its requirements. Without `--programme`, the
programme in use is meant.

## Synopsis

```text
trcli requirement propose [--programme <ref>] [--accept <all|n,n,...>]
trcli requirement add --title <text> --kind <kind> [--description <text>] [--optional]
                      [--within-months <n> | --by <date>] [--amount <n> --unit <text>] [--after <ref>]...
trcli requirement due <ref> (--within-months <n> | --by <date> | --none)
trcli requirement evidence add <ref> (<record-ref> [--stage <stage>] | --note <text>) [--amount <n>]
trcli requirement evidence list <ref> | rm <ref> <n>
trcli requirement satisfy <ref> [--date <date>] [--override]
trcli requirement waive <ref> --because <text>
trcli requirement reopen <ref>
trcli requirement roadmap [--programme <ref>] [--to <file>]
trcli requirement summary [--programme <ref>]
trcli programme extend <programme-ref> --months <n> --because <text> [--leave --from <date>]
trcli programme extensions <programme-ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `requirement propose` | Shows typical requirements for the programme's level (master's, doctoral) and adds those accepted; ones already present are skipped | 0 with a message for levels with no proposal |
| `requirement add` | Adds a requirement at status `pending` | 2 on a limit of zero or negative months, a date before enrolment, or an amount that is not positive |
| `requirement due` | Sets or clears the time limit. A limit in months follows the enrolment date and any extension | 2 as above |
| `requirement evidence add` | Attaches a course, manuscript (at a stated stage), milestone, document, or note; for a requirement with an amount, what this evidence contributes. When the amount required is reached, offers to mark the requirement satisfied | 3 when the record is not found; 2 when `--amount` is given for a requirement without one |
| `requirement satisfy` | Marks satisfied with the date | 5 when a requirement it must follow is not satisfied, unless `--override` (recorded); 2 when the date is in the future |
| `requirement waive` | Marks waived; the justification is required | 2 without `--because` |
| `requirement roadmap` | Requirements in order of due date with status, time remaining or overdue, and evidence; the next is highlighted. `--to` writes a document for a supervisor or programme office | — |
| `requirement summary` | Satisfied out of total, next deadline, latest date to complete the programme, and whether the researcher is within the time allowed | 6 `check_failed` when a requirement is missed or the time allowed has passed |
| `programme extend` | Records an extension of time or, with `--leave`, a leave of absence; due dates counted from enrolment and the latest completion date move, and the tool lists what moved | 2 on months that are not positive, or without `--because` |
| (evidence changes) | When evidence is deleted, or a manuscript given as evidence moves to an earlier stage, the requirement is flagged for review; its status does not change by itself | — |
| (a linked milestone is reached) | The tool offers to mark the requirement satisfied; it does so only on acceptance | — |

## Values

| Value | Rule |
|-------|------|
| `<kind>` | `credits`, `course`, `exam`, `language`, `qualifying`, `proposal`, `teaching`, `publication`, `attendance`, `residency`, `thesis`, `defense`, `deposit`, `other` |
| status | `pending`, `in_progress` (has evidence), `satisfied`, `waived`, `missed` (limit passed, neither satisfied nor waived) |
| `--stage` | for a manuscript as evidence: the stage it must have reached, for example `submitted` or `published` |
| `--unit` | what the amount counts: `credits`, `papers`, `seminars`, `hours`, … |
| in `trcli due` | every requirement with a limit appears as kind `requirement` |
| proposals | *Master's*: minimum credits · language proficiency · qualifying exam or proposal defense · dissertation submitted · defense · final version deposited. *Doctoral*: minimum credits · proficiency in one or two languages · qualifying exam · proposal defense · teaching internship · a paper submitted or published · thesis submitted · defense · final version deposited. Generic starting points; the tool does not know any programme's rules |

## Example

```console
$ trcli requirement roadmap
Doctorate in Computer Science · enrolled 2026-03-01 · latest completion 2031-02-28 (60 months)

  ✔ req-1a4d  Minimum credits            24 / 24 credits           satisfied 2027-07-10
  ✔ req-2b7k  English proficiency        exam result attached      satisfied 2026-09-15
▶ ◐ req-3c9m  Qualifying exam            by month 24 · 2028-02-29   in 144 days
  ◐ req-4d2p  Paper submitted            1 / 1 papers  ms-5t1q submitted   awaiting your confirmation
  ○ req-5e6r  Teaching internship        by month 36 · 2029-02-28   in 509 days
  ○ req-6f8t  Proposal defense           by month 30 · 2028-08-31   after req-3c9m
  ○ req-7g1v  Thesis defense             by month 60 · 2031-02-28   after req-6f8t

2 of 7 satisfied · next: Qualifying exam in 144 days · within the time allowed ✔
```
