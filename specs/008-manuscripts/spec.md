<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Manuscripts

**Feature Branch**: `008-manuscripts`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "separate the concept of papers, reports, etc. and crud operations on it from the first spec to its own spec"

## Overview

Research ends in writing: an article, a conference paper, a thesis, a technical report, a
proposal. Each is a piece of work in its own right, with authors, a destination, a
deadline, a history of versions, and a long road from idea to publication. This
specification gathers everything TRCLI knows about **what the researcher writes** into one
place, under one name: the **manuscript**.

It **takes over** the writing content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 4 | Draft papers: create, list, view, update, delete; stages; versions; links to citations and evidence |

It **adds** what was not specified before: kinds of manuscript other than papers, authors'
contributions, a manuscript's parts (chapters and sections) with their own progress,
manuscripts made of other manuscripts, and the record of publication.

### The word "manuscript"

A **manuscript** is anything the researcher writes for others to read: a paper, a report, a
thesis. The first specification called this a "draft"; that word is kept as another name
for it, and wherever another specification says "draft" it means a manuscript.

A manuscript is a **record about** a piece of writing — what it is, who wrote it, where it
stands. The writing itself lives in files the researcher edits with their own tools; the
manuscript knows where those files are.

### Kinds of manuscript

| Kind | Examples |
|------|----------|
| Paper | Journal article, conference paper, workshop paper, short paper, preprint, review article |
| Thesis | Doctoral thesis, master's dissertation, capstone project, thesis chapter |
| Report | Technical report, lab report, project report, white paper |
| Proposal | Research proposal, grant application, thesis proposal |
| Book | Book, book chapter |
| Presentation | Abstract, poster, slides |
| Other | Essay, note, anything else the researcher writes |

### Three things that are easily confused

| | What it is | Specified in |
|---|-----------|--------------|
| **Manuscript** | What *you* write, tracked from idea to publication | here |
| **Reference** | What *others* wrote, which you read and cite | `specs/005-literature` |
| **Activity report** | What *the tool* produces from the workspace's history | `specs/006-reports` |

A "report" in this specification is a document the researcher writes — a technical report —
not the tool's account of the week.

### The concepts, in one picture

