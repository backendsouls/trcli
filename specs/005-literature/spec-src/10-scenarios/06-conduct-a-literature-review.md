### User Story 6 - Conduct a literature review (Priority: P6)

A researcher conducts a structured review: they state the review question and the
inclusion and exclusion criteria, record every search with its search string, source, and
date, collect the candidate references found, and screen them in two stages — first on
title and abstract, then on the full text — recording a reason for every exclusion. At any
moment they can produce the flow summary that reviewers expect: how many were found,
removed as duplicates, screened, excluded at each stage and why, and finally included.

**Why this priority**: A review turns a pile of references into a defensible basis for the
research. It is the most structured use of the library and depends on all of it.

**Independent Test**: Create a review with criteria, record two searches, attach candidates
including a duplicate, screen them through both stages, and produce the flow summary with
counts that add up.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a review with a title, a
   question, and a kind (narrative, systematic, scoping, mapping), **Then** it is stored
   with the status "planning".
2. **Given** a review, **When** the researcher records its inclusion and exclusion criteria,
   each with a short label, **Then** they are shown with the review and offered as reasons
   during screening.
3. **Given** a review, **When** the researcher records a search with its search string,
   source, date, any limits applied, and the number of results, **Then** the search is kept
   as part of the review.
4. **Given** a review, **When** the researcher attaches references as candidates, saying
   which search found each, **Then** each candidate is shown as "not yet screened".
5. **Given** candidates that are duplicates of one another, **When** they are attached,
   **Then** the tool reports them, counts them as duplicates removed, and keeps one.
6. **Given** a candidate, **When** the researcher screens it on title and abstract as
   included or excluded with a reason, **Then** the decision, reason, stage, and time are
   recorded.
7. **Given** a candidate included on title and abstract, **When** the researcher screens it
   on the full text, **Then** the second decision is recorded separately from the first.
8. **Given** a candidate marked excluded without a reason, **When** the researcher submits
   the decision, **Then** the tool rejects it and asks for a reason.
9. **Given** a review, **When** the researcher asks what remains to screen at a stage,
   **Then** the unscreened candidates of that stage are listed and can be screened one
   after another.
10. **Given** a review with decisions, **When** the researcher requests the flow summary,
    **Then** it shows the counts found per source, duplicates removed, screened and excluded
    at each stage grouped by reason, and included, and the counts are consistent with one
    another.
11. **Given** a review, **When** the researcher changes a decision, **Then** the new
    decision replaces the old and the change is kept in the review's history.
12. **Given** a review, **When** the researcher marks it completed, **Then** the tool warns
    if any candidate is still unscreened, and a completed review can no longer be changed
    until reopened.
13. **Given** a completed review, **When** the researcher exports it, **Then** a document is
    produced with the question, criteria, searches, flow summary, and the list of included
    references.
14. **Given** a search dated in the future or with a negative number of results, **When**
    the researcher saves it, **Then** the tool rejects it.

---
