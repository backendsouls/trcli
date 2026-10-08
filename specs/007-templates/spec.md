<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Templates

**Feature Branch**: `007-templates`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add the concept of templates for papers, reports, etc. on its own spec"

## Overview

Researchers write the same shapes of document again and again: an article with
introduction, methods, results, and discussion; a thesis with its front matter and
chapters; a weekly report to a supervisor; a response to reviewers; a research proposal.
Each has a structure someone expects, and much of its content — title, authors,
affiliations, abstract, reference list, tables of results — is already recorded in the
workspace.

This specification adds the **template**: a named, reusable skeleton for a document, made
of sections, guidance for the writer, and placeholders the tool fills from the workspace.
A researcher starts a document from a template instead of from a blank page, keeps the
parts that come from the workspace up to date as the research changes, and checks that the
document still has the shape its template requires.

### Kinds of template

| Kind | Used for | Examples provided |
|------|----------|-------------------|
| Paper | A manuscript for a draft | Research article (introduction, methods, results, discussion), conference paper, short paper, review article |
| Thesis | A degree document | Monograph thesis, thesis by articles, capstone project, single chapter |
| Report | An account of work done | Activity report (personal, supervisor, funder), experiment report, progress report |
| Proposal | A plan submitted for approval | Research proposal, pre-registration |
| Protocol | A plan for a structured activity | Literature review protocol |
| Letter | Correspondence around a submission | Response to reviewers, cover letter |
| Notes | Working documents | Reading notes, meeting notes |
| Other | Anything else the researcher defines | — |

### Where templates plug in

```text
                       ┌── start a document ──▶ manuscript of a Draft            (001)
 Template ── applied ──┼── lay out ───────────▶ Activity Report                  (006)
   sections            ├── lay out ───────────▶ Experiment report, Review export (004, 005)
   guidance            ├── lay out ───────────▶ Response letter, Reading notes   (002, 005)
   placeholders ◀── filled from ── workspace records (drafts, staff, references,
                                   results, figures, milestones, …)
```

Other specifications produce documents with a fixed layout; with this one, each of those
layouts becomes a template the researcher can choose or replace.

## User Scenarios & Testing *(mandatory)*

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

### User Story 2 - Keep a document in step with the workspace (Priority: P2)

Weeks after starting the paper, an author has been added, three results have changed, and
the reference list has grown. The researcher asks the tool to refresh the document: the
parts that come from the workspace — author list, affiliations, table of results, list of
figures, reference list, acknowledgement of funders — are brought up to date, and every
word the researcher wrote is left exactly as it was.

**Why this priority**: A template that fills a document once goes stale the next day.
Refreshing is what makes the workspace, rather than the manuscript, the place where facts
are kept — and it is what lets a number in a paper be traced to its run.

**Independent Test**: Apply a template to a draft, write text in a section, add an author
and change a reported result in the workspace, refresh the document, and confirm the author
list and result are updated and the written text is unchanged.

**Acceptance Scenarios**:

1. **Given** a document created from a template, **When** it is created, **Then** the parts
   that come from the workspace are visibly marked as managed by the tool, and everything
   else belongs to the researcher.
2. **Given** such a document and changes in the workspace, **When** the researcher
   refreshes it, **Then** every managed part shows the current content and no other part of
   the document is altered.
3. **Given** a document, **When** the researcher asks what a refresh would change, **Then**
   each managed part that would change is shown with its old and new content, and nothing
   is written.
4. **Given** a managed part that the researcher has edited by hand, **When** the document
   is refreshed, **Then** the tool reports the part, shows both versions, and does not
   overwrite it without the researcher's choice.
5. **Given** a managed part the researcher no longer wants updated, **When** they release
   it, **Then** it becomes ordinary text and is never refreshed again.
6. **Given** a document reporting results, **When** it is refreshed, **Then** each value
   shown is the one the draft currently reports, and a result that a newer run has replaced
   is flagged rather than silently changed.
