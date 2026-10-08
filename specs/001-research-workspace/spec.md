<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Research Workspace

**Feature Branch**: `001-research-workspace`

**Created**: 2026-10-08

**Status**: Draft

**Amended**: 2026-10-08 — added research questions and hypotheses (User Story 2), results,
figures and tables (User Story 6), and online lookup of paper details (User Story 1).
Existing stories were renumbered; requirement numbers were kept and new ones appended.
Experiments, pipelines, runs, results, figures, and tables (User Stories 5 and 6) were then
moved to `specs/004-experiments`; pointers remain in their place. Papers, citations, online
lookup, and bibliographic research (User Stories 1 and 3) were moved to
`specs/005-literature`; User Story 1 now covers the workspace itself. Draft papers (User
Story 4) were moved to `specs/008-manuscripts`, and courses (User Story 11) to
`specs/011-courses-roadmaps`. Staff (User Story 9) moved to `specs/012-people`,
and research questions and hypotheses (User Story 2) to `specs/013-ideas-questions`.
Methodologies (part of User Story 7) moved to `specs/014-conventions-methods`.
The workspace itself, the behaviour shared by all records, input validation, and the audit
trail (User Stories 1 and 10) moved to `specs/000-foundation`. What remains specified here
is datasets (User Story 7) and environment and reproducibility (User Story 8).

**Input**: User description: "Develop trcli, a researcher tool for modern research; it can create/crud and track papers, save citations, crud drafts papers, crud experiments(with pipelines, including manual steps), crud methodologies, crud data/datasets, crud staffs, telemetry, audit, environment, reproducibility, crud bibliographic research, courses, etc"

## Overview

TRCLI (The Research CLI) is a command-line workspace in which a researcher keeps everything
that makes up a research project in one place and in one consistent shape: the questions
they are trying to answer, the papers they read, the citations they save, the literature
reviews they run, the papers they are writing, the experiments they execute and the results
those produce, the methodologies and datasets those experiments rely on, the
people involved, and the courses that feed the work. Every record can be created, viewed,
changed, and removed, and records can be linked to one another so that a claim in a draft
can be traced back to the result, experiment, dataset, method, and environment that produced
it, and forward to the research question it helps answer.

The product is delivered as a series of independently usable slices, ordered below by
priority. Each slice is valuable on its own.

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Create and use a workspace (Priority: P1)

> **Moved (2026-10-08)**: the workspace and the behaviour shared by all records are now
> specified in `specs/000-foundation` (its User Stories 1 to 5). This story is kept as a
> pointer so that the numbering of the other stories does not change.

---

### User Story 2 - Frame research questions and hypotheses (Priority: P2)

> **Moved (2026-10-08)**: research questions and hypotheses are now specified in
> `specs/013-ideas-questions`, together with ideas, topics, and the review that keeps them
> from being forgotten. This story is kept as a pointer so that the numbering of the other
> stories does not change.

---

### User Story 3 - Run bibliographic research (Priority: P3)

> **Moved (2026-10-08)**: bibliographic research (literature reviews) is now specified in
> `specs/005-literature` (its User Stories 6 and 7). This story is kept as a pointer so
> that the numbering of the other stories does not change.

---

### User Story 4 - Write and manage draft papers (Priority: P4)

> **Moved (2026-10-08)**: papers, reports, theses, and everything else the researcher
> writes are now specified in `specs/008-manuscripts`, where a "draft" is called a
> manuscript. This story is kept as a pointer so that the numbering of the other stories
> does not change.

---

### User Story 5 - Define and run experiments with pipelines (Priority: P5)

> **Moved (2026-10-08)**: experiments, pipelines, manual steps, and runs are now specified
> in `specs/004-experiments` (its User Stories 1 to 3). This story is kept as a pointer so
> that the numbering of the other stories does not change.

---

### User Story 6 - Record results, figures, and tables (Priority: P6)

> **Moved (2026-10-08)**: results, figures, and tables are now specified in
> `specs/004-experiments` (its User Story 5). This story is kept as a pointer so that the
> numbering of the other stories does not change.

---

### User Story 7 - Manage methodologies and datasets (Priority: P7)

