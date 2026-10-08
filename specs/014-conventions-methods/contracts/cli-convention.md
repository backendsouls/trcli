# Contract: `trcli convention`

**Spec**: [Conventions, Methodologies, and the Implicit Side of Research](../spec.md) — User Story 1; FR-001 to FR-010

**Conventions of the CLI itself**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `cnv`

## Synopsis

```text
trcli convention add <title> --says <text> [--why <text>] [--origin <origin>] [--source <text|ref>]
                     [--strength <must|should|may>] [--category <text>] [--proposed]
                     [--for-kind <record-kind>]... [--for-project <ref>]... [--for-topic <topic>]...
                     [--when <text>] [--do <text>]... [--dont <text>]...
trcli convention example <ref> (--do | --dont) <text>
trcli convention adopt <ref> [--by <person>]
trcli convention reject <ref> --because <text> [--by <person>]
trcli convention retire <ref> --because <text> [--replaced-by <ref>]
trcli convention history <ref>
trcli convention incomplete
trcli convention from <thought|lesson-ref>
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `convention add` | Records a convention. Without options it is `adopted`, of origin `own`, strength `should`. Shows conventions of similar wording | 2 on a missing statement, a title already used, or an unknown origin, strength, or record kind — all reported together |
| `convention add --proposed` | Records it as under discussion; shown apart from adopted ones | — |
| `convention example` | Adds an example of following (`--do`) or not following (`--dont`) it | 2 on empty text |
| `convention adopt` / `reject` | Decides a proposed convention; records who decided and when | 2 when it is not proposed; 2 for `reject` without `--because` |
| `convention retire` | Retires a convention with a reason, optionally naming its replacement; it stays viewable. Departures from it are kept, marked | 2 without `--because` |
| `convention history` | Earlier wordings, changes of status, and who decided, with dates | — |
| `convention incomplete` | Entries worth completing: binding conventions without a rationale, entries whose source is unknown | 0 with a message when none |
| `convention from` | Turns a captured thought or a lesson into a convention | 3 when not found |

## Values

| Value | Rule |
|-------|------|
| `<title>` | 1–200 characters, unique in the workspace |
| `--says` | the statement; 1–20,000 characters, any language |
| `<origin>` | `own` (default), `lab`, `institution`, `community`, `venue`, `funder`, `literature` |
| `--source` | where it comes from: a reference from the library, a person, a document, an organization, or `unknown` |
| `--strength` | `must`, `should` (default), `may` |
| `--category` | suggested: `naming`, `files`, `notation`, `writing`, `data`, `analysis`, `authorship`, `reviewing`, `meetings`; any other text is accepted |
| `--for-kind`, `--for-project`, `--for-topic`, `--when` | what it applies to; with none, it applies everywhere and the tool suggests narrowing it |
| status | `proposed`, `adopted`, `rejected`, `retired` |
| list filters | `--origin`, `--strength`, `--category`, `--status`, `--for-kind`, `--for-project`, `--for-topic`, `--unread`, `--search` |

## Example

```console
$ trcli convention add "Dataset folder names" \
    --says "Dataset folders are named <year>-<source>-<version>, all lowercase." \
    --why "So versions sort correctly and the source is visible without opening anything." \
    --origin lab --strength must --category naming --for-kind dataset \
    --do "2026-interviews-v2" --dont "Interviews final (new)"
Added cnv-4r8w "Dataset folder names" (lab · must · applies to datasets)

$ trcli convention list --origin community
TITLE                               STRENGTH  CATEGORY   SOURCE
Report effect sizes with intervals  should    analysis   ref-3h7q
Significance threshold 0.05         should    analysis   unknown  ⚠ source unknown
```
