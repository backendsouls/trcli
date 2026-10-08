#### Input validation

- **FR-020**: The system MUST validate every value a user supplies — typed, answered to a
  question, read from a file, or brought in from another source — before anything is stored
  or acted upon.
- **FR-021**: Validation MUST cover the presence of required values, form, length, allowed
  values, numeric and date ranges, rules between values, and whether records and files that
  are named exist.
- **FR-022**: When input is invalid, the system MUST change nothing, MUST name each invalid
  value, and MUST say what is wrong and what is expected, with an example of a valid value
  where a form is required and the list of choices where a set is.
- **FR-023**: When several values are invalid, the system MUST report all of them together.
- **FR-024**: An invalid command — an unknown action or option, a missing required value —
  MUST be reported with how the command is used and, where one exists, the nearest valid
  command or option.
- **FR-025**: The system MUST NOT silently change a value a user supplied, beyond stated
  normalization rules; something unusual but allowed MUST be met with a warning, or with a
  question where the owning specification says so.
- **FR-026**: Free text MUST be accepted in any language and script and kept exactly as
  written; control characters other than line breaks and tabs MUST be rejected.
- **FR-027**: Invalid input MUST end the command in a way a calling program can tell apart
  from success and from every other kind of failure.
