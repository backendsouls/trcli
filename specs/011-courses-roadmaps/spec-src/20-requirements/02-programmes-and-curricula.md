#### Programmes and curricula

- **FR-011**: Users MUST be able to create, list, view, update, and delete programmes, each
  with a name, an institution, a degree level, the date of enrolment, the time normally
  expected, the maximum time allowed, a status (enrolled, on leave, completed, withdrawn),
  and the project it belongs to.
- **FR-012**: Users MUST be able to record more than one programme, at the same time or one
  after another.
- **FR-013**: Users MUST be able to create a curriculum for a programme with a name and the
  year it took effect, keep several curricula for one programme, and state which one they
  follow.
- **FR-014**: Users MUST be able to add, update, and remove a curriculum's components, each
  with a code unique in the curriculum, a title, credits, hours, a category (mandatory,
  elective, optional, complementary activity), a suggested term, an area, and the terms in
  which it is offered.
- **FR-015**: Users MUST be able to state that a component requires others to be completed
  first (prerequisites) or to be taken in the same term (corequisites); the system MUST
  reject a requirement on a component that does not exist and requirements that form a
  loop, naming the components.
- **FR-016**: Users MUST be able to define groups of components from which a stated number
  of credits or of components must be completed.
- **FR-017**: Users MUST be able to record a curriculum's rules: total credits required, and
  minimum credits per category, per area, and per group.
- **FR-018**: The system MUST present a curriculum as a matrix: components by suggested term
  with code, title, credits, and category, prerequisites marked, groups shown as one
  requirement with their options, and totals per term.
- **FR-019**: Users MUST be able to bring in components from a table in a file and export a
  curriculum to one; bringing in MUST report each invalid row with its reason, MUST offer a
  preview that stores nothing, and MUST let the user say which column holds which detail.
- **FR-020**: When the user moves to another curriculum of the programme, the system MUST
  show which completed components carry over and which no longer count, and MUST keep the
  earlier curriculum.