7. **Given** a placeholder whose record has been deleted or has no value, **When** the
   document is refreshed, **Then** the place is left marked as unresolved and the tool
   lists every unresolved place.
8. **Given** a document, **When** the researcher asks whether it is up to date, **Then** the
   tool answers yes or lists what is out of date, without changing anything.
9. **Given** a document that was moved or renamed outside the tool, **When** the researcher
   refreshes it by its new location, **Then** it is recognized as the same document.
10. **Given** a file that was not created from a template, **When** the researcher asks to
    refresh it, **Then** the tool says it has nothing to manage there and changes nothing.

---

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

### User Story 4 - Use templates for reports and other outputs (Priority: P4)

The documents the tool itself produces — activity reports, experiment reports, exported
literature reviews, response letters, reading notes, notebook exports — each come out in a
layout. The researcher chooses which template lays each of them out, sets a preferred
template per kind of output, and replaces a provided layout with their own.

**Why this priority**: These documents exist without templates; templates make them fit the
reader. It depends on the earlier stories and on the specifications that produce the
documents.

**Independent Test**: Produce the weekly activity report with the provided supervisor
template and with a custom one, confirm the same content appears in two layouts, then set
the custom one as preferred and confirm it is used without being named.

**Acceptance Scenarios**:

1. **Given** an output the tool produces as a document, **When** the researcher names a
   template for it, **Then** the document is laid out by that template with the same
   content it would otherwise have.
2. **Given** a kind of output, **When** the researcher sets a preferred template for it,
   **Then** that template is used whenever none is named.
3. **Given** no preferred template, **When** an output is produced, **Then** the provided
   template for that kind is used, and the result is the same as before this specification.
4. **Given** a template of the wrong kind for an output (a paper template for an activity
   report), **When** the researcher names it, **Then** the tool refuses and lists the
   templates that fit.
5. **Given** an activity report template, **When** it leaves out a section the report has
   content for, **Then** that content is left out of the document, and the tool says which
   sections were not placed.
6. **Given** a template with headings for the researcher's own words (highlights, blockers,
   help needed), **When** a report is produced with it, **Then** those headings appear for
   the researcher to fill, or filled with the narrative already given.
7. **Given** a named report definition, **When** the researcher sets its template, **Then**
   every report produced from that definition uses it.
8. **Given** a saved report, **When** it is viewed later, **Then** it keeps the layout it
   was saved with, whatever has happened to the template since.
9. **Given** an output in a structured form meant for other programs, **When** a template is
   named, **Then** the tool says templates apply only to documents and ignores it.

---

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

### User Story 6 - Check a document against its template (Priority: P6)

Before sending a paper, a researcher asks whether it still has the shape its template
requires: every required section present and in order, no section left with only its
guidance, no placeholder unresolved, and the limits the template states — words in the
abstract, words in the whole text, number of references — respected.

**Why this priority**: Venues and programmes reject documents for form before reading them.
The check is a safety net at the end of writing and needs everything before it.

**Independent Test**: Take a document created from a template, remove a required section,
leave one section with only guidance, exceed the abstract's word limit, and confirm the
check reports exactly those three problems.

**Acceptance Scenarios**:

1. **Given** a document created from a template, **When** the researcher checks it,
   **Then** the tool reports whether every required section is present and whether the
   sections are in the template's order.
2. **Given** a section that still contains only the template's guidance, **When** the
   document is checked, **Then** the section is reported as not yet written.
3. **Given** a template that states limits (words in a section, words overall, number of
   references, number of figures), **When** the document is checked, **Then** each limit is
   shown with the document's count, and those exceeded are reported.
4. **Given** unresolved placeholders or remaining guidance, **When** the document is
   checked, **Then** each is listed with where it is.
5. **Given** a document that passes, **When** it is checked, **Then** the tool says so and
   the outcome can be used by another program to allow a next step.
