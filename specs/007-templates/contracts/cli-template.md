# Contract: `trcli template`

**Spec**: [Templates](../spec.md) — User Stories 1, 3, 4, and 5; FR-001 to FR-005, FR-024 to FR-044

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Referred to by**: name (`article`, `supervisor-weekly`), optionally with a level: `provided:article`, `personal:article`, `workspace:article`

## Synopsis

```text
trcli template list [--kind <kind>] [--level <provided|personal|workspace>] [--all]
trcli template show <name> [--files]
trcli template new <name> --kind <kind> --description <text> [--format <format>] [--level <personal|workspace>]
trcli template copy <name> <new-name> [--level <personal|workspace>]
trcli template path <name>
trcli template check <name>
trcli template placeholders [--kind <kind>] [--search <text>]
trcli template prefer <output> <name> [--user] | --clear
trcli template preferences
trcli template uses <name>
trcli template export <name>... --to <file>
trcli template import <file> [--level <personal|workspace>] [--on-conflict <ask|replace|rename|skip>] [--dry-run]
trcli template rm <name>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `template list` | Templates in effect, with kind, description, version, and level; `--all` also shows those hidden by a template of the same name at a nearer level | 2 on unknown kind or level |
| `template show` | Sections in order (required or optional), guidance, questions, placeholders used, limits, and with `--files` the files it creates | 3 when not found, suggesting similar names |
| `template new` | Creates an empty template of the user's own and prints where to edit it | 2 when the name exists at that level (ignoring case and accents) or is malformed |
| `template copy` | Copies any template, including a provided one, under a new name | 2 when the new name exists at that level |
| `template path` | Prints the location of a template's files for editing | 5 for a provided template, offering `template copy` |
| `template check` | Reports every mistake with its place and what is expected: unknown placeholder, duplicated or unnamed section, undeclared or duplicated question, default of the wrong kind, invalid limit, missing or looping inclusion, unbalanced repeat or condition, anything that is not text or workspace information | 6 `check_failed` when anything is reported |
| `template placeholders` | Every placeholder available, with a description and the kind of value it yields | — |
| `template prefer` | Sets the template used for a kind of produced document when none is named; workspace level unless `--user` | 2 when the template's kind does not fit the output, listing those that fit |
| `template preferences` | The template in effect for each kind of produced document and where the choice comes from | — |
| `template uses` | Documents created from it, outputs that prefer it, report definitions that name it | — |
| `template export` | Writes the named templates, completely, to one file | 5 when `--to` exists and `--yes` is not given; 6 when a template fails its check |
| `template import` | Shows what the file contains, checks each template, adds those that pass; `--dry-run` adds nothing | 2 when the file is damaged or not a template pack; 6 naming templates that fail their check (the others are added); 5 on a name conflict when it cannot ask |
| `template rm` | Deletes a template of the user's own; lists its uses first. Documents already created are unaffected | 5 `confirmation_required`; 5 for a provided template |

## Values

| Value | Rule |
|-------|------|
| `<name>` | 1–64 characters of `a-z 0-9 -`, starting with a letter; unique within a level ignoring case and accents |
| `<kind>` | `paper`, `thesis`, `report`, `proposal`, `protocol`, `letter`, `notes`, `other` |
| `<output>` | a kind of produced document: `activity-report`, `experiment-report`, `review-export`, `response-letter`, `reading-notes`, `notebook-export`, `progress-report`, `audit-report` |
| `--format` | the writing format of the documents the template creates; the formats offered are decided at planning time |
| precedence | for one name: `workspace` over `personal` over `provided` |
| `--template <name>` | added to every command that produces a document (`report`, `experiment report`, `review export`, `submission letter`, `read notes`, `notebook export`, `audit export`); ignored with a notice when `--output json` or a structured `--format` is requested |

## Inside a template

A template is a directory of text files and one description file. What the description
states — sections, questions, limits — and the marks used in the text are part of this
contract; their exact spelling is fixed at planning time and is the same in every writing
format.

| Element | Meaning |
|---------|---------|
| Placeholder | Replaced once, when the template is applied, by workspace information or an answer |
| Managed part | A region replaced at every `trcli doc refresh`; written between two comments of the writing format, so the document stays valid without the tool |
| Guidance | Advice to the writer; written as a marked comment so `trcli doc clean` can remove it |
| Repeat | A section written once per item of a list |
| Condition | A section written only when a piece of information exists or an answer has a given value |
| Include | Another template placed at this point |

A template cannot carry out commands, read files outside the workspace, or reach the
network; `template check` and `template import` refuse one that tries.

## Example

```console
$ trcli template list --kind paper
NAME              LEVEL      VERSION  DESCRIPTION
article           workspace  3        Group layout for journal articles      (hides provided:article)
conference-paper  provided   1        Conference paper with fixed page budget
review-article    provided   1        Narrative or systematic review article
short-paper       provided   1        Short paper or extended abstract

$ trcli template check thesis-ppgcc
thesis-ppgcc: 2 problems
  chapters/02-related.txt:14   unknown placeholder "draft.keywrods" (did you mean "draft.keywords"?)
  template description         question "defense_date" has default "soon", expected a date
$ echo $?
6
```
