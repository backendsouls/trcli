#### Levels, sharing, and versions

- **FR-038**: Templates MUST exist at three levels: provided with the tool, personal
  (available in every workspace of the user), and workspace (stored with one workspace).
  For a given name the workspace template MUST take precedence over the personal one, and
  the personal one over the provided one.
- **FR-039**: Template names MUST be unique within a level, ignoring letter case and accents.
- **FR-040**: Users MUST be able to export one or more templates to a single file, and bring
  such a file in at a chosen level; before adding anything the system MUST show its
  contents, check each template, and ask what to do about names that already exist
  (replace, keep both, skip).
- **FR-041**: The system MUST refuse a file that is damaged or is not a set of templates,
  and add nothing.
- **FR-042**: Each change to a template MUST advance its version; each document and each
  saved report MUST record the template and version it was created from.
- **FR-043**: Users MUST be able to list where a template is used — documents created from
  it, kinds of output that prefer it, and report definitions that name it — and to see, for
  a document, how the template's structure has changed since the version it was created
  from. The system MUST NOT restructure a document by itself.
- **FR-044**: Deleting a template that is in use MUST list its uses and require
  confirmation, and MUST NOT affect documents already created.
