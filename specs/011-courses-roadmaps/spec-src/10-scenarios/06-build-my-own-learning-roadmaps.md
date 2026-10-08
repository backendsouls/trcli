### User Story 6 - Build my own learning roadmaps (Priority: P6)

A researcher sets themselves a learning goal — "be able to run and interpret Bayesian
models" — and lays out how to get there: stages in order, each with items to complete: a
course (in their programme or elsewhere), a book or paper to read, a skill to practise, a
small task. Items can depend on others. They mark items done, see how far along each stage
is, and can share a roadmap with a colleague or start from one somebody gave them.

**Why this priority**: Much of what a researcher must learn is in no curriculum. A personal
roadmap gives that learning the same visibility. It is independent of the programme stories
and the least urgent.

**Independent Test**: Create a roadmap with two stages and six items of four kinds, add a
dependency, mark three items done, view progress per stage, export the roadmap, and bring
it into another workspace as a fresh, not-started copy.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a roadmap with a title and a goal,
   **Then** it is stored with the status "active".
2. **Given** a roadmap, **When** the researcher adds stages in order, each with a title and
   an optional target date, **Then** the roadmap shows them.
3. **Given** a stage, **When** the researcher adds an item of a kind — a course, a reference
   from the library, a skill, a task, or a free item with a web address — **Then** it is
   stored with the status "to do".
4. **Given** an item that is a course, a reference, or a task, **When** it is linked to the
   existing record, **Then** its status follows that record: the course completed, the
   reference read, the task done.
5. **Given** items, **When** the researcher states that one depends on another, **Then** the
   dependent item is shown as blocked until the other is done.
6. **Given** a roadmap, **When** the researcher views it, **Then** each stage shows its
   items with status and how many are done, and the roadmap shows its overall progress and
   what to do next.
7. **Given** an item, **When** the researcher marks it done, skipped, or in progress, and
   records the time it took or a note of what was learned, **Then** this is saved.
8. **Given** a skill item, **When** the researcher records their level before and after
   (none, basic, working, proficient), **Then** the roadmap shows the change.
9. **Given** a roadmap, **When** the researcher exports it, **Then** a file is produced with
   its stages, items, and dependencies and without their personal progress.
10. **Given** such a file, **When** the researcher brings it in, **Then** a new roadmap is
    created, not started; references it names that are not in the library are listed and
    can be added.
11. **Given** a roadmap, **When** every item is done or skipped, **Then** the tool offers to
    mark the roadmap completed with a closing note.
12. **Given** items that depend on each other in a loop, a stage without a title, or a
    target date in an invalid form, **When** the researcher saves, **Then** the tool rejects
    it.
13. **Given** several roadmaps, **When** the researcher lists them, **Then** each is shown
    with its goal, progress, and next item.

---