```text
 Project ── contains ──▶ Manuscript ── has ──▶ Parts (chapters, sections)
                           │  │  │                  └─ may be another Manuscript
        Staff ── authors ──┘  │  └── has ──▶ Versions
        (order, contribution) │
                              ├── cites ──▶ Citations / Bibliography        (005)
                              ├── reports ─▶ Results, Figures, Tables        (004)
                              ├── answers ─▶ Research Questions              (001)
                              ├── written in ▶ Document from a Template      (007)
                              ├── sent as ──▶ Submissions and reviews        (002)
                              └── becomes ──▶ Publication ── also a ──▶ Reference (005)
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Register and manage what I am writing (Priority: P1)

A researcher records each thing they are writing or plan to write: its title, what kind of
manuscript it is, its abstract and keywords, where it is headed, when it is due, and where
its files are. They list everything they have in progress, find a manuscript again, change
its details, and remove one they no longer need.

**Why this priority**: Knowing what one is writing, of what kind, for where, and by when is
the minimum, and every other story hangs from this record. With only this story a
researcher has a register of their writing.

**Independent Test**: Create a paper and a thesis, fill in their details, list manuscripts
filtered by kind and ordered by deadline, change a title, and delete one.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a manuscript with a title and a
   kind, **Then** it is stored with a unique identifier and the stage "idea".
2. **Given** a manuscript, **When** the researcher records its abstract, keywords,
   language, more specific type within its kind (for a paper: journal article, conference
   paper…), target venue, deadline, and the location of its files, **Then** they are saved
   and shown.
3. **Given** a manuscript created with only a title, **When** no kind is given, **Then** it
   is stored as a paper.
4. **Given** several manuscripts, **When** the researcher lists them, **Then** each is shown
   with its kind, stage, deadline, and title, and the list can be filtered by kind, stage,
   author, venue, tag, and deadline, and ordered by deadline, title, or last change.
5. **Given** manuscripts, **When** the researcher searches for words, **Then** those whose
   title, abstract, or keywords contain them are shown.
6. **Given** a manuscript, **When** the researcher views it, **Then** all its details are
   shown with its authors, current stage, latest version, parts, and what it is linked to.
7. **Given** a manuscript, **When** the researcher changes any detail, **Then** only that
   detail changes.
8. **Given** a manuscript, **When** the researcher changes its kind, **Then** the tool shows
   which details no longer apply and which become available, and requires confirmation.
9. **Given** a manuscript with versions, parts, or links, **When** the researcher deletes
   it, **Then** the tool lists what it has and what refers to it, and requires explicit
   confirmation; the manuscript's files are never deleted.
10. **Given** a manuscript that has been published, **When** the researcher deletes it,
    **Then** the tool warns that it is a published work and asks for confirmation a second
    time.
11. **Given** a missing title, an unknown kind, a deadline in an invalid form, or a file
    location that does not exist, **When** the researcher saves, **Then** the tool rejects
    it and reports every problem together.
12. **Given** a manuscript, **When** the researcher duplicates it, **Then** a new manuscript
    is created with the same details, authors, and links, at the stage "idea", without
    versions.

---

### User Story 2 - Follow a manuscript from idea to publication (Priority: P2)

A researcher moves a manuscript through its life: idea, outlining, drafting, revising,
submitted, under review, accepted, published — or abandoned. Each move is dated and kept.
They see at a glance where everything stands, what is due soon, and how long each
manuscript has been in its current stage.

**Why this priority**: The stage is the single most asked-about fact of any piece of
writing — by supervisors, co-authors, and the researcher themselves.

**Independent Test**: Move a manuscript through four stages, view its history with dates,
list manuscripts grouped by stage, and see which are due within 30 days.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher changes its stage, **Then** the new stage
   and the date are recorded, and the earlier stage and how long it lasted are kept in its
   history.
2. **Given** a manuscript, **When** the researcher views its history, **Then** stage
   changes, versions, and changes of venue or deadline are shown in order of time.
3. **Given** manuscripts at different stages, **When** the researcher asks for an overview,
   **Then** they are shown grouped by stage, with the number in each.
4. **Given** manuscripts with deadlines, **When** the researcher asks what is due within a
   period, **Then** those manuscripts are listed in date order, overdue ones first.
5. **Given** a manuscript, **When** the researcher moves it backwards (from submitted to
   revising), **Then** the move is accepted and recorded like any other.
6. **Given** a manuscript, **When** the researcher abandons it, **Then** a reason is
   required, and it leaves the lists of work in progress unless asked for.
7. **Given** an abandoned manuscript, **When** the researcher resumes it, **Then** it
   returns to the stage it had before.
8. **Given** a thesis, **When** its stages are shown, **Then** "accepted" reads "defended"
   and "published" reads "deposited", the stages being otherwise the same.
9. **Given** a manuscript moved to "published", **When** the move is made, **Then** the tool
   asks for the publication details, or records that they are still to be given.
10. **Given** a manuscript whose deadline changes, **When** the new date is saved, **Then**
    the earlier date is kept in the history.
11. **Given** a manuscript that has stayed in one stage longer than a period the researcher
    has set, **When** the overview is shown, **Then** it is marked as stalled.
12. **Given** a stage that does not exist, **When** the researcher sets it, **Then** the
    tool rejects it and lists the valid stages.

---

### User Story 3 - Record authors and contributions (Priority: P3)

A researcher records who the authors of a manuscript are, in order, who the corresponding
author is, and what each contributed — conceived the study, ran the experiments, wrote the
first draft. They add people to thank who are not authors. From this the tool can write the
author list, the affiliations, and the contribution statement that venues ask for.

**Why this priority**: Authorship is credit, and its order and contributions are asked for
by almost every venue. It builds on the staff register and on the manuscript record.

**Independent Test**: Give a manuscript three authors in order, mark one as corresponding,
record two contribution roles each, add an acknowledged person, and produce the
contribution statement.

**Acceptance Scenarios**:

1. **Given** a manuscript and people in the staff register, **When** the researcher sets
   its authors in order, **Then** the manuscript shows them in that order with their
   affiliations.
2. **Given** an author who is not in the staff register, **When** the researcher adds them
   by name and affiliation, **Then** they are accepted, and the tool offers to add them to
   the register.
3. **Given** authors, **When** the researcher moves one to another position, **Then** the
   order changes and the earlier order is kept in the history.
4. **Given** authors, **When** the researcher marks one or more as corresponding, or marks
   authors as having contributed equally, **Then** this is shown with the author list.
5. **Given** an author, **When** the researcher records their contributions from the
   standard roles (conceptualization, methodology, software, investigation, data curation,
   analysis, writing the original draft, review and editing, visualization, supervision,
   funding acquisition, project administration), **Then** they are stored.
6. **Given** recorded contributions, **When** the researcher asks for the contribution
   statement, **Then** a text listing each role and who performed it is produced.
7. **Given** a manuscript, **When** the researcher adds people or organizations to
   acknowledge, with what for, **Then** they are stored separately from the authors.
8. **Given** a manuscript supported by grants, **When** the researcher asks for the
   acknowledgements, **Then** the acknowledged people and the funders are both included.
9. **Given** the same person added twice as an author, **When** the researcher saves,
   **Then** the tool rejects it.
10. **Given** a manuscript with no corresponding author, **When** it is moved to
    "submitted", **Then** the tool warns.
11. **Given** an author removed from the staff register later, **When** the manuscript is
    viewed, **Then** the author's name and affiliation as recorded remain.
12. **Given** a person, **When** the researcher asks for what they authored, **Then** every
    manuscript they are an author of is listed with their position.

---

### User Story 4 - Keep versions (Priority: P4)

At meaningful moments — the first full draft, the version sent to the supervisor, the
version submitted — the researcher records a version: a number, a label, a note of what
changed, and a fingerprint of the manuscript's files at that moment. Later they can say
exactly which version was sent where, and whether the files they have now are still that
version.

**Why this priority**: "Which version did the reviewers see?" needs an answer months later.
Versions give one without the tool having to store the text.

**Independent Test**: Record two versions with labels, list them, change the files, and
confirm the tool reports that the files no longer match the latest version.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher records a version with a note of what
   changed, **Then** it receives the next number in sequence, the date, and the stage the
   manuscript was at.
2. **Given** a manuscript with files, **When** a version is recorded, **Then** a fingerprint
   and the size of the files are kept with it, along with the word count when the files
   can be read as text.
3. **Given** a version, **When** the researcher gives it a label ("sent to supervisor",
   "camera-ready"), **Then** the version can be found by that label.
4. **Given** versions, **When** the researcher lists them, **Then** each shows its number,
   label, date, stage, word count, and note.
5. **Given** a version, **When** the researcher asks whether the current files match it,
   **Then** the tool says they match, or that they have changed since.
6. **Given** two versions, **When** the researcher compares them, **Then** the differences
   in stage, authors, word count, parts, cited references, and reported results are shown.
7. **Given** a version, **When** the researcher tries to change its number or fingerprint,
   **Then** the tool refuses; only the label and the note can be changed.
8. **Given** a version that a submission refers to, **When** the researcher deletes it,
   **Then** the tool refuses and names the submission.
9. **Given** a manuscript with no file location, **When** a version is recorded, **Then** it
   is recorded without a fingerprint, and the tool says so.
10. **Given** files that have not changed since the latest version, **When** the researcher
    records another version, **Then** the tool says nothing changed and asks whether to
    record it anyway.

---

### User Story 5 - Connect a manuscript to the research behind it (Priority: P5)

A researcher ties a manuscript to what it rests on: the research questions it answers, the
references it cites, the results, figures, and tables it reports, the experiments,
datasets, and methods behind them. They can then ask of any manuscript "what does this
stand on?" and, before sending it, whether anything it stands on has changed or is missing.

**Why this priority**: These links are what make a manuscript traceable — the reason to keep
writing and research in the same workspace. They need the other specifications' records to
exist.

**Independent Test**: Link a manuscript to a research question, a bibliography, two results,
and a figure; view what it rests on; then change one result with a newer run and confirm
the readiness check reports it.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher links the research questions and
   hypotheses it addresses, **Then** each shows the manuscript and the manuscript shows
   them.
2. **Given** a manuscript, **When** the researcher links citations or a whole bibliography
   to it, **Then** the manuscript lists what it cites and can export its reference list.
3. **Given** a manuscript, **When** the researcher links the results, figures, and tables it
   reports, **Then** the manuscript lists each with the run it came from.
4. **Given** a manuscript, **When** the researcher asks what it rests on, **Then** the
   questions, citations, results, figures, tables, experiments, datasets, methodologies,
   and grants behind it are shown, grouped by kind.
5. **Given** any record, **When** the researcher asks which manuscripts use it, **Then**
   those manuscripts are listed.
6. **Given** a manuscript, **When** the researcher asks whether it is ready, **Then** the
   tool reports: reported results that a newer run has replaced, figures or tables whose
   files have changed, citations whose reference lacks required details, authors without
   affiliation, a missing abstract or corresponding author, and an approaching or passed
   deadline — or says that nothing was found.
7. **Given** a readiness check with nothing found, **When** it ends, **Then** its outcome
   can be used by another program to allow a next step.
8. **Given** a manuscript written in a document created from a template, **When** readiness
   is checked, **Then** the document's own check is included.
9. **Given** a result linked to a manuscript, **When** the result is deleted, **Then** the
   tool lists the manuscripts reporting it and requires confirmation.
10. **Given** a record that does not exist, **When** the researcher links it, **Then** the
    tool rejects the link.

---

### User Story 6 - Structure a manuscript into parts (Priority: P6)

A researcher breaks a long manuscript into parts — the chapters of a thesis, the sections
of a paper — each with a title, a status, a target length, and, if useful, its own deadline
and the person writing it. They see how much of the whole is done. A thesis made of
published articles is assembled by making other manuscripts its parts.

**Why this priority**: A thesis or a long report is not written as one thing, and "how far
along is it?" is answered part by part. Shorter manuscripts do not need this, so it comes
after the stories everyone uses.

**Independent Test**: Give a thesis five chapters with target lengths, set their statuses,
make one chapter an existing paper, and view the progress of the whole.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher adds parts in order, each with a title,
   **Then** the manuscript shows its outline.
2. **Given** a part, **When** the researcher adds parts beneath it, **Then** the outline
   shows them nested, numbered as they will appear.
3. **Given** a part, **When** the researcher sets its status (not started, outlined,
   drafting, drafted, revised, final), target length, deadline, the person writing it, and
   the location of its file, **Then** these are saved.
4. **Given** parts with statuses and targets, **When** the researcher asks for the
   manuscript's progress, **Then** the number of parts at each status, the current length
   against the target for each part and overall, and the parts that are late are shown.
5. **Given** parts whose files can be read as text, **When** progress is shown, **Then**
   current lengths are counted from the files; otherwise the researcher can enter them.
6. **Given** a manuscript and another manuscript, **When** the researcher makes the second a
   part of the first, **Then** the outline shows it with its own stage, and the part's
   status follows that stage.
7. **Given** a part, **When** the researcher moves it to another position or under another
   part, **Then** the outline and the numbering follow.
8. **Given** a manuscript made a part of itself, directly or through other manuscripts,
   **When** the researcher saves, **Then** the tool rejects it.
9. **Given** a part with parts beneath it, **When** the researcher removes it, **Then** the
   tool lists what is beneath and asks whether to remove or keep those one level up.
10. **Given** a manuscript created from a template, **When** its document is created,
    **Then** the template's sections become the manuscript's parts unless parts already
    exist.
11. **Given** a target length that is zero or negative, or a part without a title, **When**
    the researcher saves, **Then** the tool rejects it.

---

### User Story 7 - Record publication and list my output (Priority: P7)

When a manuscript is published, the researcher records where and when: the venue, the
date, the volume and pages, its persistent identifier, its licence, and whether it is
openly accessible. The published work becomes a reference in their own library, so they can
cite it like any other. They can then list everything they have published, in the form a
CV, a programme, or a funder asks for.

**Why this priority**: Publication is the end of the road, and the list of publications is
what a researcher is most often asked to produce. It comes last because it needs
manuscripts that have travelled the whole road.

**Independent Test**: Record the publication of a manuscript, confirm a matching reference
exists in the library and is linked to it, and produce a list of publications by year in a
citation style.

**Acceptance Scenarios**:

1. **Given** an accepted manuscript, **When** the researcher records its publication with a
   venue and a date, **Then** its stage becomes "published" and the details are shown.
2. **Given** a publication, **When** the researcher records its volume, issue, pages,
   persistent identifier, web address, licence, and whether it is openly accessible,
   **Then** they are saved.
3. **Given** a persistent identifier and a network connection, **When** the researcher asks
   the tool to fill in the publication details from it, **Then** the details are fetched,
   shown, and saved on acceptance, as for any reference.
4. **Given** a recorded publication, **When** it is saved, **Then** a reference for the
   published work exists in the library, linked to the manuscript; if one already exists,
   the tool links it instead of creating a second.
5. **Given** a manuscript first shared as a preprint and later published, **When** both are
   recorded, **Then** the manuscript shows both, and they are versions of the same work.
6. **Given** published manuscripts, **When** the researcher asks for their publication list,
   **Then** they are listed in a chosen citation style, grouped by year or by kind, newest
   first, with the researcher's own name marked.
7. **Given** a publication list, **When** the researcher filters it by period, kind,
   project, or grant, **Then** only matching publications are shown.
8. **Given** a publication list, **When** the researcher asks to include work in progress,
   **Then** submitted and accepted manuscripts are added under their own headings.
9. **Given** a publication list, **When** the researcher exports it, **Then** a document, or
   a bibliography file, is produced.
10. **Given** a publication date in the future, an identifier in an invalid form, or a
    publication for a manuscript that is abandoned, **When** the researcher saves, **Then**
    the tool rejects it.
11. **Given** a published manuscript, **When** the researcher records a correction or a
    retraction with its date and note, **Then** it is shown with the publication and in
    the publication list.

---

### Edge Cases

- Two manuscripts have the same title: both are accepted; identifiers tell them apart, and
  the tool mentions the other when the second is created.
- A manuscript has no authors yet: it is accepted; the readiness check reports it.
- A manuscript's files are moved or deleted outside the tool: the manuscript is kept; the
  tool reports the files as missing when it next needs them, and never guesses a new place.
- A manuscript's file location is a directory with many files: the fingerprint and word
  count cover the files the researcher has said belong to it, or all text files otherwise.
- Files cannot be read as text (a word-processor file): versions are recorded with a
  fingerprint and size, without a word count, and the tool says so.
- A deadline passes while a manuscript is still being drafted: it is shown as overdue
  everywhere it appears; nothing changes by itself.
- A manuscript is moved to "published" without ever having been "submitted": it is accepted
  (a technical report is simply published), with no warning for kinds that have no review.
- A manuscript is rejected by a venue: its stage returns to "revising", the target venue is
  cleared, and the rejection is kept in the history (submissions themselves are specified
  in `specs/002-research-lifecycle`).
- The target venue changes: the earlier venue is kept in the history.
- An author is listed whose name is written differently in the staff register and in the
  published work: both forms are kept; the published form is used for the reference.
- Authors are reordered after a version was recorded: the version keeps the order it had.
- A contribution role is recorded for someone who is not an author: the tool rejects it and
  offers to acknowledge the person instead.
- A part's own manuscript is deleted: the part remains as a plain part with the title it
  had.
- A part's deadline is later than the manuscript's: accepted with a warning.
- The sum of the parts' target lengths differs from the manuscript's target: both are
  shown; neither is changed.
- A manuscript is duplicated to prepare a submission elsewhere: the copy is linked to the
  original as derived from it.
- The published work already exists in the library because the researcher added it by hand:
  it is linked, not duplicated.
- A manuscript belongs to two projects: it is one manuscript, listed in both.
- A manuscript's kind is changed from paper to thesis after parts and versions exist: parts
  and versions are kept; only the kind-specific details are affected.
- Text is given in an invalid form — a title that is empty or far too long, keywords
  beyond the allowed number: the tool rejects it, naming each value and the limit.

## Requirements *(mandatory)*

### Functional Requirements

#### Manuscripts

- **FR-001**: Users MUST be able to create, list, view, update, and delete manuscripts, each
  with a title, a kind, an abstract, keywords, a language, a target venue, a deadline, a
  target length, and the location of its files.
- **FR-002**: The system MUST support these kinds of manuscript: paper, thesis, report,
  proposal, book, presentation, and other; a manuscript created without a kind MUST be a
  paper.
- **FR-003**: Users MUST be able to record a more specific type within a kind (for example
  journal article, conference paper, doctoral thesis, technical report, grant application,
  book chapter, poster) and the details particular to it (for a thesis: institution,
  programme, degree, supervisors; for a proposal: funder and call; for a presentation: the
  event).
- **FR-004**: Users MUST be able to filter manuscripts by kind, type, stage, author, venue,
  tag, project, and deadline, order them by deadline, title, stage, or last change, and
  search by words in title, abstract, and keywords.
- **FR-005**: Viewing a manuscript MUST show its details, authors, stage, latest version,
  parts, publication, and everything it is linked to.
- **FR-006**: Changing a manuscript's kind MUST show which details no longer apply and which
  become available, require confirmation, and keep its authors, versions, parts, and links.
- **FR-007**: Before deleting a manuscript, the system MUST list its versions, parts, and
  links and what refers to it, and MUST require explicit confirmation, twice for a
  published manuscript. Deleting a manuscript MUST NOT delete its files.
- **FR-008**: Users MUST be able to duplicate a manuscript; the copy MUST carry the details,
  authors, and links, start at the stage "idea" with no versions, and be recorded as
  derived from the original.
- **FR-009**: The system MUST record a manuscript's files by location only, without taking a
  copy, and MUST report them as missing when they cannot be found.

#### Stages and deadlines

- **FR-010**: The system MUST track each manuscript's stage: idea, outlining, drafting,
  revising, submitted, under review, accepted, published, or abandoned.
- **FR-011**: Any move between stages MUST be allowed, forwards or backwards; each MUST be
  recorded with its date, and the history MUST show how long each stage lasted.
- **FR-012**: Abandoning a manuscript MUST require a reason; resuming it MUST return it to
  the stage it had before.
- **FR-013**: For a thesis, the system MUST show "accepted" as "defended" and "published" as
  "deposited"; the stages MUST otherwise be the same for every kind.
- **FR-014**: The system MUST keep the history of changes to a manuscript's deadline and
  target venue.
- **FR-015**: Users MUST be able to see manuscripts grouped by stage with counts, and those
  due within a chosen period, overdue first; abandoned and published manuscripts MUST be
  left out of work in progress unless asked for.
- **FR-016**: Users MUST be able to set a period after which a manuscript that has not
  changed stage is shown as stalled.
- **FR-017**: A manuscript's deadline MUST appear in the workspace's view of what is due.

#### Authors and contributions

- **FR-018**: Users MUST be able to set a manuscript's authors in order, each either a
  person from the staff register or a name with an affiliation, and to reorder them; the
  same person MUST NOT appear twice.
- **FR-019**: Users MUST be able to mark authors as corresponding and as having contributed
  equally.
- **FR-020**: Users MUST be able to record each author's contributions from a standard set
  of roles: conceptualization, methodology, software, validation, formal analysis,
  investigation, resources, data curation, writing the original draft, review and editing,
  visualization, supervision, project administration, and funding acquisition.
- **FR-021**: The system MUST produce, for a manuscript, the author list with affiliations,
  a contribution statement, and the acknowledgements including acknowledged people and
  supporting funders.
- **FR-022**: Users MUST be able to record people and organizations to acknowledge, with
  what for, separately from authors; a contribution role MUST NOT be recorded for someone
  who is not an author.
- **FR-023**: A manuscript MUST keep its authors' names and affiliations as recorded when
  those people are later changed in, or removed from, the staff register.
- **FR-024**: Users MUST be able to list the manuscripts a person is an author of, with
  their position in each.
- **FR-025**: The system MUST warn when a manuscript without a corresponding author is
  moved to "submitted".

#### Versions

- **FR-026**: Users MUST be able to record a version of a manuscript with a note of what
  changed and an optional label; versions MUST be numbered in sequence and never
  renumbered or reused.
- **FR-027**: Each version MUST record the date, the manuscript's stage, its authors in
  order, its parts, what it cites and reports, and — when the manuscript has files — a
  fingerprint and size of those files and, when they can be read as text, a word count.
- **FR-028**: A version's number, date, and fingerprint MUST NOT be changeable; its label
  and note MAY be.
- **FR-029**: Users MUST be able to ask whether the current files match a version, and to
  compare two versions by stage, authors, word count, parts, citations, and reported
  results.
- **FR-030**: The system MUST refuse to delete a version that a submission or a publication
  refers to.
- **FR-031**: When the files have not changed since the latest version, the system MUST say
  so before recording another.

#### Links and readiness

- **FR-032**: Users MUST be able to link a manuscript to the research questions and
  hypotheses it addresses, to citations and bibliographies, to the results, figures, and
  tables it reports, and to experiments, datasets, methodologies, and grants; every link
  MUST be visible from both ends.
- **FR-033**: The system MUST show, for a manuscript, everything it rests on, grouped by
  kind, with the run behind each reported result, figure, and table; and, for any record,
  the manuscripts that use it.
- **FR-034**: Users MUST be able to export the reference list of a manuscript.
- **FR-035**: Users MUST be able to check whether a manuscript is ready; the check MUST
  report reported results that a newer run has replaced, figures and tables whose files
  have changed, citations whose reference lacks required details, authors without
  affiliation, a missing abstract, a missing corresponding author, missing files, a deadline
  that is near or passed, and — when the manuscript is written in a document created from
  a template — the findings of that document's own check.
- **FR-036**: The outcome of the readiness check MUST distinguish "nothing found" from
  "problems found" in a way another program can act on, and the check MUST change nothing.

#### Parts

- **FR-037**: Users MUST be able to give a manuscript an ordered outline of parts, nested to
  any depth, each with a title, a status (not started, outlined, drafting, drafted, revised,
  final), a target length, a deadline, the person writing it, and the location of its file.
- **FR-038**: Users MUST be able to add, rename, move, and remove parts; removing a part
  that has parts beneath it MUST ask whether to remove those or keep them one level up.
- **FR-039**: Users MUST be able to make another manuscript a part of a manuscript; such a
  part's status MUST follow that manuscript's stage, and the system MUST reject a
  manuscript made a part of itself directly or indirectly.
- **FR-040**: The system MUST show a manuscript's progress: parts at each status, current
  length against target for each part and overall, and parts that are late; lengths MUST be
  counted from files that can be read as text and MUST otherwise be enterable by hand.
- **FR-041**: When a document is created for a manuscript from a template, the template's
  sections MUST become the manuscript's parts unless it already has parts.

#### Publication

- **FR-042**: Users MUST be able to record the publication of a manuscript: venue, date,
  volume, issue, pages, persistent identifier, web address, licence, and whether it is
  openly accessible; recording it MUST set the stage to "published".
- **FR-043**: Users MUST be able to fill in publication details from a persistent
  identifier, under the same rules as online lookup of references in
  `specs/005-literature`.
- **FR-044**: A recorded publication MUST have a corresponding reference in the library,
  linked to the manuscript; the system MUST link an existing reference for the same work
  instead of creating a second one.
- **FR-045**: A manuscript MUST be able to have more than one public form (a preprint and a
  published article), recorded as versions of the same work.
- **FR-046**: Users MUST be able to record a correction or a retraction of a published
  manuscript, with its date and a note.
- **FR-047**: Users MUST be able to produce a list of their publications in a chosen
  citation style, grouped by year or kind, newest first, with their own name marked,
  filtered by period, kind, project, and grant, optionally including submitted and accepted
  work under separate headings, and export it as a document or a bibliography file.
- **FR-048**: The system MUST reject a publication dated in the future or recorded for an
  abandoned manuscript.

#### Common behavior

- **FR-049**: Every value a user supplies for manuscripts, stages, authors, contributions,
  versions, parts, links, and publications MUST be validated before anything is stored;
  invalid input MUST change nothing and MUST be reported per value, all together, with what
  is expected.
- **FR-050**: Manuscripts MUST support tags, notes, and free links to any other record, and
  MUST belong to projects, as every record of the workspace does.
- **FR-051**: Every creation, change, deletion, stage change, version, and publication
  MUST be recorded in the workspace's audit trail; checks and lists MUST NOT.
- **FR-052**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-053**: The word "draft" MUST be accepted wherever "manuscript" is, so that commands
  and documents written for earlier specifications remain valid.
- **FR-054**: Every command MUST have built-in help, and manuscripts, stages, authors and
  contributions, versions, parts, readiness, and publications MUST each have a usage guide
  with examples.

### Key Entities *(include if feature involves data)*

- **Manuscript**: A piece of writing the researcher authors, tracked from idea to
  publication. Has a kind and type, a title, abstract, keywords, language, target venue,
  deadline, target length, file location, a stage with its history, authors, versions,
  parts, links, and possibly publications. Also called a "draft".
- **Kind / Type**: What sort of manuscript it is (paper, thesis, report, proposal, book,
  presentation, other) and, within that, more precisely (journal article, technical
  report…).
- **Stage**: Where a manuscript stands on the way to publication, with the date of each
  change.
- **Authorship**: One author of one manuscript: their position, whether corresponding,
  whether an equal contributor, their name and affiliation as recorded, and their
  contributions.
- **Contribution Role**: One of the standard things an author can have done for a
  manuscript.
- **Acknowledgement**: A person or organization thanked in a manuscript without being an
  author, and what for.
- **Version**: A numbered moment in a manuscript's history: date, stage, label, note,
  authors, parts, citations and reported findings at that moment, and the fingerprint, size,
  and word count of its files.
- **Part**: A chapter or section of a manuscript, in an outline, with a status, target
  length, deadline, and writer; it may itself be another manuscript.
- **Publication**: The public form of a manuscript: venue, date, identifier, licence, and
  access; with any correction or retraction. Mirrored by a reference in the library.
- **Readiness Finding**: Something the readiness check found that should be dealt with
  before the manuscript is sent.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can register a manuscript with its title, kind, and deadline in under
  30 seconds, and find any manuscript among 500 in under 2 seconds.
- **SC-002**: A user can tell the stage and deadline of everything they are writing from a
  single view in under 5 seconds.
- **SC-003**: For any manuscript, a user can say on which date it entered each stage and how
  long it stayed there, in under 30 seconds.
- **SC-004**: A user can set three authors in order with their contributions and obtain the
  contribution statement in under 3 minutes.
- **SC-005**: For any recorded version, the tool correctly says whether the current files
  match it in 100% of cases.
- **SC-006**: For any submission or publication, a user can identify the exact version
  concerned in under 30 seconds.
- **SC-007**: For any result, figure, or table reported in a manuscript, a user can reach
  the run that produced it in under 1 minute; for any record, the manuscripts that use it
  are listed in under 5 seconds.
- **SC-008**: The readiness check reports 100% of replaced results, changed figure files,
  missing files, and missing corresponding authors, and reports nothing for a manuscript
  that has none of these.
- **SC-009**: A user can see the progress of a thesis with 8 chapters, part by part and
  overall, in a single view in under 5 seconds.
- **SC-010**: Recording a publication never produces a second reference for a work already
  in the library, in 100% of cases.
- **SC-011**: A user can produce their full publication list in a chosen citation style in
  under 30 seconds.
- **SC-012**: 100% of invalid inputs are rejected before any data changes, each with the
  invalid value named.
- **SC-013**: Deleting a manuscript never deletes or alters any of its files, in 100% of
  cases.
- **SC-014**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns what the researcher writes.** User Story 4 of
  `specs/001-research-workspace` (draft papers, FR-026 to FR-029) is replaced by this one;
  that specification keeps a short pointer in its place.
- **"Manuscript" is the name chosen for "papers, reports, etc."** "Draft" remains accepted
  everywhere as another word for it, so other specifications need no rewording.
- **"Reports" here are documents the researcher writes** (technical reports, lab reports,
  project reports). The tool's own accounts of activity are specified in
  `specs/006-reports` and are not manuscripts; a researcher who wants to keep working on
  one registers a manuscript and uses the exported document as its file.
- **The tool does not hold the text.** A manuscript records where its files are. Writing,
  formatting, and keeping the history of the text itself are done with the researcher's
  own tools; a version here is a recorded moment with a fingerprint, not a stored copy, and
  cannot be restored from the workspace.
- **Starting and laying out the files is specified in `specs/007-templates`**; a manuscript
  works with or without a template.
- **Submissions, reviewers' comments, and responses stay in `specs/002-research-lifecycle`**
  (its User Story 8). The request was to move the content of the first specification; a
  submission refers to a manuscript and one of its versions, and moves the manuscript
  between the stages defined here.
- **Results, figures, tables, and the flag for a replaced result are specified in
  `specs/004-experiments`**; citations, bibliographies, and the reference created for a
  publication in `specs/005-literature`; research questions, staff, and the audit trail in
  `specs/001-research-workspace`; projects and the view of what is due in
  `specs/003-research-projects`. This specification links to them and does not redefine
  them.
- **Stages are the same for every kind**, with different wording for theses. Kinds without
  peer review simply skip the stages they do not use.
- **Contribution roles follow the taxonomy most venues ask for**; the list is fixed in this
  specification and can be extended later.
- **Word counts are approximate**, counted from files that can be read as text, ignoring the
  marks of the writing format as far as it allows.
- **A publication list covers manuscripts recorded in the workspace.** Works published
  before using the tool appear once the researcher registers them as published manuscripts,
  which can be done from their identifiers.
- **Single researcher.** Authors are people named on a manuscript; they are not users of the
  workspace, and there is no shared editing.
