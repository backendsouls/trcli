# Contract: `trcli handbook`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 6; FR-040 to FR-046

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli handbook export --to <file> [--name <text>] [--origin <origin>]... [--category <text>]...
                      [--project <ref>] [--only <conventions|methods|checklists>]...
trcli handbook document --to <file> [same selection options]
trcli handbook import <file> [--on-change <ask|accept|keep-mine>] [--dry-run]
trcli handbook sources
trcli handbook changes <source-name>
trcli handbook start-here
trcli handbook methods-statement <manuscript-ref> [--to <file>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `handbook export` | Writes the selected conventions, methodologies, and checklists, completely, to one file. Private notes, people's contact details, and anything marked private are left out | 5 when the file exists and `--yes` is not given; 2 when the selection is empty |
| `handbook document` | A readable handbook organized by category, each entry with its origin, strength, rationale, and examples | as above |
| `handbook import` (first time) | Shows what the file contains and adds each entry with its origin and the handbook it came from, marked as received and unread | 2 when the file is damaged or is not a handbook — nothing is added |
| `handbook import` (newer version) | Shows what was added, changed, and retired since the version held, and applies changes on acceptance. Entries changed or retired locally are not overwritten without the researcher's choice | 5 when it cannot ask and `--on-change` is `ask`; 0 with "nothing new" when unchanged |
| `handbook sources` | The handbooks this workspace has received, with their version, date, and number of entries, and how many are unread or locally changed | — |
| `handbook changes` | What differs between the entries as received from a handbook and as they now are locally | — |
| `handbook start-here` | The entries a newcomer should read first: binding conventions and the methodologies in current use | — |
| `handbook methods-statement` | A draft text for a manuscript: the methodologies used with versions and sources, how each was adapted, the assumptions made with their status, and the departures from conventions, methods, and checklists | 0 with a notice naming anything the manuscript rests on that has no methodology recorded |

## Values

| Value | Rule |
|-------|------|
| `--name` | the handbook's name, shown to whoever brings it in; defaults to the workspace's name |
| `--on-change` | for entries changed both in the handbook and locally: `ask` (default on a terminal), `accept` the handbook's, `keep-mine` |
| received entries | keep the origin they were given by whoever exported them; a member who disagrees records a departure rather than editing the entry |
| conflicts between handbooks | entries from several handbooks that conflict are all kept and marked; precedence is as in [cli-applies.md](./cli-applies.md) |
| nothing is shipped | the tool comes with no handbook, convention, or guideline of its own |

## Example

```console
$ trcli handbook import lab-handbook-2026-10.trcli-handbook
"Speech Lab handbook" version 7 (you hold version 6)
  added     2   cnv "Transcription file naming" · mth "Speaker-stratified splits"
  changed   1   cnv "Dataset folder names" — strength should → must
  retired   1   cnv "Weekly backup to the shared disk"
  yours     1   mth "5-fold cross-validation" was changed locally — keeping your version
Apply these changes? [y/N] y
Applied. 3 entries are unread — see: trcli read --unread

$ trcli handbook methods-statement ms-5t1q
Methods
  Stratified 5-fold cross-validation (lab procedure, version 2), adapted from k-fold
  cross-validation [ref-5p8c]: folds are stratified by speaker; the test set is held out
  before folding.
Assumptions
  Annotators were independent of one another — checked, does NOT hold (see res-4c7j).
Departures
  Dataset "Pilot interviews" does not follow the lab's folder naming: the name is fixed by
  the partner's export tool.
```
