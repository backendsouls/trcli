## Assumptions

- **This specification owns people.** User Story 9 of `specs/001-research-workspace`
  (FR-048 to FR-050) is replaced by this one; that specification keeps a short pointer.
  "Staff member" there means a person here.
- **It is the minimum, on purpose.** The request asked for just what a lab head needs. Four
  things were added to the plain register — positions with dates, an overview, supervision
  with meetings, and handover — and nothing else. Anything a personnel system does
  (salaries, contracts, leave, evaluations, recruitment) is out of scope.
- **The workspace is the lab head's own.** As everywhere in TRCLI, one person uses a
  workspace. The people in it are records: they do not log in, see the workspace, or
  receive anything from it. A group sharing one workspace, with roles and permissions, is
  collaboration in `specs/002-research-lifecycle`.
- **A student uses the same features from the other side.** A single researcher's workspace
  has the same register — their supervisor, co-authors, and collaborators — and the same
  meeting records; the group views simply have little to show.
- **Assignments stay where they are defined.** Who authors a manuscript is specified in
  `specs/008-manuscripts`, who is responsible for an experiment or performs a step in
  `specs/004-experiments`, who owns a task or supervises a project in
  `specs/003-research-projects`. This specification adds membership of a project and
  gathers all of them per person; it does not redefine them.
- **Agreed actions are tasks** of `specs/003-research-projects`, and dates to watch appear
  in its view of what is due; a supervisee's "next requirement" comes from
  `specs/011-courses-roadmaps`; funding may refer to a grant of
  `specs/002-research-lifecycle`.
- **"Private" is the marking introduced in `specs/006-reports`**; this specification relies
  on it to keep notes about people out of reports and exports.
- **Personal data is kept to what running a lab needs**: name, affiliation, contact,
  identifier, dates. The tool stores no identity documents, addresses, or health or
  financial details, and the lab head remains responsible for handling what they record
  according to their institution's rules.
- **Workload is counted, not measured.** The overview counts active and overdue items; it
  does not estimate hours or judge performance.
- **Meetings are notes, not scheduling.** The tool records meetings that happened; it does
  not send invitations or reminders.
