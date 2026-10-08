### User Story 1 - Keep a register of the people in the group (Priority: P1)

A lab head records the people of the group and those who work with it: each person's name,
how to reach them, their position (doctoral student, postdoctoral researcher, technician,
visitor, outside collaborator…), when they joined, when they are expected to leave, and
what funds their stay. They can see at a glance who is in the group now, who is about to
leave, and who has left.

**Why this priority**: Every other part of TRCLI that names a person — an author, a
supervisor, someone responsible for an experiment — picks them from this register. With
only this story, a lab head has an accurate list of their group and its key dates.

**Independent Test**: Add four people in different positions with start and expected end
dates, list current members, list those whose stay ends within six months, mark one as
having left, and confirm they appear among former members.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the lab head adds a person with a name and a position,
   **Then** the person is stored as a current member of the group.
2. **Given** a person, **When** the lab head records their affiliation, e-mail address,
   researcher identifier, and web page, **Then** these are saved and shown.
3. **Given** a person, **When** the lab head records the date they joined, the date they are
   expected to leave, and what funds their stay with the date that funding ends, **Then**
   these are saved and shown.
4. **Given** people in the register, **When** the lab head lists them, **Then** current
   members are shown by default, with position, joining date, and expected end, and the list
   can be filtered by position, status, and funding.
5. **Given** people with expected end dates or funding end dates, **When** the lab head
   asks who is leaving or losing funding within a period, **Then** those people are listed
   in date order, and those dates also appear in the workspace's view of what is due.
6. **Given** a person who is not a member of the group, **When** the lab head adds them as
   an outside collaborator, **Then** they can be named as authors and contacts, and are not
   counted among the group's members.
7. **Given** a person whose position changes (a master's student becomes a doctoral
   student), **When** the lab head records the new position with its date, **Then** the
   earlier one is kept in the person's history.
8. **Given** a person, **When** the lab head views them, **Then** their details, position
   history, supervisor, and a summary of what they are assigned to are shown.
9. **Given** an e-mail address or a researcher identifier in an invalid form, an expected
   end before the joining date, or a missing name, **When** the lab head saves, **Then** the
   tool rejects it and reports every problem together.
10. **Given** a person with the same name or the same researcher identifier as one already
    in the register, **When** they are added, **Then** the tool reports the likely duplicate
    and asks whether to add or cancel.
11. **Given** a person, **When** the lab head adds a private note about them, **Then** the
    note is visible only in the workspace and is never included in anything produced for
    others.

---