6. **Given** a template requiring that the document not reveal its authors, **When** the
   document is checked, **Then** places where an author's name or affiliation from the
   workspace appears are reported.
7. **Given** a document with sections the template does not have, **When** it is checked,
   **Then** they are listed as additional, and are not treated as errors.
8. **Given** a document and a different template of the same kind, **When** the researcher
   checks the document against that template, **Then** the differences in structure are
   reported, to help move a paper from one venue to another.
9. **Given** a file the tool cannot read as text, **When** it is checked, **Then** the tool
   says so and reports nothing else.

---

### Edge Cases

- A template is applied to a draft that already has a manuscript: the tool says so and asks
  whether to create a second document, replace, or cancel; it never replaces by default.
- A template asks for information and the command cannot ask (it is run by another
  program): required items not supplied beforehand make it fail, listing them all.
- An answer to a template's question is of the wrong kind or outside its allowed values:
  it is rejected with what is expected, and nothing is created.
- A placeholder refers to information that exists for some drafts and not others (a draft
  with no funders): the template's condition decides; without one, the place is left empty
  and reported.
- A list a section repeats over is empty: the section does not appear, and nothing is left
  behind in its place.
- A value placed in a document contains characters that have a special meaning in the
  document's writing format: they are written so that they appear as themselves.
- A title, name, or abstract contains accents, non-Latin characters, or mathematics: it is
  placed as written.
- The researcher deletes the marks around a managed part: the part becomes ordinary text;
  the next refresh reports that a managed part is missing and offers to put it back.
- Two managed parts of the same kind exist in one document: both are refreshed.
- A managed part's content would be identical after refresh: the file is not rewritten.
- A document is refreshed while it is open in an editor with unsaved changes: the tool
  cannot know; it writes only when the file on disk is unchanged since it was read, and
  otherwise stops and says so.
- A template refers to another template (a thesis that includes the chapter template), and
  that one is missing or refers back to the first: the check reports it; a loop is refused.
- A template tries to place the content of a file from outside the workspace, or to have a
  command carried out: it is refused; a template can only arrange text and workspace
  information.
- A template brought in from someone else asks for information: the questions are shown
  before anything is created, like any other template.
- A provided template changes in a new version of the tool: documents created earlier are
  untouched, and the researcher's copies of it are untouched.
- Two templates at the same level differ only by letter case or accents in their names:
  the second is refused.
- A template's stated limit is zero or negative, or its sections are all optional and none
  exists: the check of the template reports it.
- A document is checked against a template that states no limits: only structure is
  checked.
- A template is deleted while a report definition names it: the definition is reported as
  needing a template when next used, and falls back to nothing by itself.

## Requirements *(mandatory)*

### Functional Requirements

#### Templates and their kinds

- **FR-001**: The system MUST support templates, each with a name, a kind (paper, thesis,
  report, proposal, protocol, letter, notes, other), a description, a version, an ordered
  set of sections, and the writing format of the documents it creates.
- **FR-002**: The system MUST provide templates for at least: a research article, a
  conference paper, a short paper, a review article, a monograph thesis, a thesis by
  articles, a capstone project, a thesis chapter, an activity report for each audience
  (personal, supervisor, funder), an experiment report, a research proposal, a
  pre-registration, a literature review protocol, a response to reviewers, a cover letter,
  reading notes, and meeting notes.
- **FR-003**: Users MUST be able to list templates, filtered by kind and by level, and see
  for each its name, kind, description, version, level, and whether a template of the same
  name at another level is hidden by it.
- **FR-004**: Users MUST be able to preview a template: its sections in order, which are
  required, their guidance, the information it asks for, the placeholders it uses, the
  limits it states, and the files it creates.
- **FR-005**: A template MAY consist of several files and MAY include other templates; the
  system MUST reject inclusion that forms a loop or refers to a missing template.

#### Applying a template

