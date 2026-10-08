### User Story 3 - See where I stand in the curriculum (Priority: P3)

A researcher sees their progress through the curriculum: which components are completed, in
progress, and still to do; credits earned against required, in total and per category; and
which components they could take next because their prerequisites are met. A course taken
elsewhere, or under another name, is counted by declaring it equivalent to a component.

**Why this priority**: "How much is left?" and "what can I take next term?" are the two
questions a student asks every term. This answers both from records that already exist.

**Independent Test**: With a curriculum and several completed courses, view progress and
confirm the credits per category; declare an outside course equivalent to a component and
confirm it now counts; list what can be taken next and confirm a component with an unmet
prerequisite is not on it.

**Acceptance Scenarios**:

1. **Given** a course taken with the same code as a component of the curriculum, **When**
   it is recorded, **Then** it is counted for that component without further action.
2. **Given** a course with a different code or from another institution, **When** the
   researcher declares it equivalent to a component, **Then** it counts for that component,
   and the equivalence is recorded with a note of who granted it.
3. **Given** a curriculum and courses, **When** the researcher asks for their progress,
   **Then** each component is shown as completed, in progress, planned, or to do, with the
   course and grade behind it.
4. **Given** progress, **When** it is summarized, **Then** credits earned, in progress, and
   remaining are shown in total and for each category and area with a minimum, and each
   rule of the curriculum is shown as met or not.
5. **Given** a curriculum, **When** the researcher asks what they can take next, **Then**
   the components not yet completed whose prerequisites are all completed are listed,
   mandatory ones first.
6. **Given** a component whose prerequisite is not completed, **When** the researcher asks
   why it is not available, **Then** the tool names the prerequisites still missing.
7. **Given** a course that fulfils no component, **When** progress is shown, **Then** it is
   listed separately as counting toward free or extra credits, or not at all, as the
   researcher states.
8. **Given** an elective group needing a number of credits, **When** progress is shown,
   **Then** the credits earned within the group are shown against the number needed.
9. **Given** a component for which the researcher was exempted, **When** the exemption is
   recorded with its justification, **Then** the component counts as completed without a
   grade.
10. **Given** one course declared equivalent to two components, or two courses to the same
    component, **When** the researcher saves, **Then** the tool warns and asks for
    confirmation, since institutions differ on whether this is allowed.
11. **Given** progress, **When** the researcher compares it with the suggested terms,
    **Then** the tool shows whether they are ahead of, on, or behind the suggested pace.
12. **Given** a programme with no curriculum recorded, **When** the researcher asks for
    progress, **Then** the tool shows the transcript totals and says no curriculum is
    recorded.

---
