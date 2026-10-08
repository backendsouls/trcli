### User Story 4 - Plan the terms ahead (Priority: P4)

A researcher lays out which components to take in each coming term. The tool checks the
plan: prerequisites come first, the credit load of each term is within the limits the
researcher sets, every mandatory component is somewhere, and the last term falls within the
time the programme allows. They can keep more than one plan and compare them.

**Why this priority**: Planning is the forward-looking use of the curriculum. It is worth
doing once position is known, and it is what prevents discovering in the final year that a
prerequisite chain is one term too long.

**Independent Test**: Plan three terms, deliberately place a component before its
prerequisite and overload one term, and confirm the check reports both; fix them and
confirm the projected completion date.

**Acceptance Scenarios**:

1. **Given** a curriculum, **When** the researcher places components in future terms,
   **Then** the plan is stored and shown term by term with the credits of each term.
2. **Given** a plan, **When** it is checked, **Then** the tool reports components placed
   before or with a prerequisite that must come first, terms above or below the credit load
   the researcher set, mandatory components placed nowhere, and rules of the curriculum the
   plan would not meet.
3. **Given** a plan, **When** it is shown, **Then** the term in which the curriculum would
   be completed is shown, and whether that is within the expected and the maximum time.
4. **Given** a plan, **When** the researcher asks the tool to propose one, **Then**
   remaining components are spread over terms respecting prerequisites, suggested terms,
   and the credit load, as a proposal to edit; nothing is saved without acceptance.
5. **Given** a component offered only in certain terms, **When** the researcher records
   that, **Then** the check reports a plan that places it in another term.
6. **Given** a term that begins, **When** the researcher enrols in the planned components,
   **Then** courses are created from the plan for that term in one step.
7. **Given** a course failed or dropped, **When** the plan is checked, **Then** the
   component is shown as unplaced again, with what else is now delayed by it.
8. **Given** two plans, **When** the researcher compares them, **Then** their differences
   term by term and their completion terms are shown.
9. **Given** a plan, **When** the researcher marks it as the one they follow, **Then**
   progress and the view of what is due use it.
10. **Given** a term in the past, or a component already completed, **When** it is placed
    in a plan, **Then** the tool rejects it.
11. **Given** terms defined by the researcher's institution (two or three per year, with
    their dates), **When** the researcher records them, **Then** plans and courses use
    those terms.

---
