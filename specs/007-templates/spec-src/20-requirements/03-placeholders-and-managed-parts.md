#### Placeholders and managed parts

- **FR-013**: Templates MUST be able to place information from the workspace, including: a
  draft's title, abstract, ordered authors with affiliations and identifiers, target venue,
  and versions; the citations and reference list of a draft or bibliography; the results,
  figures, and tables a draft reports, with their runs; the funders and grants supporting
  a draft; a project's title, type, institution, programme, supervisors, and milestones; an
  experiment's objective, design, pipeline, runs, and conclusion; a review's question,
  criteria, searches, flow summary, and included references; the content of an activity
  report; the current date; and the researcher's name.
- **FR-014**: Users MUST be able to list every placeholder available, each with a
  description and the kind of value it yields.
- **FR-015**: Templates MUST be able to repeat a section for each item of a list and to
  include a section only under a condition on workspace information or on an answer.
- **FR-016**: Content placed from the workspace that is meant to stay current MUST be marked
  in the document as a managed part, distinguishable from the researcher's own text in the
  document's writing format.
- **FR-017**: Users MUST be able to refresh a document; refreshing MUST update every managed
  part to the workspace's current content and MUST NOT alter anything outside managed
  parts.
- **FR-018**: Users MUST be able to see what a refresh would change, part by part, and to
  ask whether a document is up to date, without anything being written.
- **FR-019**: When a managed part was edited by hand, a refresh MUST report it, show both
  versions, and not overwrite it without the user's choice; users MUST be able to release a
  managed part so it is never refreshed again.
- **FR-020**: A placeholder that cannot be resolved MUST be left visibly marked in the
  document, and the system MUST list every unresolved place.
- **FR-021**: A refresh MUST flag, and MUST NOT silently change, a reported result that a
  newer run has replaced.
- **FR-022**: The system MUST recognize a document it created after it has been moved or
  renamed, and MUST refuse to refresh a file that has no managed parts.
- **FR-023**: A refresh MUST NOT write a file whose content on disk has changed since the
  system read it, and MUST NOT rewrite a file when nothing would change.
