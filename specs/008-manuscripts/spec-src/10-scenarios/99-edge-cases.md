### Edge Cases

- Two manuscripts have the same title: both are accepted; identifiers tell them apart, and
  the tool mentions the other when the second is created.
- A manuscript has no authors yet: it is accepted; the readiness check reports it.
- A manuscript's files are moved or deleted outside the tool: the manuscript is kept; the
  tool reports the files as missing when it next needs them, and never guesses a new place.
- A manuscript's file location is a directory with many files: the fingerprint and word
  count cover the files the researcher has said belong to it, or all text files otherwise.
- Files cannot be read as text (a word-processor file): versions are recorded with a
  fingerprint and size, without a word count, and the tool says so.
- A deadline passes while a manuscript is still being drafted: it is shown as overdue
  everywhere it appears; nothing changes by itself.
- A manuscript is moved to "published" without ever having been "submitted": it is accepted
  (a technical report is simply published), with no warning for kinds that have no review.
- A manuscript is rejected by a venue: its stage returns to "revising", the target venue is
  cleared, and the rejection is kept in the history (submissions themselves are specified
  in `specs/002-research-lifecycle`).
- The target venue changes: the earlier venue is kept in the history.
- An author is listed whose name is written differently in the staff register and in the
  published work: both forms are kept; the published form is used for the reference.
- Authors are reordered after a version was recorded: the version keeps the order it had.
- A contribution role is recorded for someone who is not an author: the tool rejects it and
  offers to acknowledge the person instead.
- A part's own manuscript is deleted: the part remains as a plain part with the title it
  had.
- A part's deadline is later than the manuscript's: accepted with a warning.
- The sum of the parts' target lengths differs from the manuscript's target: both are
  shown; neither is changed.
- A manuscript is duplicated to prepare a submission elsewhere: the copy is linked to the
  original as derived from it.
- The published work already exists in the library because the researcher added it by hand:
  it is linked, not duplicated.
- A manuscript belongs to two projects: it is one manuscript, listed in both.
- A manuscript's kind is changed from paper to thesis after parts and versions exist: parts
  and versions are kept; only the kind-specific details are affected.
- Text is given in an invalid form — a title that is empty or far too long, keywords
  beyond the allowed number: the tool rejects it, naming each value and the limit.
