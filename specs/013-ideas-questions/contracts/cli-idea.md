# Contract: `trcli idea`, `trcli inbox`

**Spec**: [Ideas, Topics, Questions, and Hypotheses](../spec.md) — User Stories 1, 2, and 6; FR-001 to FR-025

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `tht` (a thought in the inbox), `ida` (an idea)

## Synopsis

```text
trcli idea "<text>" [--from <record-ref>]... [--by <person>] [--hint <idea|question|hypothesis|task>]
                    [--topic <topic>]...
trcli idea - [--lines]                      # read the text from the standard input
trcli idea --file <path> [--lines]

trcli inbox [--limit <n>] [--parked] [--discarded]
trcli inbox sort [--limit <n>]
trcli inbox keep <thought>... [--title <text>] [--topic <topic>]...
trcli inbox to question <thought> [--parent <question-ref>]
trcli inbox to hypothesis <thought> --question <question-ref>
trcli inbox to task <thought> [--due <date>]
trcli inbox to topic <thought>
trcli inbox merge <thought> <idea-ref|question-ref>
trcli inbox split <thought>
trcli inbox park <thought>... (--until <date> | --for <span>)
trcli inbox discard <thought>... --because <text>
trcli inbox restore <thought>
trcli inbox undo <thought>

trcli idea add --text <text> [--title <text>] [--topic <topic>]...
trcli idea develop <ref> [--title <text>] [--description <text>] [--why <text>] [--takes <text>]
trcli idea append <ref> <text>
trcli idea argue <ref> (--for | --against) <text> [--reference <ref>]
trcli idea relate <ref> <idea-ref> [--builds-on | --resembles] [--remove]
trcli idea inspired <ref> <reference-ref>... [--remove]
trcli idea rate <ref> [--promise <1-5>] [--effort <1-5>]
trcli idea credit <ref> [--originator <person>] [--contributor <person>]...
trcli idea state <ref> <raw|developing|ready|dropped> [--because <text>]
trcli idea revive <ref>
trcli idea to <question|project|experiment|manuscript|task> <ref> [options of that record's `add`]
trcli idea history <ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply to `idea` and are not
repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `idea "<text>"` | **Capture.** Stores the text in the inbox with the date and time and prints one line. Asks nothing, ever. Belongs to the current project if there is one | 2 only when the text is empty; 4 `no_workspace` when there is no workspace and no default one — the text is printed back so it is not lost |
| `idea -` / `--file` | Captures from the standard input or a file; with `--lines`, one thought per non-empty line | 2 when empty |
| (hints) | `--hint` and `--topic` are remembered as suggestions for sorting; the thought still lands in the inbox | 0 with a notice when a named topic does not exist yet |
| `inbox` | Unsorted thoughts, oldest first, with age, origin, and hint, and how many are waiting | 0 with "inbox is empty" |
| `inbox sort` | Presents thoughts one at a time with similar existing ideas and questions; for each: keep, turn into, merge, split, park, discard, skip, or stop. The hinted outcome is offered first. Decisions made are kept when stopped | 5 when it cannot ask a person — use the direct commands |
| `inbox keep` | Keeps thoughts as ideas, optionally with a title and topics | 3 when a thought is not found |
| `inbox to …` | Creates the record from the thought's text, date, origin, and originator; the record remembers the thought | 3 when the question is not found, offering to create one from the same text |
| `inbox merge` | Adds the thought's text to an idea or question as a dated note | 3 when the target is not found |
| `inbox split` | Replaces a thought by two, each to be sorted | 5 when it cannot ask for the two texts |
| `inbox park` | Removes thoughts from the inbox until a date; they return by themselves | 2 on a date in the past or a span that is not positive |
| `inbox discard` | Discards with a reason; the thought is kept and can be listed with `inbox --discarded` | 2 without `--because` |
| `inbox restore` | Returns a discarded or parked thought to the inbox | — |
| `inbox undo` | Returns a sorted thought to the inbox; what was created from it is removed if unchanged, otherwise the tool asks whether to keep it | 5 when it cannot ask and the record was changed |
| `idea develop` | Sets the parts given: title, description, why it matters, what it would take | — |
| `idea append` | Adds a dated addition | 2 on empty text |
| `idea argue` | Adds an argument for or against, optionally backed by a reference | 3 when the reference is not found |
| `idea relate` | Links two ideas | 2 when both are the same idea |
| `idea rate` | Sets promise and effort; `idea list --sort promise\|effort\|value` orders by them | 2 outside 1–5 |
| `idea state` | Moves between states; `dropped` requires `--because` | 2 on unknown state or a missing reason |
| `idea revive` | Returns a dropped idea to the state it had before | 2 when not dropped |
| `idea to …` | Creates the record from the idea's title and description; the idea becomes `realized` and each shows the other | as the target record's `add` |
| `idea history` | Capture, sorting, additions, changes of state, and what it became | — |

## Values

| Value | Rule |
|-------|------|
| `<text>` | any text, any language, stored exactly as written; only empty text is refused |
| `<thought>` | a `tht-…` handle, or its position in `trcli inbox` (`1`, `2`, …) |
| `<person>` | a person from the register (`specs/012-people`) or a name in quotes |
| `<topic>` | a topic name; an unknown name is offered for creation at sorting |
| `--for`, spans | `<n>d`, `<n>w`, `<n>m` |
| list filters (`idea list`) | `--state`, `--topic`, `--promise-min`, `--originator`, `--search`; dropped and realized ideas are hidden unless asked for |
| nothing is erased | every thought is in the inbox, sorted into something, parked, or discarded-and-kept |
| in `trcli status` | "Inbox: 7 waiting, oldest 12 days" |

## Example

```console
$ trcli idea "try the vision paper's augmentation on our audio data" --from ref-7k3f
Captured tht-3m9c (inbox: 4 waiting)

$ trcli inbox sort
1 of 4 · captured 12 days ago · from ref-2j5n
  "does the effect disappear with more data?"
  Similar: rq-4m2p "Does pre-training reduce the labelled data needed?"
[k]eep  [q]uestion  [h]ypothesis  [t]ask  [m]erge  [s]plit  [p]ark  [d]iscard  [n]ext  [x] stop > h
Under which question? [rq-4m2p] ↵
Created hyp-8d4f under rq-4m2p

$ trcli inbox park 2 --for 3m
Parked tht-5p1a until 2027-01-08
```
