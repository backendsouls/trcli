#### Applying a template

- **FR-006**: Users MUST be able to apply a template to create a document at a location they
  choose; applying a paper or thesis template to a draft MUST record on the draft the
  document's location and the template and version used.
- **FR-007**: Applying a template MUST fill every placeholder for which the workspace holds
  a value, and MUST ask for each piece of information the template declares, offering its
  default and accepting answers supplied beforehand.
- **FR-008**: The system MUST validate every answer against the declared kind, allowed
  values, and whether it is required; with invalid or missing required answers it MUST
  create nothing and report all of them together.
- **FR-009**: The system MUST NOT overwrite an existing file when applying a template unless
  the user explicitly chooses to, after being shown the files concerned.
- **FR-010**: Users MUST be able to see what applying a template would create without
  anything being written.
- **FR-011**: When it cannot ask (not run from a terminal), applying a template MUST fail,
  listing every required item not supplied, rather than wait.
- **FR-012**: Values placed in a document MUST appear as themselves in the document's
  writing format, whatever characters they contain.
