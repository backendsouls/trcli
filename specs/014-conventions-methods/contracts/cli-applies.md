# Contract: `trcli applies`, `trcli departure`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 3; FR-018 to FR-025

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `dpt` (departure)

## Synopsis

```text
trcli applies <record-ref>
trcli applies --kind <record-kind> [--project <ref>] [--topic <topic>]
trcli applies conflicts

trcli departure add <work-ref> --from <convention-ref | method-ref[#<step>] | checklist-ref#<item>>
                    --because <text> [--instead <text>] [--approved-by <person>]
trcli departure list [<work-ref>] [--project <ref>] [--from <ref>] [--origin <origin>]
trcli departure show <ref> | edit <ref> [fields] | rm <ref>
trcli departure frequent [--limit <n>]

trcli read <entry-ref>...
trcli read --unread [--origin <origin>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `applies <record>` | Everything that concerns one record, together: conventions for its kind, project, and topics; the methodology it follows and its version; the assumptions under it; the decisions that concern it; the checklists applied to it. Binding entries first, each with its origin | 3 when the record is not found |
| `applies --kind` | The conventions for a kind of record, before one is created | 2 on unknown kind |
| `applies conflicts` | Conventions that apply to the same thing and say different things, with which takes precedence and why; those the rule cannot order are marked unresolved | 6 `check_failed` when any is unresolved |
| (on `add` of any record) | When conventions apply to the kind being created, one line says how many and how to see them. It asks nothing and never fails the command | — |
| `departure add` | Records that a piece of work knowingly does not follow a convention, a methodology step, or a checklist item, with the reason and what was done instead | 2 without `--because`; 2 when what is named does not apply to the work; 3 when something is not found |
| `departure add` (institution, venue, or funder origin) | Asks who approved it | 0 with a warning when nobody is named; 5 when it cannot ask and `--approved-by` is absent |
| `departure list` | Departures of a piece of work, a project, or the workspace, each with what was departed from, the reason, and the date. Departures from retired or changed entries stay, marked | — |
| `departure frequent` | The conventions most often departed from — candidates for changing | — |
| `read` | Marks entries as read by the researcher, with the date | 3 when not found |
| `read --unread` | Entries adopted, received, or changed since the researcher last read them | 6 when any binding entry is unread |

## Precedence between conventions that conflict

| Order | Origin | Reason |
|-------|--------|--------|
| 1 | venue, funder, institution | the researcher cannot waive them |
| 2 | lab | agreed by the group |
| 3 | community | the field's practice |
| 4 | literature, own | chosen by the researcher |

Within one origin: `must` over `should` over `may`. A conflict between entries of the same
origin and strength is unresolved and is the researcher's to settle.

## Values

| Value | Rule |
|-------|------|
| `<work-ref>` | any record: a dataset, an experiment, a run, a manuscript, a review, a model, … |
| `--from` | a convention; a methodology, optionally with `#<step number>`; or a checklist with `#<item number>` |
| `<entry-ref>` | a convention, methodology, or checklist |
| settings | `conventions.mention_on_add` (default `true`) turns the one-line mention on or off |
| the tool does not enforce | it cannot tell whether a convention was followed; it shows what applies and records departures |

## Example

```console
$ trcli dataset add --name "Pilot interviews" --location data/pilot_final
Added dat-5v2n "Pilot interviews" · version 1
note: 2 conventions apply to datasets (1 must) — see: trcli applies dat-5v2n

$ trcli applies dat-5v2n
Conventions
  must    cnv-4r8w  Dataset folder names (lab)        <year>-<source>-<version>, all lowercase
  should  cnv-7t1y  Keep a README in every dataset (own)
Methodology   —
Assumptions   —
Checklists    —

$ trcli departure add dat-5v2n --from cnv-4r8w \
    --because "Folder name is fixed by the partner's export tool" --instead "Kept the partner's name; noted the year in the description"
Recorded dpt-2e6m: dat-5v2n departs from cnv-4r8w "Dataset folder names"
```