- **FR-006**: Users MUST be able to apply a template to create a document at a location they
  choose; applying a paper or thesis template to a draft MUST record on the draft the
  document's location and the template and version used.
- **FR-007**: Applying a template MUST fill every placeholder for which the workspace holds
  a value, and MUST ask for each piece of information the template declares, offering its
  default and accepting answers supplied beforehand.
- **FR-008**: The system MUST validate every answer against the declared kind, allowed
  values, and whether it is required; with invalid or missing required answers it MUST
  create nothing and report all of them together.
- **FR-009**: The system MUST NOT overwrite an existing file when applying a template unless
  the user explicitly chooses to, after being shown the files concerned.
- **FR-010**: Users MUST be able to see what applying a template would create without
  anything being written.
- **FR-011**: When it cannot ask (not run from a terminal), applying a template MUST fail,
  listing every required item not supplied, rather than wait.
- **FR-012**: Values placed in a document MUST appear as themselves in the document's
  writing format, whatever characters they contain.

#### Placeholders and managed parts

- **FR-013**: Templates MUST be able to place information from the workspace, including: a
  draft's title, abstract, ordered authors with affiliations and identifiers, target venue,
  and versions; the citations and reference list of a draft or bibliography; the results,
  figures, and tables a draft reports, with their runs; the funders and grants supporting
  a draft; a project's title, type, institution, programme, supervisors, and milestones; an
  experiment's objective, design, pipeline, runs, and conclusion; a review's question,
  criteria, searches, flow summary, and included references; the content of an activity
  report; the current date; and the researcher's name.
- **FR-014**: Users MUST be able to list every placeholder available, each with a
  description and the kind of value it yields.
- **FR-015**: Templates MUST be able to repeat a section for each item of a list and to
  include a section only under a condition on workspace information or on an answer.
- **FR-016**: Content placed from the workspace that is meant to stay current MUST be marked
  in the document as a managed part, distinguishable from the researcher's own text in the
  document's writing format.
- **FR-017**: Users MUST be able to refresh a document; refreshing MUST update every managed
  part to the workspace's current content and MUST NOT alter anything outside managed
  parts.
- **FR-018**: Users MUST be able to see what a refresh would change, part by part, and to
  ask whether a document is up to date, without anything being written.
- **FR-019**: When a managed part was edited by hand, a refresh MUST report it, show both
  versions, and not overwrite it without the user's choice; users MUST be able to release a
  managed part so it is never refreshed again.
- **FR-020**: A placeholder that cannot be resolved MUST be left visibly marked in the
  document, and the system MUST list every unresolved place.
- **FR-021**: A refresh MUST flag, and MUST NOT silently change, a reported result that a
  newer run has replaced.
- **FR-022**: The system MUST recognize a document it created after it has been moved or
  renamed, and MUST refuse to refresh a file that has no managed parts.
- **FR-023**: A refresh MUST NOT write a file whose content on disk has changed since the
  system read it, and MUST NOT rewrite a file when nothing would change.

#### Creating and checking templates

- **FR-024**: Users MUST be able to create a template from nothing or by copying any
  template under a new name, and to update and delete their own templates.
- **FR-025**: Users MUST be able to add, remove, reorder, and rename a template's sections,
  mark each required or optional, write guidance for each, and state limits (words in a
  section, words overall, number of references, number of figures and tables).
- **FR-026**: Guidance MUST appear in created documents marked so that it is
  distinguishable from the document's text and removable in one step.
- **FR-027**: Users MUST be able to declare the information a template asks for: a name, a
  question, a kind of value (text, number, date, yes/no, one of a list), whether it is
  required, and a default.
- **FR-028**: Users MUST be able to check a template; the check MUST report every unknown
  placeholder, duplicated or unnamed section, undeclared or duplicated question, default of
  the wrong kind, invalid limit, missing or looping inclusion, and unbalanced repeat or
  condition, each with its place and what is expected.
