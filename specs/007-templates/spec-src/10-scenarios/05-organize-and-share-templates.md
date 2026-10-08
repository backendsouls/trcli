### User Story 5 - Organize and share templates (Priority: P5)

A researcher keeps personal templates that follow them across all their workspaces, and
workspace templates that belong to one body of work. A supervisor hands the group's
templates to a new student as one file, which the student brings in. When a template is
improved, the researcher can see which documents were created from the earlier version.

**Why this priority**: Templates are worth most when they are shared — a programme's thesis
structure, a group's report. This is convenience on top of working templates.

**Independent Test**: Create a personal template and a workspace template with the same
name, confirm which one is used, export two templates to one file, bring the file into
another workspace, and list the documents created from a template.

**Acceptance Scenarios**:

1. **Given** a template, **When** the researcher saves it as personal, **Then** it is
   available in every workspace they use; saved in the workspace, it is available only
   there and travels with it.
2. **Given** templates with the same name at several levels, **When** the name is used,
   **Then** the workspace's template wins over the personal one, which wins over the
   provided one, and listing shows which is in effect and which are hidden.
3. **Given** templates, **When** the researcher exports some of them, **Then** one file is
   produced that contains them completely.
4. **Given** such a file, **When** the researcher brings it in, **Then** the tool shows what
   it contains, checks each template, and adds those that pass at the level chosen.
5. **Given** a file containing a template whose name already exists at that level, **When**
   it is brought in, **Then** the tool asks whether to replace, keep both under a new name,
   or skip.
6. **Given** a template that has changed, **When** it is saved, **Then** its version
   advances, and documents record the version they were created from.
7. **Given** a template, **When** the researcher asks where it is used, **Then** the
   documents created from it, the outputs that prefer it, and the report definitions that
   name it are listed.
8. **Given** a document created from an earlier version of a template, **When** the
   researcher asks, **Then** the tool shows what changed in the template's structure since,
   and changes nothing in the document by itself.
9. **Given** a template that is in use, **When** the researcher deletes it, **Then** the
   tool lists the uses and requires confirmation; documents already created are unaffected.
10. **Given** a file that is damaged or contains something that is not a template, **When**
    it is brought in, **Then** the tool refuses and adds nothing.

---
