## Assumptions

- **"Semiannual" is the name chosen for "half-year".** "Semester" and "half-year" are
  accepted as other words for it. The name can be changed without changing behavior.
- **"Bimonthly" means every two months.** The word also means twice a month in everyday
  use; this specification never uses it that way, and every report states its exact dates.
- **Quarterly and annual were added** because institutions and funders commonly ask for
  them and they follow the same rules; they can be dropped without affecting the others.
- **Reports are read from the audit trail** specified in `specs/001-research-workspace`,
  which records every change with its time. A report therefore covers exactly what the
  workspace recorded; work done outside the tool appears only if the researcher writes it
  into the narrative or the lab notebook.
- **Sections follow the other specifications**: literature (`specs/005-literature`),
  experiments (`specs/004-experiments`), projects, milestones, and tasks
  (`specs/003-research-projects`), and the rest (`specs/001-research-workspace`,
  `specs/002-research-lifecycle`). A section exists only once the records it reports on
  exist; reports work with whichever of those are available.
- **This specification supersedes the per-project progress report** of
  `specs/003-research-projects` (its FR-044): that report is the supervisor-audience report
  scoped to one project. The "what is due" view of that specification is reused here, not
  redefined.
- **No time tracking.** Reports say what happened and when, not how many hours were spent.
- **No scheduling or delivery.** The tool produces a report when asked; it does not run by
  itself at the end of a period, send messages, or remind anyone. The researcher sends the
  exported document.
- **Privacy markings**: "private" is introduced by this specification and can be set on any
  note or record; "sensitive" is the marking on datasets defined in
  `specs/002-research-lifecycle`. Unmarked items are never withheld.
- **Significant events** for longer reports are: milestones reached or missed, drafts
  changing stage, submissions and decisions, experiments concluded, hypotheses changing
  status, reviews completed, and datasets or software published.
- **Dates use the researcher's local time zone** at the moment a report is produced; a
  report states the zone used.
- **Single researcher.** A report describes the workspace's activity; attributing activity
  to several collaborators arrives with collaboration in `specs/002-research-lifecycle`.
- **The exported document is plain and portable**, readable anywhere and convertible by the
  researcher's own tools; producing typeset or branded documents is out of scope.
