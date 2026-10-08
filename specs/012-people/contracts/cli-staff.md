# Contract: `trcli staff` (alias: `person`)

**Spec**: [People and Lab Management](../spec.md) — User Stories 1 and 2; FR-001 to FR-018

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `stf`

`trcli person …` is accepted everywhere `trcli staff …` is.

## Synopsis

```text
trcli staff add --name "<Family>, <Given>" [--position <text>] [--collaborator]
                [--affiliation <text>] [--email <address>] [--orcid <id>] [--web <address>]
                [--from <date>] [--until <date>] [--funding <text|grant-ref>] [--funding-until <date>]
trcli staff position <ref> <text> --from <date> [--until <date>]
trcli staff funding <ref> <text|grant-ref> [--until <date>] | --clear
trcli staff me <ref>
trcli staff leaving [--within <span>]
trcli staff private <ref> <text>
trcli staff forget <ref>

trcli staff assign <ref> <project-ref> [--role <text>] [--remove]
trcli staff assignments <ref> [--history]
trcli staff due <ref> [--within <span>]
trcli staff overview [--to <file>]
trcli staff unowned
```

The shared verbs `list`, `show`, `edit`, `tag`, and `note` apply and are not repeated.
`rm` is replaced by `forget` (see below).

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `staff add` | Adds a person as a current member, or with `--collaborator` as an outside collaborator. Reports a likely duplicate (same name or researcher identifier) and asks whether to add | 2 on a missing name, an invalid address or identifier, or `--until` before `--from` — all reported together; 5 on a duplicate when it cannot ask |
| `staff position` | Records a new position from a date; the earlier one is kept in the person's history | 2 when the dates are out of order |
| `staff funding` | Records what funds the person's stay and until when | 3 when a grant is not found |
| `staff me` | States which person is the researcher using this workspace; "my" views refer to them | 3 when not found |
| `staff list` | Shared verb. Current members by default, with position, joining date, expected end, and funding end; a member past their expected end is marked | — |
| `staff leaving` | Members whose stay or funding ends within the span, in date order; these dates also appear in `trcli due` | 2 on an invalid span |
| `staff private` | Adds a note about a person that never leaves the workspace | 2 on empty text |
| `staff forget` | Removes a person: explains that their name remains where the record of the research needs it, removes contact details and private notes | 5 `confirmation_required`; 5 `blocked_by_dependents` while they hold open items — run a handover first |
| `staff assign` | Adds a person to a project with a role, or removes them; visible from both | 3 when the project is not found; 5 for a former member without confirmation |
| `staff assignments` | Everything a person is on across the workspace — projects, manuscripts (with author position), experiments, manual steps, tasks, datasets, supervisees — with status and next date | — |
| `staff due` | What is due for one person within a span | — |
| `staff overview` | Each current member with active items by kind, next deadline, and overdue items; marks members with no active assignment or too many overdue. `--to` writes a document without private notes | — |
| `staff unowned` | Active projects, experiments, and manuscripts with nobody responsible | 6 `check_failed` when any |

## Values

| Value | Rule |
|-------|------|
| `--name` | family name 1–100 characters; given name optional |
| `--position` | free text, 1–100 characters; suggested: `group leader`, `faculty`, `postdoc`, `doctoral student`, `master's student`, `undergraduate student`, `technician`, `visitor`; default `member` |
| `--email` | one `@`, a domain with a dot |
| `--orcid` | `0000-0000-0000-000X` with a valid check digit; unique |
| relation | `member`, `collaborator`, `former`; list filters `--members` (default), `--collaborators`, `--former`, `--all` |
| list filters | `--position`, `--funding`, `--supervisor <ref>`, `--search` |
| settings | `people.overdue_alert` (default `3` overdue items), `people.leaving_within` (default `6m`) |
| private notes and contact details | never in exports, reports, or documents for others; contact details only with `--with-contact` |

## Example

```console
$ trcli staff overview
PERSON              POSITION          PROJECTS  MANUSCRIPTS  EXPERIMENTS  TASKS  NEXT DEADLINE        OVERDUE
Costa, Bruno        doctoral student         1            2            3      7  2026-10-12 task            0
Lima, Carla         postdoc                  2            3            1      4  2026-10-20 milestone       1
Rocha, Davi         master's student         1            0            0      0  —                          0  ⚠ nothing active
Silva, Ana          doctoral student         1            1            2      9  2026-10-09 task            4  ⚠ 4 overdue

$ trcli staff leaving --within 6m
2026-12-15  Rocha, Davi    scholarship ends       (expected to leave 2027-02-28)
2027-02-28  Rocha, Davi    expected to leave
2027-03-31  Lima, Carla    expected to leave
```
