### User Story 16 - Connect with other tools and extend the workspace (Priority: P16)

A researcher exchanges records with the tools they already use — reference managers,
writing tools, and computational notebooks — and adds record types of their own with their
own fields, which then behave like built-in ones.

**Why this priority**: No tool is used alone, but connections are only worth building once
the records they carry are stable.

**Independent Test**: Keep a bibliography file used by a writing tool in step with a draft's
citations, and define a custom record type with two fields and create a record of it.

**Acceptance Scenarios**:

1. **Given** a draft, **When** the researcher asks the tool to keep a bibliography file in
   step with the draft's citations, **Then** the file is updated whenever those citations
   change.
2. **Given** a reference manager's library export, **When** the researcher synchronizes it
   repeatedly, **Then** new and changed entries are brought in without creating duplicates.
3. **Given** a computational notebook, **When** it is registered as a pipeline step or a
   source of results, **Then** runs record which version of the notebook was used.
4. **Given** a workspace, **When** the researcher defines a custom record type with named
   fields and their kinds, **Then** records of that type can be created, listed, viewed,
   updated, deleted, linked, and are validated and audited like built-in ones.
5. **Given** a custom record type with existing records, **When** the researcher changes its
   fields, **Then** the tool shows the effect on existing records and requires confirmation.
6. **Given** an extension from someone else, **When** the researcher adds it, **Then** the
   tool shows what it adds and what it can access before it is turned on.

---
