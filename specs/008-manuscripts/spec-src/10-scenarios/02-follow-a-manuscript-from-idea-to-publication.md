### User Story 2 - Follow a manuscript from idea to publication (Priority: P2)

A researcher moves a manuscript through its life: idea, outlining, drafting, revising,
submitted, under review, accepted, published — or abandoned. Each move is dated and kept.
They see at a glance where everything stands, what is due soon, and how long each
manuscript has been in its current stage.

**Why this priority**: The stage is the single most asked-about fact of any piece of
writing — by supervisors, co-authors, and the researcher themselves.

**Independent Test**: Move a manuscript through four stages, view its history with dates,
list manuscripts grouped by stage, and see which are due within 30 days.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher changes its stage, **Then** the new stage
   and the date are recorded, and the earlier stage and how long it lasted are kept in its
   history.
2. **Given** a manuscript, **When** the researcher views its history, **Then** stage
   changes, versions, and changes of venue or deadline are shown in order of time.
3. **Given** manuscripts at different stages, **When** the researcher asks for an overview,
   **Then** they are shown grouped by stage, with the number in each.
4. **Given** manuscripts with deadlines, **When** the researcher asks what is due within a
   period, **Then** those manuscripts are listed in date order, overdue ones first.
5. **Given** a manuscript, **When** the researcher moves it backwards (from submitted to
   revising), **Then** the move is accepted and recorded like any other.
6. **Given** a manuscript, **When** the researcher abandons it, **Then** a reason is
   required, and it leaves the lists of work in progress unless asked for.
7. **Given** an abandoned manuscript, **When** the researcher resumes it, **Then** it
   returns to the stage it had before.
8. **Given** a thesis, **When** its stages are shown, **Then** "accepted" reads "defended"
   and "published" reads "deposited", the stages being otherwise the same.
9. **Given** a manuscript moved to "published", **When** the move is made, **Then** the tool
   asks for the publication details, or records that they are still to be given.
10. **Given** a manuscript whose deadline changes, **When** the new date is saved, **Then**
    the earlier date is kept in the history.
11. **Given** a manuscript that has stayed in one stage longer than a period the researcher
    has set, **When** the overview is shown, **Then** it is marked as stalled.
12. **Given** a stage that does not exist, **When** the researcher sets it, **Then** the
    tool rejects it and lists the valid stages.

---
