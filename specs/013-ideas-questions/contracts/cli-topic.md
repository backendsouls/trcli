# Contract: `trcli topic`

**Spec**: [Ideas, Topics, Questions, and Hypotheses](../spec.md) — User Story 4; FR-035 to FR-042

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `top` — a topic is usually named instead: `"label noise"`

## Synopsis

```text
trcli topic add <name> [--description <text>] [--under <topic>]... [--state <pursuing|watching|set-aside>]
trcli topic open <topic>
trcli topic put <topic> <record-ref>... [--remove]
trcli topic under <topic> <broader-topic> [--remove]
trcli topic state <topic> <pursuing|watching|set-aside>
trcli topic tree
trcli topic gaps
trcli topic merge <keep-topic> <other-topic>
trcli topic from-tag <tag> [--name <name>] [--keep-tag]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.
Every record noun also accepts `--topic <topic>` on `add` and as a filter on `list`.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `topic add` | Creates a topic, optionally under broader ones | 2 when the name exists (ignoring case and accents) or is empty |
| `topic open` | Everything under the topic and its narrower topics, grouped by kind, with counts, most recent activity first | 3 when not found, suggesting similar names |
| `topic put` | Places any records under a topic; a record may be under several and exists once | 3 when a record is not found |
| `topic under` | Places a topic under a broader one; a topic may have several | 2 when it would place a topic beneath itself, directly or indirectly |
| `topic list` | Shared verb. Each topic with its numbers of ideas, open questions, and references, and when something was last added; `--state` filters | — |
| `topic tree` | Topics arranged by broader and narrower | — |
| `topic gaps` | Topics that hold ideas but no research question | 0 with a message when none |
| `topic merge` | Keeps the first, holding everything of both | 5 `confirmation_required` |
| `topic from-tag` | Creates a topic holding every record that carries the tag; the tag is removed from them unless `--keep-tag` | 3 when no record carries the tag |
| `topic rm` | Lists what is under the topic; the records themselves are kept | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `<name>` | 1–100 characters; unique in the workspace ignoring case and accents |
| state | `pursuing` (default), `watching`, `set-aside` |
| a topic and a tag | are different things and may share a name; a topic has a description, a place among topics, and a page; a tag is a word on a record |
| a topic and a glossary concept | are different things: a concept defines a term (`specs/002-research-lifecycle`); a topic gathers work |

## Example

```console
$ trcli topic open "label noise"
Label noise · pursuing · under: Robustness
  Narrower: Annotator disagreement

Ideas (5)            ida-2f7h  Use disagreement as a signal, not as error         developing  ★★★★
                     ida-9k3m  Try the vision paper's augmentation on audio       raw
                     … 3 more
Questions (1 open)   rq-6t1b   How much label noise can pre-training absorb?      open · 1 hypothesis untested
References (12)      4 to read · 2 reading · 6 read
Experiments (1)      exp-2b6r  Baseline                                            active
Last activity        2026-10-08 (idea captured)

$ trcli topic gaps
Annotator disagreement   3 ideas · no research question yet
```
