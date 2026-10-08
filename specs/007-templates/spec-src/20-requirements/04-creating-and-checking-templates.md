#### Creating and checking templates

- **FR-024**: Users MUST be able to create a template from nothing or by copying any
  template under a new name, and to update and delete their own templates.
- **FR-025**: Users MUST be able to add, remove, reorder, and rename a template's sections,
  mark each required or optional, write guidance for each, and state limits (words in a
  section, words overall, number of references, number of figures and tables).
- **FR-026**: Guidance MUST appear in created documents marked so that it is
  distinguishable from the document's text and removable in one step.
- **FR-027**: Users MUST be able to declare the information a template asks for: a name, a
  question, a kind of value (text, number, date, yes/no, one of a list), whether it is
  required, and a default.
- **FR-028**: Users MUST be able to check a template; the check MUST report every unknown
  placeholder, duplicated or unnamed section, undeclared or duplicated question, default of
  the wrong kind, invalid limit, missing or looping inclusion, and unbalanced repeat or
  condition, each with its place and what is expected.
- **FR-029**: The system MUST refuse to apply or bring in a template that fails its check.
- **FR-030**: Provided templates MUST NOT be changed or deleted; the system MUST offer to
  copy them instead.
- **FR-031**: A template MUST only arrange text and information from the workspace: it MUST
  NOT be able to carry out commands, read files outside the workspace, or reach the
  network.
