#### Custom types and templates

- **FR-045**: Users MUST be able to define custom project types with a name, description,
  whether it is a degree type, additional details to record, and proposed milestones.
- **FR-046**: Users MUST be able to change the milestones a built-in type proposes and
  restore the original proposal; such changes MUST apply only to projects created
  afterwards.
- **FR-047**: The system MUST refuse to delete a type used by any project, and MUST reject a
  type whose name duplicates an existing one.
- **FR-048**: Users MUST be able to change a project's type; the system MUST show the
  effect, keep all existing milestones, offer the new type's milestones, and require
  confirmation.
