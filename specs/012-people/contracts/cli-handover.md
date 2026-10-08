# Contract: `trcli handover`

**Spec**: [People and Lab Management](../spec.md) — User Story 4; FR-026 to FR-032

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `hnd` — a handover is usually named by its person

## Synopsis

```text
trcli handover start <person-ref> [--leaving <date>]
trcli handover show <person-ref>
trcli handover give <person-ref> <item>... --to <person-ref> [--notes <text>]
trcli handover close <person-ref> <item>... [--notes <text>]
trcli handover keep <person-ref> <item>... [--notes <text>]
trcli handover note <person-ref> <item> <text>
trcli handover complete <person-ref> [--stays-as-collaborator]
trcli handover cancel <person-ref>
trcli handover export <person-ref> --to <file>
trcli staff return <person-ref> --position <text> --from <date>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `handover start` | Lists everything the person currently holds, by kind: projects, experiments, manual steps waiting for them, open tasks, datasets, manuscripts in progress, people they supervise | 2 when the person is not a current member or a handover is already open; 0 with a message when they hold nothing |
| `handover show` | The items with the decision taken for each, and those still undecided | 3 when no handover is open |
| `handover give` | Moves an item to another person; the item records from whom it came and when | 3 when the item or person is not found; 5 when the receiver is a former member |
| `handover close` | Closes an item: a task is cancelled, an experiment abandoned with the reason "handover", a waiting step failed with that reason | 5 `confirmation_required` |
| `handover keep` | Leaves an item with the departing person | — |
| `handover note` | Adds notes for whoever takes an item — where things are, what the next step was | 2 on empty text |
| `handover complete` | Makes the person a former member with their leaving date, or an outside collaborator with `--stays-as-collaborator` | 5 `confirmation_required` listing undecided items, when any remain |
| `handover cancel` | Cancels the handover; items already moved stay where they are | 5 `confirmation_required` |
| `handover export` | A document listing what went to whom, with the notes | 5 when the file exists and `--yes` is not given |
| `staff return` | Records that a former member is back, with a new position; earlier periods are kept | 2 when the person is not a former member |

## Values

| Value | Rule |
|-------|------|
| `<item>` | the handle of a record the person holds (`prj-…`, `exp-…`, `tsk-…`, `dat-…`, `ms-…`, `stf-…` for a supervisee), or `all:<kind>` for every item of a kind |
| `--leaving` | the person's last day; defaults to their expected end, or today |
| after completion | the person's name stays on every past record; they remain selectable as an author; they are hidden from `staff list` unless `--former` |
| manuscripts | authorship is never changed by a handover; only the responsibility for work in progress is |

## Example

```console
$ trcli handover start stf-2c5e --leaving 2027-02-28
Rocha, Davi holds 6 items:
  project      prj-3f8a  Capstone co-supervision      member
  experiment   exp-9d1b  Ablation study               responsible · 1 run paused at a manual step
  dataset      dat-7p2q  Interviews                   responsible
  task         tsk-1k4m  Clean transcripts            due 2026-11-02
  task         tsk-2l5n  Write methods section        due 2026-12-01
  manuscript   ms-3e8k   Study B                      first author · drafting

$ trcli handover give stf-2c5e exp-9d1b dat-7p2q --to stf-4a9b --notes "Raw audio is on the lab disk under /interviews/2026"
Gave exp-9d1b and dat-7p2q to Silva, Ana

$ trcli handover complete stf-2c5e --stays-as-collaborator
3 items are undecided: tsk-1k4m, tsk-2l5n, ms-3e8k. Complete anyway? [y/N]
```