> **Moved in part (2026-10-08)**: methodologies are now specified in
> `specs/014-conventions-methods`, with their steps, origin, versions, and adaptations.
> Datasets remain specified here; scenarios 1 and 2 below are superseded by that
> specification.

A researcher records the methodologies they use (name, description, procedure, references)
and the datasets they work with (name, description, origin, license, location, version,
integrity fingerprint), and links both to the experiments that use them.

**Why this priority**: Methods and data explain how a result was obtained. They are
reusable across experiments and are required before reproducibility can be assessed.

**Independent Test**: Create a methodology and a dataset, link both to an experiment, and
view the experiment showing which method and which dataset version it uses.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a methodology with a name and a
   description, **Then** it is stored and can be listed, viewed, changed, and removed.
2. **Given** a methodology, **When** the researcher links papers that describe it, **Then**
   the methodology shows those references.
3. **Given** a workspace, **When** the researcher registers a dataset with a name and a
   location, **Then** it is stored with a version and an integrity fingerprint.
4. **Given** a registered dataset whose content has changed, **When** the researcher asks
   the tool to verify it, **Then** the tool reports that it no longer matches and offers to
   record a new version.
5. **Given** an experiment, **When** the researcher links a methodology and a dataset
   version, **Then** runs of that experiment record which ones were used.
6. **Given** a dataset whose location does not exist, **When** the researcher tries to
   register it, **Then** the tool rejects it with a clear message.

---

### User Story 8 - Capture environment and verify reproducibility (Priority: P8)

A researcher captures the environment in which an experiment ran (the machine, the
operating system, the tools and their versions, and the settings used) and later asks
whether a past run can be reproduced: the tool compares the current environment, data, and
method against what was recorded and reports every difference. The researcher can also
produce a reproducibility package that a colleague can use to repeat the work.

**Why this priority**: Reproducibility is the quality bar of modern research, but it can
only be checked once experiments, methods, and datasets are recorded.

**Independent Test**: Run an experiment, capture its environment, change one thing in the
environment or data, and run a reproducibility check that reports exactly that difference.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher captures the current environment,
   **Then** a named environment snapshot is stored.
2. **Given** an experiment run, **When** it starts, **Then** the environment snapshot in
   effect is recorded with the run automatically.
3. **Given** a past run, **When** the researcher requests a reproducibility check, **Then**
   the tool reports "reproducible" or lists each difference in environment, dataset version,
   methodology, and pipeline definition.
4. **Given** a past run, **When** the researcher requests a reproducibility package,
   **Then** a single shareable bundle is produced describing the pipeline, method, dataset
   references, environment, and results needed to repeat the run.
5. **Given** two environment snapshots, **When** the researcher compares them, **Then** the
   differences are listed item by item.
6. **Given** an environment containing secret values, **When** a snapshot is captured,
   **Then** secret values are not stored.

---

### User Story 9 - Manage staff and responsibilities (Priority: P9)

> **Moved (2026-10-08)**: the register of people and their assignments is now specified in
> `specs/012-people`, with positions, supervision, meetings, and handover. This story is
> kept as a pointer so that the numbering of the other stories does not change.

---

### User Story 10 - Review the audit trail and telemetry (Priority: P10)

> **Moved (2026-10-08)**: the audit trail and local measurements are now specified in
> `specs/000-foundation` (its User Story 6). This story is kept as a pointer so that the
> numbering of the other stories does not change.

---

### User Story 11 - Track courses (Priority: P11)

> **Moved (2026-10-08)**: courses are now specified in `specs/011-courses-roadmaps`, together
> with programmes, curricula, graduate roadmaps, and learning roadmaps. This story is kept
> as a pointer so that the numbering of the other stories does not change.

---

### Edge Cases

- A dataset is very large or temporarily unreachable: registration and verification report
  progress, and an unreachable dataset is reported as unreachable rather than as changed.

## Requirements *(mandatory)*

### Functional Requirements

#### Workspace and common behavior

*FR-001 to FR-011 moved to `specs/000-foundation` (requirement groups "Workspace", "Records",
"Output and interaction", and "Help and documentation").*

