#### Checking documents

- **FR-045**: Users MUST be able to check a document against the template it was created
  from, or against any template of the same kind; the check MUST report missing required
  sections, sections out of order, sections that contain only guidance, remaining guidance,
  unresolved placeholders, managed parts out of date, and every stated limit with the
  document's count.
- **FR-046**: Sections present in the document and absent from the template MUST be listed
  as additional and MUST NOT be treated as errors.
- **FR-047**: When a template requires that a document not reveal its authors, the check
  MUST report each place where an author's name, affiliation, or identifier recorded in the
  workspace appears.
- **FR-048**: The outcome of a check MUST distinguish "passes" from "has problems" in a way
  another program can act on, and MUST change nothing in the document.
