## Assumptions

- **This specification owns venues.** User Story 12 of `specs/002-research-lifecycle`
  (FR-052 and FR-053: venues, calls, talks) is replaced by this one; that specification
  keeps a short pointer.
- **"Places to publish (events, conferences, journals)" is read as** venues that last
  (journals, conference series), their occurrences (events), and their invitations to
  submit (calls). Talks are included because they were specified with venues before and are
  what a researcher does at an event.
- **Submissions and peer review stay where they are**, in `specs/002-research-lifecycle`
  (its User Story 8). This specification reads them to show a venue's history; it does not
  redefine them. Manuscripts, their target venue and deadline, and their stages are those of
  `specs/008-manuscripts`.
- **The researcher records what they know; the tool looks nothing up.** It does not fetch
  calls for papers, deadlines, rankings, fees, or acceptance rates from anywhere, and ships
  no list of venues and no ranking scheme. Figures go out of date, which is why each is
  kept with the date it was noted.
- **Ranking schemes are whatever the researcher's context uses** — a national
  classification, a field's conference ranking, a journal quartile. The tool stores scheme
  name, value, and year; it does not know which values are better unless the researcher
  says, when recording a counting rule, what the minimum is.
- **Suggestions come only from the researcher's own register**, by overlap of topics and
  kind. The tool does not recommend venues it has not been told about and does not judge a
  venue's quality or legitimacy; "avoid" is the researcher's own mark.
- **Requirements are compared only where the workspace knows the answer**: kind, length,
  language, and anonymity of the manuscript. Everything else is shown for the researcher to
  check, and a venue's checklist can be kept with `specs/014-conventions-methods`.
- **The researcher's own figures are few.** Time to decision and acceptance record rest on
  their own submissions, often one or two; the tool says how many and never presents them
  as the venue's statistics.
- **Attending an event is recorded lightly**: state, practical dates, costs, notes. Booking
  travel, expense claims, and schedules of sessions are out of scope.
- **Other specifications are used, not redefined**: the view of what is due
  (`specs/003-research-projects`), templates (`specs/007-templates`), citation styles
  (`specs/005-literature`), grants (`specs/002-research-lifecycle`), the inbox and topics
  (`specs/013-ideas-questions`), the "private" marking (`specs/006-reports`), and links,
  tags, notes, and the audit trail (`specs/001-research-workspace`).
- **Sharing a list of venues is by file**, as elsewhere in TRCLI.
- **Single researcher**, as elsewhere.