#### Input validation

*FR-012 to FR-015 moved to `specs/000-foundation` (FR-020 to FR-027 there).*

#### Papers and citations

*FR-016 to FR-021 moved to `specs/005-literature` (requirement groups "Reference library",
"Import and export", and "Citations and bibliographies").*

#### Bibliographic research

*FR-022 to FR-025 moved to `specs/005-literature` (requirement group "Literature reviews").*

#### Draft papers

*FR-026 to FR-029 moved to `specs/008-manuscripts` (requirement groups "Manuscripts",
"Stages and deadlines", "Versions", and "Links and readiness").*

#### Experiments and pipelines

*FR-030 to FR-038 moved to `specs/004-experiments` (requirement groups "Experiments",
"Pipelines and steps", and "Runs").*

#### Methodologies and datasets

- **FR-039**: *Moved to `specs/014-conventions-methods` (FR-011 to FR-017 there).*
- **FR-040**: Users MUST be able to register datasets with a name, description, origin,
  license, location, version, and integrity fingerprint, without the tool taking a copy of
  the data itself.
- **FR-041**: Users MUST be able to verify a dataset against its recorded fingerprint and
  record a new version when its content has changed.
- **FR-042**: Each run MUST record which methodology and which dataset versions it used.

#### Environment and reproducibility

- **FR-043**: Users MUST be able to capture a named snapshot of the working environment
  (machine, operating system, tools and versions, and settings) and compare two snapshots.
- **FR-044**: The system MUST record the environment in effect with every run
  automatically.
- **FR-045**: The system MUST NOT store secret values when capturing an environment.
- **FR-046**: Users MUST be able to check whether a past run is reproducible; the check MUST
  compare the current environment, dataset versions, methodology, and pipeline definition
  with those recorded and list every difference.
- **FR-047**: Users MUST be able to produce a single shareable reproducibility package for a
  run, containing what a colleague needs to repeat it.

#### Staff

*FR-048 to FR-050 moved to `specs/012-people` (requirement groups "People and positions",
"Assignments and overview", and "Handover").*

#### Audit and telemetry

*FR-051 to FR-055 moved to `specs/000-foundation` (FR-046 to FR-054 there).*

#### Courses

*FR-056 and FR-057 moved to `specs/011-courses-roadmaps` (requirement group "Courses").*

#### Research questions and hypotheses

*FR-058 to FR-064 moved to `specs/013-ideas-questions` (FR-026 to FR-034 there).*

#### Results, figures, and tables

*FR-065 to FR-071 moved to `specs/004-experiments` (FR-038 to FR-044 there).*

#### Online lookup of paper details

*FR-072 to FR-076 moved to `specs/005-literature` (FR-016 to FR-020 there).*

### Key Entities *(include if feature involves data)*

- **Workspace, Record, Tag, Note, Link, Setting, Audit Entry, Measurement**: specified in
  `specs/000-foundation`.
- **Research Question, Hypothesis, Idea, Topic**: specified in `specs/013-ideas-questions`.
- **Paper (Reference), Citation, Bibliography, Literature Review**: specified in
  `specs/005-literature`. Records here link to them (a draft uses citations; a research
  question is addressed by papers).
- **Draft (Manuscript), Version**: specified in `specs/008-manuscripts`. Records here link
  to them (a research question is addressed by manuscripts; staff are their authors).
- **Experiment, Pipeline, Step, Run, Result, Figure / Table**: specified in
  `specs/004-experiments`. Records here link to them (a draft reports results; a dataset
  version is used by runs; a hypothesis has results as evidence).
- **Methodology**: specified in `specs/014-conventions-methods`.
- **Dataset**: A body of data known by reference: its origin, license, location, versions,
  and integrity fingerprints.
- **Environment Snapshot**: A recorded description of a working environment at a moment in
  time, free of secret values.
- **Reproducibility Package**: A shareable bundle describing everything needed to repeat a
  run.
