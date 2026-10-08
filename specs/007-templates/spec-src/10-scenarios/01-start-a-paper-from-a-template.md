### User Story 1 - Start a paper from a template (Priority: P1)

A researcher about to write a paper looks at the templates available, previews one, and
applies it to their draft. The tool creates the manuscript with the template's sections
and guidance, and with the draft's title, authors, affiliations, and abstract already in
place.

**Why this priority**: Starting a document is the moment a template is worth the most, and
papers are the document researchers care most about. With only this story, nobody starts
from a blank page.

**Independent Test**: List the paper templates, preview one, apply it to a draft that has a
title and two authors, and confirm the manuscript is created with every section of the
template and with the title and authors filled in.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher lists templates, **Then** each is shown
   with its name, kind, a one-line description, and where it comes from (provided with the
   tool, the researcher's own, or the workspace's).
2. **Given** templates of several kinds, **When** the researcher lists those of one kind,
   **Then** only that kind is shown.
3. **Given** a template, **When** the researcher previews it, **Then** its sections are
   shown in order, each marked required or optional, with its guidance and the information
   it will ask for.
4. **Given** a draft and a paper template, **When** the researcher applies the template to
   the draft, **Then** the manuscript is created at the chosen location with every section
   of the template, and the draft records that location and the template used.
5. **Given** a draft with a title, authors with affiliations, and an abstract, **When** a
   template is applied, **Then** those appear in the manuscript where the template places
   them.
6. **Given** a template that asks for information the workspace does not hold (for example
   a running title or keywords), **When** it is applied, **Then** the tool asks for each
   item, offering the template's default, and accepts the answers given beforehand without
   asking.
7. **Given** a location that already holds files the template would create, **When** the
   researcher applies it, **Then** the tool lists those files and creates nothing unless
   the researcher explicitly chooses to replace them.
8. **Given** a template, **When** the researcher asks what applying it would create,
   **Then** the files and their sections are listed and nothing is written.
9. **Given** a template made of several files (a thesis with a file per chapter), **When**
   it is applied, **Then** all of them are created in the arrangement the template defines.
10. **Given** a template name that does not exist, **When** the researcher applies it,
    **Then** the tool says so and suggests templates with similar names.
11. **Given** a paper template applied without a draft, **When** the researcher applies it,
    **Then** the tool asks which draft it is for, or creates the document unattached when
    the researcher says so.

---
