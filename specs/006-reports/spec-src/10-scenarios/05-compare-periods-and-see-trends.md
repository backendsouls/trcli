### User Story 5 - Compare periods and see trends (Priority: P5)

A researcher sees how a period compares with the one before it — more references read,
fewer tasks completed — and looks at a series of periods side by side, for example the last
twelve weeks, to see how their work is trending.

**Why this priority**: Comparison turns a report into feedback. It needs nothing new except
the reports themselves, and is valuable only once they are in regular use.

**Independent Test**: With activity in two consecutive weeks, produce the weekly report
with comparison and confirm the differences are correct; produce a series of the last four
weeks and confirm each column matches that week's own report.

**Acceptance Scenarios**:

1. **Given** a report, **When** the researcher asks for comparison, **Then** each count in
   the summary is shown beside the count for the preceding period of the same length, with
   the difference.
2. **Given** a current period that has not ended, **When** it is compared, **Then** the
   comparison is with the same portion of the preceding period, and says so.
3. **Given** two periods named by the researcher, **When** they ask to compare them,
   **Then** their summaries are shown side by side.
4. **Given** a period length and a number, **When** the researcher asks for a series,
   **Then** one column per period is shown, for the chosen counts, oldest first.
5. **Given** a series, **When** it is shown, **Then** each column matches exactly what that
   period's own report shows.
6. **Given** a preceding period with no activity, **When** it is compared, **Then** the
   difference is shown as a count, not as a percentage of zero.
7. **Given** a series longer than the workspace's life, **When** it is shown, **Then**
   periods before the workspace existed are marked as not applicable rather than zero.

---
