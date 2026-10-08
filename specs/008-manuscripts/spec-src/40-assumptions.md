## Assumptions

- **This specification owns what the researcher writes.** User Story 4 of
  `specs/001-research-workspace` (draft papers, FR-026 to FR-029) is replaced by this one;
  that specification keeps a short pointer in its place.
- **"Manuscript" is the name chosen for "papers, reports, etc."** "Draft" remains accepted
  everywhere as another word for it, so other specifications need no rewording.
- **"Reports" here are documents the researcher writes** (technical reports, lab reports,
  project reports). The tool's own accounts of activity are specified in
  `specs/006-reports` and are not manuscripts; a researcher who wants to keep working on
  one registers a manuscript and uses the exported document as its file.
- **The tool does not hold the text.** A manuscript records where its files are. Writing,
  formatting, and keeping the history of the text itself are done with the researcher's
  own tools; a version here is a recorded moment with a fingerprint, not a stored copy, and
  cannot be restored from the workspace.
- **Starting and laying out the files is specified in `specs/007-templates`**; a manuscript
  works with or without a template.
- **Submissions, reviewers' comments, and responses stay in `specs/002-research-lifecycle`**
  (its User Story 8). The request was to move the content of the first specification; a
  submission refers to a manuscript and one of its versions, and moves the manuscript
  between the stages defined here.
- **Results, figures, tables, and the flag for a replaced result are specified in
  `specs/004-experiments`**; citations, bibliographies, and the reference created for a
  publication in `specs/005-literature`; research questions, staff, and the audit trail in
  `specs/001-research-workspace`; projects and the view of what is due in
  `specs/003-research-projects`. This specification links to them and does not redefine
  them.
- **Stages are the same for every kind**, with different wording for theses. Kinds without
  peer review simply skip the stages they do not use.
- **Contribution roles follow the taxonomy most venues ask for**; the list is fixed in this
  specification and can be extended later.
- **Word counts are approximate**, counted from files that can be read as text, ignoring the
  marks of the writing format as far as it allows.
- **A publication list covers manuscripts recorded in the workspace.** Works published
  before using the tool appear once the researcher registers them as published manuscripts,
  which can be done from their identifiers.
- **Single researcher.** Authors are people named on a manuscript; they are not users of the
  workspace, and there is no shared editing.
