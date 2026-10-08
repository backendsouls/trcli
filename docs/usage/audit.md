# Audit trail and telemetry: `trcli audit`, `trcli telemetry`

## Purpose

Everything that changes in a workspace is written down as it happens: what was done, to
what, by whom, when, and what exactly changed. This is the **audit trail**. You — or a
supervisor, a reviewer, an auditor — can look through it, check that nobody has tampered
with it, and hand over a report of it. There is no command that changes or removes an
entry.

Separately, the tool keeps simple **telemetry** about its own use, for your benefit: which
commands ran, how long they took, whether they worked. It stays in the workspace, is never
sent anywhere, never holds what you typed, and can be turned off.

## Commands

| Command | What it does |
|---------|--------------|
| `trcli audit list [filters] [--limit <n>]` | Entries that match every filter given, newest first |
| `trcli audit verify` | Checks that the trail was not altered or shortened outside the tool |
| `trcli audit export [filters] --to <file> [--format markdown\|json\|csv]` | Writes the matching entries as a report |
| `trcli telemetry show` | Commands used, how often, how they ended, how long they took |
| `trcli telemetry on` / `off` / `status` | Turns local telemetry on or off, or says which it is |

| Filter | Meaning |
|--------|---------|
| `--record <ref>` | Only entries about this record |
| `--kind <kind>` | Only entries about records of this kind |
| `--actor <name>` | Only entries made by this actor |
| `--action <action>` | `create`, `update`, `delete`, `status`, `link`, `unlink`, `tag`, `untag`, `note`, `import`, `export`, `run`, `confirm`, `setting`, `upgrade` |
| `--from <date>` | On or after this day (`YYYY-MM-DD`, UTC) |
| `--to <date>` (in `audit list`; also `--until`) | On or before this day |
| `--until <date>` (in `audit export`, where `--to` names the file) | On or before this day |

### What an entry holds

Its position in the trail; when; who acted — the name set with `trcli workspace edit
--researcher`, otherwise your name on the machine; the action; the record's short name and
what it was called at the time (kept even after the record is deleted); and each field
that changed, with its value before and after.

### How tampering is detected, and the limit of that

Every entry carries a hash of its own content chained to the entry before it, and the end
of the trail is also kept in `.trcli/audit.head`, outside the database. `trcli audit
verify` recomputes the chain: an entry altered or removed in the middle, and entries
removed from the end, are reported with where. Someone who rewrites the whole chain *and*
the head file consistently is not detected; guarding against that needs a copy kept
elsewhere.

## Examples

```console
$ trcli init --name "Field work"
Created workspace "Field work" in [..]/work/.trcli
$ trcli specimen add --title "Soil sample 14"
Saved specimen spc-z7kj "Soil sample 14"
$ trcli specimen edit spc --title "Soil sample 14 (dry)"
Saved specimen spc-z7kj "Soil sample 14 (dry)"
$ trcli audit list
SEQ  WHEN              ACTOR  ACTION  WHAT
3    2026-10-08 14:00  ana    update  spc-z7kj "Soil sample 14 (dry)" title: So…
2    2026-10-08 14:00  ana    create  spc-z7kj "Soil sample 14" title: (none) →…
1    2026-10-08 14:00  ana    create  "Field work" name: (none) → Field work
$ trcli audit list --action update --output json
{
  "data": {
    "items": [
      {
        "action": "update",
        "actor": "ana",
        "at": "2026-10-08T14:00:00Z",
        "changes": [
          {
            "after": "Soil sample 14 (dry)",
            "before": "Soil sample 14",
            "field": "title"
          }
        ],
        "display_name": "Soil sample 14 (dry)",
        "handle": "spc-z7kj",
        "hash": "102e151e4fbc594ff437436df1b94ac00a4e260daa00342085e26aff5e5212a5",
        "kind": "specimen",
        "record_id": "01920000-0000-7000-8f9e-72ebf533e025",
        "sequence": 3
      }
    ],
    "total": 1
  },
  "ok": true,
  "warnings": []
}
$ trcli audit verify
✓ The audit trail is intact: 3 entries verified.
```

Hand over a report:

```console
$ trcli audit export --to audit.md
Exported 3 audit entries to audit.md (markdown)
$ trcli audit export --action create --from 2026-10-01 --until 2026-10-31 --to created.csv --format csv
Exported 2 audit entries to created.csv (csv)
```

Telemetry:

```console
$ trcli telemetry off
Set telemetry.enabled = false in this workspace
$ trcli telemetry status
Telemetry is off. It is kept in this workspace only and is never sent anywhere.
$ trcli telemetry on
Set telemetry.enabled = true in this workspace
```

## When it fails

Filters are checked together, and a rule between two of them is reported with the rule
(exit code 2):

```console
$ trcli audit list --from 2026-10-08 --to 2026-10-01 --limit 0
error: 2 values are invalid
  --limit "0"        is out of range; expected a whole number from 1 to 1000 (for example: 1)
  --to "2026-10-01"  is before the start (2026-10-08); expected an end on or after the start
Nothing was changed.
[exit 2]
```

An export does not write over a file unless you say so (exit code 5):

```console
$ trcli audit export --to audit.md --no-input
error: Writing over audit.md needs confirmation, and nobody can be asked
  audit.md
Nothing was changed.
Next: run the command again with `--yes` to confirm beforehand
[exit 5]
```

| What happened | Exit code | What the tool says and what to do |
|---------------|-----------|-----------------------------------|
| An invalid filter | 2 | Every invalid filter, with what is expected |
| `audit verify` finds an entry altered, missing, or removed from the end | 6 | Which entry and which check failed; compare with a backup |
| `.trcli/audit.head` is missing while there are entries | 6 | That a removal at the end cannot be ruled out |
| The export's file exists | 5 | Run again with `--yes` to write over it |
| The export's file cannot be written | 7 | Why; nothing is recorded |
