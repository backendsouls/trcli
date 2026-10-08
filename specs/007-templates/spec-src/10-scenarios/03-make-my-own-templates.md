### User Story 3 - Make my own templates (Priority: P3)

A researcher's programme requires a thesis structure the tool does not provide, and their
group has its own layout for a weekly report. They copy the nearest template, change its
sections and guidance, declare the information it should ask for, and save it under a new
name. The tool checks the template and tells them precisely what is wrong before it can
mislead anyone.

**Why this priority**: Provided templates can only cover the common cases; institutions,
venues, and groups each have their own. Custom templates are what make the feature fit real
requirements.

**Independent Test**: Copy a provided template under a new name, add a required section and
a piece of information to ask for, check it, apply it, and confirm the new section and the
question appear.

**Acceptance Scenarios**:

1. **Given** a provided template, **When** the researcher copies it under a new name,
   **Then** an editable template of their own is created and the provided one is unchanged.
2. **Given** no starting point, **When** the researcher creates an empty template with a
   name, a kind, and a description, **Then** it is created ready to be filled in.
3. **Given** their own template, **When** the researcher adds, removes, reorders, or renames
   sections and marks each required or optional, **Then** the template reflects it.
4. **Given** a section, **When** the researcher writes guidance for it, **Then** the
   guidance appears in documents created from the template, marked so that it can be told
   apart from the document's own text and removed in one step.
5. **Given** a template, **When** the researcher declares a piece of information to ask for,
   with a name, a question, a kind of value, whether it is required, and a default,
   **Then** applying the template asks for it and validates the answer.
6. **Given** a template, **When** the researcher places a placeholder for something the
   workspace holds — the draft's title, its authors, the reported results, the reference
   list, the project's supervisor — **Then** applying the template fills it.
7. **Given** a template, **When** the researcher marks a section as repeating for each item
   of a list (each author, each reported figure, each chapter), **Then** the document
   contains one copy per item.
8. **Given** a template, **When** the researcher marks a section as present only under a
   condition (only when the draft has funders), **Then** the section appears only then.
9. **Given** a template with a mistake — an unknown placeholder, a section declared twice,
   a question without a name, a default of the wrong kind — **When** the researcher checks
   it, **Then** every mistake is listed with where it is and what is expected.
10. **Given** a template that fails its check, **When** the researcher applies it, **Then**
    the tool refuses and shows the check's findings.
11. **Given** a provided template, **When** the researcher tries to change or delete it,
    **Then** the tool refuses and offers to make a copy.
12. **Given** a template, **When** the researcher asks which placeholders exist, **Then**
    every piece of workspace information that can be placed is listed with a description.

---