- **Staff Member (Person)**: specified in `specs/012-people`.
- **Course**: specified in `specs/011-courses-roadmaps`.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: *Moved to `specs/000-foundation` (SC-001).*
- **SC-002**: *Moved to `specs/005-literature` (SC-003).*
- **SC-003**: *Moved to `specs/005-literature` (SC-004).*
- **SC-004**: *Moved to `specs/000-foundation` (SC-004).*
- **SC-005**: *Moved to `specs/004-experiments` (SC-002).*
- **SC-006**: *Moved to `specs/004-experiments` (SC-004).*
- **SC-007**: *Moved to `specs/004-experiments` (SC-008).*
- **SC-008**: A reproducibility check correctly reports 100% of deliberately introduced
  differences in environment, dataset version, methodology, and pipeline definition.
- **SC-009**: A colleague given only a reproducibility package and access to the referenced
  data can repeat the run without asking the original researcher any question.
- **SC-010**: *Moved to `specs/000-foundation` (SC-009).*
- **SC-011**: *Moved to `specs/000-foundation` (SC-010).*
- **SC-012**: *Moved to `specs/000-foundation` (SC-018).*
- **SC-013**: *Moved to `specs/000-foundation` (SC-013).*

- **SC-014**: *Moved to `specs/005-literature` (SC-002).*
- **SC-015**: *Moved to `specs/000-foundation` (SC-011).*
- **SC-016**: *Moved to `specs/004-experiments` (SC-008).*
- **SC-017**: *Moved to `specs/013-ideas-questions` (SC-004).*
- **SC-018**: *Moved to `specs/004-experiments` (SC-009).*

## Assumptions

- **Projects are specified separately**: Organizing records into typed projects (doctoral,
  master's, capstone, independent research, and others) with milestones and tasks is
  specified in `specs/003-research-projects`. Until it is delivered, a workspace behaves as
  a single project.
- **Single researcher, local workspace**: The tool is used by one person at a time on their
  own machine. There are no user accounts, sign-in, or permissions. Staff members are
  records describing people, not accounts that can log in. Sharing a workspace with
  colleagues happens by sharing its files through whatever means the team already uses;
  simultaneous editing by several people is out of scope.
- **Actor identity**: The "actor" in audit entries and manual-step confirmations is the
  person identified by the machine's current user and the workspace's configured researcher
  name; it is not authenticated.
- **Telemetry means measurements of the research work and local tool usage**, kept inside
  the workspace for the researcher's own benefit. It is not usage reporting to the tool's
  authors, and nothing is sent anywhere.
- **Courses are those the researcher takes or teaches** in connection with the research.
  Building course content, managing enrolment, and grading are out of scope.
- **Works offline, looks up online on request**: Every capability except online lookup
  works without a network connection. Lookup uses freely available public catalogues that
  need no account, contacts them only when the researcher asks, and sends only the
  identifier. Searching catalogues by keyword and downloading full texts are out of scope.
- **Results are entered by the researcher**: The tool records the findings the researcher
  names; it does not extract numbers or figures from run outputs by itself, and it does not
  create plots.
- **Research questions are optional but encouraged**: Records can exist without being linked
  to a question; the tool reports unlinked records rather than forbidding them.
- **Datasets and manuscripts are referenced, not stored**: The tool records where they are,
  which version they are, and their fingerprint. Storing, moving, or backing up the data and
  the manuscript text is out of scope.
- **Writing happens elsewhere**: The tool tracks drafts and their bibliography; it is not a
  text editor or a typesetting system.
- **Automated steps are instructions the user's own machine can carry out**. Scheduling work
  on remote clusters or cloud services is out of scope for this specification.
- **Audit entries are kept for the life of the workspace** and are removed only when the
  workspace itself is deleted.
- **Interface language is English**, with dates shown in an unambiguous international form.
- **"etc." in the request** is read as an expectation that further record types can be added
  later in the same shape (create, list, view, update, delete, link, audit). Further record
  types and capabilities (reading annotations, tasks, backup, lab notebook, submissions,
  funding, ethics, and others) are specified separately in `specs/002-research-lifecycle`.
- **Delivery is incremental**: User stories are delivered in priority order, each as its own
  planned and reviewed slice; this specification describes the whole product so that the
  slices stay consistent with one another.
