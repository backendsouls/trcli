### User Story 6 - Define custom project types and milestone templates (Priority: P6)

A researcher whose programme does not match the built-in types — a specialization course, a
research internship, a programme with its own required checkpoints — defines a project type
of their own, with the details it asks for and the milestones it proposes, or adjusts the
milestones a built-in type proposes to match their institution.

**Why this priority**: Degree structures differ between countries and institutions. The
built-in types cover the common cases; this story covers the rest and can come last.

**Independent Test**: Define a custom type with two extra details and four proposed
milestones, create a project of that type, and confirm the details are asked for and the
milestones proposed; then change the milestones a built-in type proposes and confirm new
projects use the change while existing ones are untouched.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher defines a project type with a name, a
   description, and whether it is a degree type, **Then** it can be chosen when creating
   projects.
2. **Given** a custom type, **When** the researcher defines the milestones it proposes, each
   with a title, an order, and a typical position in the project's duration, **Then** new
   projects of that type are offered those milestones.
3. **Given** a built-in type, **When** the researcher changes the milestones it proposes,
   **Then** the change applies to projects created afterwards and existing projects are not
   altered.
4. **Given** a built-in type whose proposed milestones were changed, **When** the researcher
   asks to restore the original, **Then** the original proposal returns.
5. **Given** a custom type used by existing projects, **When** the researcher deletes it,
   **Then** the tool refuses and lists the projects that use it.
6. **Given** a custom type with the same name as an existing type, **When** the researcher
   saves it, **Then** the tool rejects it.
7. **Given** an existing project, **When** the researcher changes its type, **Then** the
   tool shows which details would be added or no longer apply, keeps all existing
   milestones, offers the new type's milestones, and requires confirmation.

---