- **FR-029**: The system MUST refuse to apply or bring in a template that fails its check.
- **FR-030**: Provided templates MUST NOT be changed or deleted; the system MUST offer to
  copy them instead.
- **FR-031**: A template MUST only arrange text and information from the workspace: it MUST
  NOT be able to carry out commands, read files outside the workspace, or reach the
  network.

#### Templates for produced documents

- **FR-032**: Every document the system produces — activity reports, experiment reports,
  exported literature reviews, response letters, reading notes, notebook exports, progress
  reports, and audit reports — MUST be laid out by a template, and users MUST be able to
  name the template to use.
- **FR-033**: Users MUST be able to set a preferred template per kind of produced document,
  for themselves and per workspace; without one, the provided template MUST be used and
  MUST give the layout defined by the specification that owns the document.
- **FR-034**: The system MUST refuse a template whose kind does not fit the document being
  produced and list those that fit.
- **FR-035**: A template MUST NOT change what a produced document is allowed to contain: it
  MUST NOT place content the document's audience or scope withholds, and when it leaves out
  content the document has, the system MUST say which sections were not placed.
- **FR-036**: Users MUST be able to set the template of a named report definition; a saved
  report MUST keep the layout it was saved with.
- **FR-037**: Templates MUST apply only to documents; for output in a structured form meant
  for other programs, a named template MUST be ignored with a notice.

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

#### Checking documents

- **FR-045**: Users MUST be able to check a document against the template it was created
  from, or against any template of the same kind; the check MUST report missing required
  sections, sections out of order, sections that contain only guidance, remaining guidance,
  unresolved placeholders, managed parts out of date, and every stated limit with the
  document's count.
- **FR-046**: Sections present in the document and absent from the template MUST be listed
  as additional and MUST NOT be treated as errors.
- **FR-047**: When a template requires that a document not reveal its authors, the check
  MUST report each place where an author's name, affiliation, or identifier recorded in the
  workspace appears.
- **FR-048**: The outcome of a check MUST distinguish "passes" from "has problems" in a way
  another program can act on, and MUST change nothing in the document.

#### Common behavior

- **FR-049**: Every value a user supplies — template names, kinds, levels, locations,
  answers to a template's questions, and the content of templates brought in — MUST be
  validated before anything is created or stored; invalid input MUST change nothing and
  MUST be reported per value, all together, with what is expected.
- **FR-050**: Creating, changing, deleting, bringing in, and exporting templates, applying a
  template, and refreshing a document MUST be recorded in the workspace's audit trail;
  previewing and checking MUST NOT.
- **FR-051**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-052**: Templates MUST work without a network connection.
- **FR-053**: Every command MUST have built-in help; templates MUST have a usage guide with
  examples for applying, refreshing, creating, sharing, and checking, and a reference of
  every placeholder.

### Key Entities *(include if feature involves data)*

- **Template**: A named, versioned skeleton for a document of one kind: its sections, the
  guidance for each, the information it asks for, the placeholders it uses, the limits it
  states, and the files it creates. Lives at one level.
- **Level**: Where a template comes from and how far it reaches: provided, personal, or
  workspace.
- **Section**: One part of a template, in order, required or optional, possibly repeated
  per item of a list or present only under a condition.
- **Guidance**: Advice to the writer attached to a section, shown in the document until
  removed.
- **Question**: A piece of information a template asks for when applied, with its kind,
  default, and whether it is required.
- **Placeholder**: A place in a template where information from the workspace or an answer
  is put.
- **Limit**: A bound a template states on a document: words in a section or overall, number
  of references, figures, or tables; or a requirement such as not revealing the authors.
- **Document**: A file, or set of files, created from a template, remembering the template
  and version it came from and, for papers and theses, the draft it belongs to.
- **Managed Part**: A marked region of a document whose content comes from the workspace
  and is kept current by refreshing.
- **Template Preference**: The template chosen for a kind of produced document, for a user
  or a workspace.
