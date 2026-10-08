### User Story 2 - See who is doing what (Priority: P2)

A lab head assigns people to the group's work — as members of a project, authors of a
manuscript, responsible for an experiment, performers of a manual step, owners of a task or
a dataset — and looks at it from the other side: for one person, everything they are on;
for the whole group, a table of who is on what, who has little, and which pieces of work
have nobody.

**Why this priority**: "Who is working on this?" and "what is this person working on?" are
the questions a lab head answers most often. The assignments already exist across the other
specifications; this gathers them into one view.

**Independent Test**: Assign three people across two projects, a manuscript, and several
tasks; view one person's assignments; view the group overview; and list the active projects
and experiments that have nobody responsible.

**Acceptance Scenarios**:

1. **Given** a project, **When** the lab head adds people to it with their role in it,
   **Then** the project lists its members and each person lists the project.
2. **Given** a person, **When** the lab head asks what they are assigned to, **Then** their
   projects, manuscripts (with their position among the authors), experiments, manual
   steps, tasks, and datasets are listed, with the status and next date of each.
3. **Given** the group, **When** the lab head asks for the overview, **Then** each current
   member is shown with the number of active items of each kind, their next deadline, and
   their overdue items.
4. **Given** the overview, **When** a member has no active assignment, or more overdue
   items than a number the lab head has set, **Then** they are marked.
5. **Given** active projects, experiments, and manuscripts, **When** the lab head asks what
   has nobody responsible, **Then** those are listed.
6. **Given** a person, **When** the lab head lists what is due for them within a period,
   **Then** their tasks, milestones, and deadlines are listed in date order.
7. **Given** a person assigned to something, **When** the lab head removes the assignment,
   **Then** it is removed from both sides and the earlier assignment is kept in history.
8. **Given** a person who has left, **When** the lab head tries to assign them something
   new, **Then** the tool warns and asks for confirmation.
9. **Given** a person or a piece of work that does not exist, **When** the lab head assigns,
   **Then** the tool rejects it.
10. **Given** the overview, **When** the lab head exports it, **Then** a document is
    produced without private notes.

---
