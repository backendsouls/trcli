### User Story 2 - Describe how the work is done (Priority: P2)

A researcher records the methodologies their work relies on: what each is for, when to use
it and when not, its steps in order, what it needs and what it produces, how to tell it
went well, and the mistakes people make. They say where it comes from — a published method,
the lab's own procedure, common practice, or something worked out for this research — and,
when they adapt one, exactly how their version differs.

**Why this priority**: The method is what a result's credibility rests on, and what a
methods section must describe. Experiments, reviews, and models already point to
methodologies; this makes what they point to worth reading.

**Independent Test**: Record a methodology from the literature with its reference and five
steps, adapt it into a lab variant stating two differences, link the variant to an
experiment, and view the experiment's method with its lineage.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a methodology with a name and a
   purpose, **Then** it is stored at version 1.
2. **Given** a methodology, **When** the researcher records when to use it, when not to, what
   it needs beforehand, and what it produces, **Then** these are shown with it.
3. **Given** a methodology, **When** the researcher writes its procedure as ordered steps,
   each with what to do and an optional check that it was done right, **Then** the steps are
   stored in order.
4. **Given** a methodology, **When** the researcher records its origin and, for one from the
   literature, the references that define it, **Then** these are shown, and the references
   list the methodology.
5. **Given** a methodology, **When** the researcher records its known pitfalls and its
   limitations, **Then** they are shown with it.
6. **Given** a methodology, **When** the researcher adapts it into a new one, **Then** the
   new methodology records which one it was adapted from and each difference with its
   reason.
7. **Given** an adapted methodology, **When** it is viewed, **Then** its lineage back to the
   original is shown, with the differences introduced at each step.
8. **Given** a methodology, **When** its procedure changes and a new version is recorded
   with a note, **Then** earlier versions remain viewable, and work that used an earlier
   version still shows that version.
9. **Given** an experiment, a literature review, or a model, **When** the researcher links
   the methodology it follows, **Then** the version in effect is recorded with it.
10. **Given** a methodology, **When** the researcher asks where it is used, **Then** the
    experiments, reviews, models, and manuscripts that rely on it are listed, by version.
11. **Given** a methodology with steps, **When** the researcher turns it into the pipeline of
    an experiment, **Then** a step is created for each, to be marked automated or manual.
12. **Given** methodologies, **When** the researcher lists them filtered by origin, kind of
    work, or status, or searches them, **Then** only matching ones are shown.
13. **Given** a methodology in use, **When** the researcher deletes it, **Then** the tool
    refuses and lists what uses it; it can be retired instead.
14. **Given** a missing name or purpose, a step without text, or a methodology adapted from
    itself, **When** the researcher saves, **Then** the tool rejects it.

---
