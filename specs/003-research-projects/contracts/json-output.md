# Contract: Structured Output of the Project Commands

**Plan**: [../plan.md](../plan.md) | **Data model**: [../data-model.md](../data-model.md) | **General rules**: [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

What `--output json` prints for the commands of this specification. The envelope
(`ok`, `data`, `warnings`, `error`) and its rules are defined once in the general contract;
this file gives the shape of `data`. Field names match the data model. Absent optional
values are `null`, never omitted. Dates are `YYYY-MM-DD`; timestamps are RFC 3339 in UTC.

Changing a field's name or meaning after release is a breaking change; adding a field is
not.

## Project

```json
{
  "kind": "project",
  "id": "0199a3f2-…", "handle": "prj-1d4c",
  "title": "Doctorate",
  "type": { "key": "doctoral", "name": "Doctoral (PhD)", "is_degree": true },
  "goal": null,
  "status": "active",
  "period": { "starts_on": "2026-03-01", "ends_on": "2030-02-28" },
  "academic": {
    "institution": "…", "programme": "…", "degree_sought": "…",
    "supervisor": { "handle": "stf-9c2e", "name": "Lima, Carla" },
    "co_supervisors": [], "committee": []
  },
  "funder": null, "grant_reference": null, "extra_details": {},
  "closing_note": null,
  "is_current": true,
  "read_only": false,
  "created_at": "2026-10-08T14:02:11Z", "updated_at": "2026-10-08T14:02:11Z"
}
```

`project summary` adds `"counts": { "milestone": 6, "task": 11, "reference": 240, … }`.
`project list` returns `{ "items": [ Project… ], "total": n }`.

## Milestone

```json
{
  "kind": "milestone",
  "id": "…", "handle": "mil-7c3k",
  "project": { "handle": "prj-1d4c", "title": "Doctorate" },
  "title": "Qualifying exam",
  "description": null,
  "target_on": "2028-02-29", "date_confirmed": true,
  "status": "upcoming",
  "reached_on": null,
  "required_by_programme": true,
  "deliverable": null,
  "overdue_days": null,
  "days_remaining": 144,
  "delay_days": null,
  "tasks": { "done": 2, "total": 5 },
  "date_changes": [ { "from": "2027-12-15", "to": "2028-02-29", "reason": "…", "changed_at": "…" } ]
}
```

## Task

```json
{
  "kind": "task",
  "id": "…", "handle": "tsk-5w0h", "number": 15,
  "project": { "handle": "prj-1d4c", "title": "Doctorate" },
  "milestone": { "handle": "mil-7c3k", "title": "Qualifying exam" },
  "parent": null,
  "title": "Rerun baseline with new split",
  "description": null,
  "status": "in_progress",
  "priority": "normal",
  "starred": true,
  "due_on": "2026-10-10", "overdue_days": null, "days_remaining": 2,
  "effort": null,
  "responsible": { "handle": "stf-4a9b", "name": "Silva, Ana" },
  "blocked_by": [],
  "recurrence": null,
  "completed_on": null,
  "subtasks": { "done": 0, "total": 0 },
  "concerns": [ { "kind": "experiment", "handle": "exp-2b6r", "name": "Baseline" } ],
  "has_notes": false
}
```

`number` is the task's short number **in this copy of the workspace**; it is not the same
on another machine. Use `handle` to refer to a task from outside.

## What is due — `trcli due`

```json
{
  "period": { "from": "2026-10-08", "to": "2026-10-22" },
  "scope": "all-projects",
  "items": [
    {
      "when": "2026-10-05", "overdue": true, "days": -3,
      "kind": "task",
      "record": { "handle": "tsk-3r8j", "title": "Send abstract" },
      "project": { "handle": "prj-1d4c", "title": "Doctorate" }
    }
  ],
  "next_after_period": null
}
```

`kind` is `task` or `milestone` here; other specifications add kinds of their own
(`grant`, `call`, `requirement`, …) without changing this shape.

## Progress — `trcli progress`

```json
{
  "project": { "handle": "prj-1d4c", "title": "Doctorate" },
  "milestones": { "reached": 1, "total": 6 },
  "tasks": { "done": 5, "total": 11 },
  "time_elapsed_percent": 15,
  "next_milestone": { "handle": "mil-7c3k", "title": "Qualifying exam", "target_on": "2028-02-29", "days_remaining": 144 },
  "behind": false,
  "overdue_milestones": []
}
```

`time_elapsed_percent` is `null` when the project has no full period.

## To-do list — `trcli todo`

The same content as the text form, nested, with no decoration (FR-071).

```json
{
  "view": "board",
  "scope": "all-projects",
  "boards": [
    {
      "project": { "handle": "prj-1d4c", "title": "Doctorate", "status": "active" },
      "done": 5, "total": 11, "complete": false,
      "sections": [
        {
          "milestone": {
            "handle": "mil-7c3k", "title": "Qualifying exam", "target_on": "2028-02-29",
            "status": "upcoming", "required_by_programme": true,
            "days_remaining": 144, "overdue_days": null
          },
          "done": 2, "total": 5,
          "lines": [
            {
              "number": 16, "handle": "tsk-8j2m",
              "title": "Write the related-work chapter",
              "status": "to_do", "priority": "urgent", "starred": false,
              "due_on": "2026-10-09", "days_remaining": 1, "overdue_days": null,
              "blocked_by": [], "repeats": false,
              "responsible": null, "has_notes": false,
              "children": [
                { "number": 17, "handle": "tsk-1q4z", "title": "Summarize the 2024 survey", "status": "to_do", "children": [] }
              ]
            }
          ],
          "more": 0
        },
        { "milestone": null, "done": 2, "total": 3, "lines": [ ], "more": 0 }
      ]
    }
  ],
  "summary": {
    "percent_done": 57,
    "done": 8, "in_progress": 1, "pending": 4, "blocked": 1,
    "hidden": 3
  }
}
```

- A section with `"milestone": null` is "No milestone".
- `more` is the number of lines left out of a long section.
- Child lines carry every field of a line; the example shortens them.
- With `--timeline`, `"view": "timeline"` and `boards` is replaced by
  `"days": [ { "label": "overdue" | "today" | "tomorrow" | "this_week" | "later" | "no_date", "lines": [ … ] } ]`,
  each line also carrying its `project` and `milestone`.

## Acting on several tasks — `trcli todo done 12 15 99`

```json
{
  "outcomes": [
    { "number": 12, "handle": "tsk-2a7c", "result": "changed", "status": "done" },
    { "number": 15, "handle": "tsk-5w0h", "result": "unchanged", "status": "done" },
    { "number": 99, "handle": null, "result": "not_found", "status": null }
  ],
  "offers": [
    { "kind": "reach_milestone", "record": { "handle": "mil-7c3k", "title": "Qualifying exam" } }
  ]
}
```

`result` is `changed`, `unchanged`, or `not_found`. `offers` lists what the tool would have
asked on a terminal — complete a parent, reach a milestone — and did not do; with
`--output json` nothing is asked and nothing offered is done unless `--yes` is given. The
exit code is 3 only when every number was `not_found`.
