#### Templates for produced documents

- **FR-032**: Every document the system produces — activity reports, experiment reports,
  exported literature reviews, response letters, reading notes, notebook exports, progress
  reports, and audit reports — MUST be laid out by a template, and users MUST be able to
  name the template to use.
- **FR-033**: Users MUST be able to set a preferred template per kind of produced document,
  for themselves and per workspace; without one, the provided template MUST be used and
  MUST give the layout defined by the specification that owns the document.
- **FR-034**: The system MUST refuse a template whose kind does not fit the document being
  produced and list those that fit.
- **FR-035**: A template MUST NOT change what a produced document is allowed to contain: it
  MUST NOT place content the document's audience or scope withholds, and when it leaves out
  content the document has, the system MUST say which sections were not placed.
- **FR-036**: Users MUST be able to set the template of a named report definition; a saved
  report MUST keep the layout it was saved with.
- **FR-037**: Templates MUST apply only to documents; for output in a structured form meant
  for other programs, a named template MUST be ignored with a notice.
