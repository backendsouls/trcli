### User Story 2 - Report on longer and custom periods (Priority: P2)

A researcher asks for a bimonthly, quarterly, semiannual, or annual report, or for any
range of dates they choose — "from the 3rd of March to the 17th of April", "the last 10
days", "since my last meeting". Long reports are summarized more heavily so they stay
readable.

**Why this priority**: These are the periods institutions and funders ask for. They reuse
everything in the first story and add the rules for where periods begin and how much detail
a long report shows.

**Independent Test**: Request a semiannual report and confirm its dates; set the first
month of the year to August and confirm the dates move; request a custom range and a "last
10 days" report and confirm their dates and content.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher requests a bimonthly, quarterly,
   semiannual, or annual report, **Then** it covers the current period of that length,
   counted from the first month of the researcher's year.
2. **Given** the first month of the year is set to August, **When** the researcher requests
   the semiannual report in October, **Then** it covers August to January.
3. **Given** any of these periods, **When** the researcher asks for the previous one, or
   names a date, or names the period by its number and year ("second semiannual period of
   2026"), **Then** the report covers that period.
4. **Given** a start date and an end date, **When** the researcher requests a custom
   report, **Then** it covers both dates and every day between them.
5. **Given** a number of days, weeks, or months, **When** the researcher requests a report
   for "the last N", **Then** it covers that span ending today.
6. **Given** only a start date, **When** the researcher requests a custom report, **Then**
   it covers from that date to now.
7. **Given** a report longer than one month, **When** it is shown, **Then** activity is
   summarized by month within the period, and individual items are listed only for
   significant events (milestones reached, drafts changing stage, experiments concluded,
   reviews completed) unless the researcher asks for full detail.
8. **Given** an end date before the start date, a start date in the future, or a span of
   zero or a negative number, **When** the researcher requests the report, **Then** the
   tool rejects it and explains.
9. **Given** a period that begins before the workspace existed, **When** the report is
   produced, **Then** it covers from the workspace's first day and says so.
10. **Given** a period that has not ended, **When** the report is produced, **Then** it is
    marked as partial, with the date it was produced.
11. **Given** the word "semester" or "half-year" is used for the period, **When** the
    researcher requests the report, **Then** it is understood as semiannual.

---
