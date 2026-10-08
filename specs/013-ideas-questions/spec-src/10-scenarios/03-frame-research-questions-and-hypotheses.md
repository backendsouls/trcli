### User Story 3 - Frame research questions and hypotheses (Priority: P3)

A researcher records the questions the research is trying to answer and the hypotheses that
follow from them, and links every other kind of record to them. At any time they can ask,
for one question, "what have I read, reviewed, tried, found, and written about this?", and
for the whole workspace, "which questions are still open, and which have nothing happening?".

**Why this priority**: The question is what gives the rest of the workspace its meaning.
It comes after capture and sorting only because those are what feed it; it is the part of
this specification every other specification links to.

**Independent Test**: Create a research question with two hypotheses, link references to
the question, link a result to a hypothesis as evidence, mark the hypothesis supported, and
view the question showing its hypotheses and everything linked to it.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a research question with a
   statement, **Then** it is stored with the status "open".
2. **Given** a research question, **When** the researcher adds a hypothesis with a statement
   and what would count as support or refutation, **Then** the hypothesis is stored under
   that question with the status "untested".
3. **Given** a research question, **When** the researcher adds a narrower sub-question,
   **Then** the sub-question is shown beneath its parent.
4. **Given** questions and hypotheses, **When** the researcher links references, reviews,
   experiments, results, manuscripts, methodologies, or datasets to them, **Then** the link
   is visible from both ends.
5. **Given** a research question, **When** the researcher asks for its overview, **Then**
   they see its sub-questions, its hypotheses with their status, the ideas it grew from, and
   every linked record grouped by kind.
6. **Given** a hypothesis, **When** the researcher sets its status to supported, refuted, or
   inconclusive, **Then** the tool requires at least one linked result as evidence and
   records the date of the change.
7. **Given** a research question, **When** the researcher marks it answered or abandoned,
   **Then** the status, date, and a required closing note are recorded.
8. **Given** a research question, **When** the researcher records its importance and how
   feasible it seems, **Then** questions can be ordered by either.
9. **Given** records that are linked to no research question, **When** the researcher asks
   for unlinked records, **Then** they are listed by kind.
10. **Given** open questions, **When** the researcher asks which have had no activity for a
    period — no linked record added, no hypothesis changed — **Then** those are listed with
    the date of their last activity.
11. **Given** a hypothesis that an experiment tests, **When** the experiment is concluded
    with an outcome for it, **Then** the tool offers to update the hypothesis and does so
    only on acceptance.
12. **Given** a question whose statement the researcher refines, **When** it is changed,
    **Then** the earlier wording is kept in the question's history.
13. **Given** a question with sub-questions, **When** the researcher deletes it, **Then** the
    tool refuses until the sub-questions are moved or removed; deleting a question or
    hypothesis with linked records lists them and requires confirmation, and the linked
    records are kept.
14. **Given** a sub-question made the parent of its own ancestor, or a missing statement,
    **When** the researcher saves, **Then** the tool rejects it.

---
