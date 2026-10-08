#### Periods

- **FR-001**: Users MUST be able to request a report for these periods: daily, weekly,
  monthly, bimonthly (two months), quarterly, semiannual (six months), annual, and custom.
- **FR-002**: For every period other than custom, users MUST be able to choose the current
  period (to now), the previous complete period, the period containing a given date, or a
  period named by its number and year.
- **FR-003**: A custom period MUST accept a start date and an end date (both included), a
  start date alone (to now), or a span of the last N days, weeks, or months.
- **FR-004**: Weeks MUST begin on a configurable first day (Monday by default). Bimonthly,
  quarterly, semiannual, and annual periods MUST be counted from a configurable first month
  of the year (January by default).
- **FR-005**: The system MUST accept "semester" and "half-year" as other names for
  semiannual, and MUST treat "bimonthly" as every two months in all cases.
- **FR-006**: Every report MUST state the exact first and last date it covers, the local
  time zone used, the moment it was produced, and whether the period is complete or partial.
- **FR-007**: The system MUST reject a period whose end is before its start, that lies
  entirely in the future, or whose span is not a positive number, and MUST limit a period
  that begins before the workspace existed to the workspace's first day, saying so.
- **FR-008**: Period boundaries MUST follow calendar days in the researcher's local time,
  including on days when clocks change.