- **Template Pack**: One file holding one or more templates for sharing.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can go from a draft with a title and authors to a manuscript with the
  full structure of a research article, title and authors in place, in under 1 minute.
- **SC-002**: After a refresh, 100% of managed parts show the workspace's current content,
  and 0 characters outside managed parts have changed.
- **SC-003**: A user can find out whether a document is up to date with the workspace in
  under 5 seconds, for a document with 50 managed parts.
- **SC-004**: A user can copy a provided template, add a section and a question, and have it
  pass its check in under 10 minutes using only the usage guide.
- **SC-005**: 100% of mistakes of the kinds listed in FR-028 are reported by the check of a
  template, each with its place.
- **SC-006**: No template, provided or brought in, can cause a command to be carried out, a
  file outside the workspace to be read, or the network to be reached, in 100% of attempts.
- **SC-007**: With no preferred template set, every document the tool produces is identical
  to what the owning specification defines.
- **SC-008**: A template pack exported from one workspace and brought into another yields
  templates that create identical documents from identical records.
- **SC-009**: The check of a document reports 100% of removed required sections, sections
  left with only guidance, unresolved placeholders, and exceeded limits, and reports no
  problem for a document that has none.
- **SC-010**: Applying a template never overwrites an existing file without the user's
  explicit choice, in 100% of cases.
- **SC-011**: At least 80% of users who start a paper in the tool start it from a template
  rather than from an empty file.
- **SC-012**: 90% of first-time users apply a template to a draft successfully on their
  first attempt using only the built-in help.

## Assumptions

- **Templates are for documents.** "Templates for papers, reports, etc." is read as
  skeletons for the documents a researcher writes or the tool produces. Reusable shapes for
  *records* already exist under other names and stay where they are: milestones proposed
  per project type (`specs/003-research-projects`), copying a pipeline or replicating an
  experiment (`specs/004-experiments`), named report definitions (`specs/006-reports`).
- **Documents are plain text in a writing format the researcher already uses.** The tool
  creates and updates text files; it does not typeset them, convert between formats, or
  produce word-processor files. Which writing formats the provided templates come in is
  decided at planning time, with at least two in common use among researchers.
- **The tool is still not an editor.** The researcher writes in their own editor. The tool
  reads a document only to refresh managed parts and to check structure and counts.
- **Word counts are approximate** in the way all word counts are: they count the words of
  the text and ignore the marks of the writing format as far as the format allows. They are
  a guide to a venue's limit, not a guarantee of its own count.
- **Publishers' and institutions' official templates are not shipped.** Their licences
  vary. The provided templates give common structures; a researcher brings an official
  template in as their own, and adds placeholders to it if they want it filled.
- **"Not revealing the authors"** is checked only against names, affiliations, and
  identifiers recorded in the workspace. It cannot detect self-citation phrasing or other
  indirect clues.
- **Managed parts are marked with ordinary comments of the writing format**, so a document
  remains valid and readable without the tool, and can be sent to co-authors who do not
  use it.
- **This specification depends on** `specs/001-research-workspace` (drafts, staff, audit),
  and, for each document it lays out, on the specification that owns it:
  `specs/006-reports` (activity reports and definitions), `specs/004-experiments`
  (experiment reports, results, figures), `specs/005-literature` (reference lists, review
  exports, reading notes), `specs/003-research-projects` (project details, progress), and
  `specs/002-research-lifecycle` (response letters, notebook, pre-registration, funders).
  A template for a document works once that document's owner exists.
- **Those specifications keep ownership of content.** They decide what a document contains
  and what is withheld; templates decide only how it is arranged.
- **Personal templates are kept with the user's settings**, outside any workspace; workspace
  templates are part of the workspace and are included in its backups and exports.
- **No template marketplace or online catalogue.** Sharing is by handing over a file.
- **Single researcher**, as elsewhere: there are no permissions on templates.
