# Contract: `trcli checklist`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 5; FR-033 to FR-039

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `chk` — an item is named by its number (`7`, or `3.2` under a heading)

## Synopsis

```text
trcli checklist add <name> [--origin <origin>] [--source <text|ref>] [--for <kind-of-work>]
trcli checklist heading add <ref> <text>
trcli checklist item add <ref> <statement> [--under <heading>] [--optional] [--guidance <text>]
trcli checklist item edit <ref> <item> [fields] | move <ref> <item> --to <position> | rm <ref> <item>
trcli checklist import <file> --name <name> [--map <column>=<detail>]... [--dry-run]
trcli checklist export <ref> --to <file>
trcli checklist version add <ref> --note <text>

trcli checklist apply <ref> <work-ref>
trcli checklist answer <work-ref> <ref> <item> (--satisfied --where <text|record-ref>
                                               | --not-applicable --because <text>
                                               | --not-satisfied [--note <text>])
trcli checklist go <work-ref> <ref>
trcli checklist state <work-ref> [<ref>]
trcli checklist report <work-ref> <ref> --to <file>
trcli checklist unapply <work-ref> <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `checklist add` | Creates an empty checklist at version 1 | 2 when the name is in use or the origin is unknown |
| `checklist item add` | Adds an item, required unless `--optional` | 2 on an empty statement |
| `checklist import` | Brings items in from a table, one row per item; reports each invalid row. `--dry-run` stores nothing | 2 when the file cannot be read as a table or every row is invalid |
| `checklist version add` | Records the checklist as it stands as a new version; existing applications keep the items they were applied with | 2 without `--note` |
| `checklist apply` | Applies a checklist to a manuscript, experiment, literature review, or project; every item starts unanswered | 5 when it is already applied to that work, asking whether a second application is intended |
| `checklist answer` | Answers one item | 2 for `--satisfied` without `--where`, or `--not-applicable` without `--because`; 3 on unknown item |
| `checklist go` | Presents unanswered items one at a time with their guidance; can be stopped at any time | 5 when it cannot ask a person |
| `checklist state` | Satisfied, not applicable, not satisfied, and unanswered, required items distinguished, with the unanswered and not-satisfied listed. Says when a newer version of the checklist exists and what differs | 6 `check_failed` when a required item is unanswered or not satisfied; 0 when complete |
| `checklist report` | A document listing each item, its answer, and where it is addressed — the form venues ask for | 5 when the file exists and `--yes` is not given |
| `checklist unapply` | Removes an application and its answers | 5 `confirmation_required` |
| (in `trcli manuscript ready`) | Required items unanswered or not satisfied in checklists applied to the manuscript are among the findings | — |

## Values

| Value | Rule |
|-------|------|
| `<name>` | 1–200 characters, unique in the workspace |
| `--for` | `manuscript`, `experiment`, `review`, `project`, or other text |
| `--where` | a part of the manuscript, a record, a page or section, or a note; required for `--satisfied` |
| `--map` | `<column heading or number>=<detail>`, details being `number`, `heading`, `statement`, `required`, `guidance` |
| licensed guidelines | nothing is shipped; for a guideline whose text may not be copied, record its item numbers with labels of your own and a link as `--source` |
| a departure from a checklist item | recorded with `trcli departure add … --from <checklist>#<item>` |

## Example

```console
$ trcli checklist state ms-5t1q
Lab pre-submission list (lab · version 3) applied to ms-5t1q "Our Paper"
  required   9 satisfied · 1 not applicable · 2 unanswered        optional   1 satisfied · 2 unanswered

  Unanswered (required)
    6    Every figure has been regenerated from its recorded run
    11   Data availability statement names where each dataset can be obtained
$ echo $?
6

$ trcli checklist answer ms-5t1q chk-3d9a 11 --satisfied --where "Section 7, Data availability"
Item 11 satisfied · 1 required item left
```
