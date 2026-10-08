### Edge Cases

- A day has 23 or 25 hours because clocks changed: the daily report covers the whole
  calendar day, whatever its length.
- Activity happens just before midnight: it belongs to the day on which it happened in the
  researcher's local time, and to that day's week and month.
- The researcher travels and their local time changes: each event keeps the moment it
  happened; reports use the local time in effect when the report is produced, and say which.
- A week spans two months or two years: it appears whole in the weekly report, and each of
  its days is counted in its own month and year.
- The first day of the week or the first month of the year is changed: later reports use the
  new setting; saved reports keep the dates they were saved with.
- A period name is ambiguous ("bimonthly"): the tool always means every two months, and the
  report header states the exact dates.
- A record is changed many times in a period: at summary and standard detail it is counted
  once as "updated"; full detail lists every change.
- A record is created and deleted within the same period: both events appear; it is not
  counted among records existing at the end.
- A record moves from one project to another during the period: its activity is reported
  under the project it belonged to when each event happened.
- A task is completed, reopened, and completed again: the report shows it completed once,
  with the latest date, and full detail shows the history.
- A run started in one period and ended in the next: it is reported as started in the first
  and as finished in the second.
- A very long or very active period would produce thousands of items: the report summarizes,
  states how many items were not listed, and full detail remains available on request.
- A report is requested for a period entirely in the future: the tool refuses, and offers
  the view of what is due instead.
- A report is requested for a period entirely before the workspace existed: the tool says
  there is nothing to report and gives the workspace's first day.
- The record of activity has been found altered (the audit check fails): the report is still
  produced and carries a visible warning that its source could not be verified.
- An area of work is not in use at all (no experiments have ever been recorded): its section
  never appears and it is not listed as "quiet".
- Two saved reports cover overlapping periods: both are kept; neither replaces the other.
- A report is exported to a place that is not writable, or the disk is full: nothing partial
  is left behind, and the tool explains.
