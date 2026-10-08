<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Conventions, Methodologies, and the Implicit Side of Research

**Feature Branch**: `014-conventions-methods`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for conventions (from lab, from community), methodologies (from the research, lab, community, literature), all the meta parts of a research that in general is implicit"

## Overview

Much of what makes research work is never written down. Files are named a certain way
"because that is how we do it here". Samples are randomized by a procedure a former student
set up. A threshold of 0.05 is used because the field uses it. Outliers are removed by a
rule somebody decided two years ago, for a reason nobody remembers. A new member learns all
this slowly, by breaking it; a reviewer asks about it and the answer takes a week to
reconstruct; and when the person who knew leaves, it is gone.

This specification is for the **implicit side of research**: the conventions, methods,
assumptions, and decisions that shape the work without appearing in it. It gives each a
place to be written down, says **where it comes from** and **how binding it is**, shows it
to the researcher at the moment it applies, and records when the work knowingly departs
from it.

It **takes over** the methodology content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 7 (methodology part), FR-039 | Methodologies: name, description, procedure, references |

### What is made explicit

| Kind | What it is | Example |
|------|------------|---------|
| **Convention** | An agreed way of doing something where other ways were possible | "Dataset folders are named `<year>-<source>-<version>`" |
| **Methodology** | How a kind of work is carried out, step by step | "Five-fold cross-validation with a held-out test set" |
| **Assumption** | Something the research takes as true without showing it | "Annotators were independent of one another" |
| **Decision** | A choice made between alternatives, with its reason | "We excluded sessions shorter than 30 s, because…" |
| **Checklist** | A list of things a piece of work must satisfy | A reporting guideline for a kind of study; the lab's pre-submission list |

### Where it comes from

Every one of these has an **origin**, because "who says so?" decides how freely it may be
changed:

| Origin | Meaning | Who may change it |
|--------|---------|-------------------|
| **Own research** | Chosen by the researcher for this work | The researcher |
| **Lab** | Agreed in the research group | The group; a member records a deviation |
| **Institution** | Required by the university, programme, or ethics body | Not the researcher |
| **Community** | The practice of the field | Nobody in particular; departing from it needs justifying |
| **Venue or funder** | Required by where the work is sent or who pays | Not the researcher |
| **Literature** | Taken from a published work, which is cited | The researcher, by adapting it and saying how |

### The concepts, in one picture

```text
 Origin (own · lab · institution · community · venue/funder · literature ── cites ──▶ Reference)
    │
    ▼
 Convention ── applies to ──▶ kinds of record · projects · topics
 Methodology ── used by ───▶ Experiment · Literature Review · Model      ── adapted from ──▶ Methodology
 Assumption ── underlies ──▶ Experiment · Model · Result · Manuscript
 Decision ──── concerns ───▶ any record            ── chose among ──▶ Alternatives
 Checklist ─── applied to ─▶ Manuscript · Experiment ── item by item ──▶ Addressed where?

 At the moment of work: "what applies here?"      When departing: Deviation, with a reason
 Together: the Handbook ── shared as a pack ──▶ another workspace
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Write down a convention (Priority: P1)

A researcher writes down a convention the moment they notice one — or the moment they are
told "we always do it this way": what it says, what it applies to, why, where it comes
from, and how binding it is. They add an example of doing it right and one of doing it
wrong. From then on it can be found, and nobody has to ask.

**Why this priority**: Conventions are the most numerous and the most invisible of the
implicit things, and writing one down is the smallest useful act. With only this story a
researcher or a lab has the beginning of a handbook.

**Independent Test**: Record three conventions with different origins and strengths, one
with examples, list them filtered by origin and by what they apply to, and find one by a
word of its text.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a convention with a short title
   and a statement, **Then** it is stored as adopted, with the origin "own research" and the
   strength "should" unless said otherwise.
2. **Given** a convention, **When** the researcher records its origin — own research, lab,
   institution, community, venue or funder, or literature — and, where relevant, the
   reference, document, or person it comes from, **Then** these are saved and shown.
3. **Given** a convention, **When** the researcher sets how binding it is — must, should, or
   may — **Then** conventions can be listed by strength.
4. **Given** a convention, **When** the researcher states what it applies to — kinds of
   record, particular projects, topics, or a described situation — **Then** it is found
   when that kind, project, or topic is asked about.
5. **Given** a convention, **When** the researcher records why it exists, **Then** the
   rationale is shown with it; a convention without one is marked as unexplained.
6. **Given** a convention, **When** the researcher adds examples of following it and of not
   following it, **Then** they are shown with it.
7. **Given** conventions, **When** the researcher lists them filtered by origin, strength,
   category, status, or what they apply to, or searches their text, **Then** only matching
   conventions are shown.
8. **Given** a convention, **When** the researcher puts it in a category — naming, files and
   folders, notation and units, writing, data handling, analysis, authorship, reviewing,
   meetings, or another — **Then** conventions can be browsed by category.
9. **Given** a convention under discussion, **When** it is recorded as proposed, **Then** it
   is shown separately from adopted ones until adopted or rejected, with who decided and
   when.
10. **Given** a convention that no longer holds, **When** the researcher retires it with a
    reason, or replaces it with another, **Then** it is kept in history and the replacement
    is shown.
11. **Given** a convention whose wording changes, **When** it is saved, **Then** the earlier
    wording and the date are kept.
12. **Given** a thought captured in the inbox that is really a convention, **When** the
    researcher sorts it, **Then** it can be turned into one.
13. **Given** a missing statement, an unknown origin or strength, or a title already used,
    **When** the researcher saves, **Then** the tool rejects it and reports every problem
    together.

---

### User Story 2 - Describe how the work is done (Priority: P2)

A researcher records the methodologies their work relies on: what each is for, when to use
it and when not, its steps in order, what it needs and what it produces, how to tell it
went well, and the mistakes people make. They say where it comes from — a published method,
the lab's own procedure, common practice, or something worked out for this research — and,
when they adapt one, exactly how their version differs.

**Why this priority**: The method is what a result's credibility rests on, and what a
methods section must describe. Experiments, reviews, and models already point to
methodologies; this makes what they point to worth reading.

**Independent Test**: Record a methodology from the literature with its reference and five
steps, adapt it into a lab variant stating two differences, link the variant to an
experiment, and view the experiment's method with its lineage.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a methodology with a name and a
   purpose, **Then** it is stored at version 1.
2. **Given** a methodology, **When** the researcher records when to use it, when not to, what
   it needs beforehand, and what it produces, **Then** these are shown with it.
3. **Given** a methodology, **When** the researcher writes its procedure as ordered steps,
   each with what to do and an optional check that it was done right, **Then** the steps are
   stored in order.
4. **Given** a methodology, **When** the researcher records its origin and, for one from the
   literature, the references that define it, **Then** these are shown, and the references
   list the methodology.
5. **Given** a methodology, **When** the researcher records its known pitfalls and its
   limitations, **Then** they are shown with it.
6. **Given** a methodology, **When** the researcher adapts it into a new one, **Then** the
   new methodology records which one it was adapted from and each difference with its
   reason.
7. **Given** an adapted methodology, **When** it is viewed, **Then** its lineage back to the
   original is shown, with the differences introduced at each step.
8. **Given** a methodology, **When** its procedure changes and a new version is recorded
   with a note, **Then** earlier versions remain viewable, and work that used an earlier
   version still shows that version.
9. **Given** an experiment, a literature review, or a model, **When** the researcher links
   the methodology it follows, **Then** the version in effect is recorded with it.
10. **Given** a methodology, **When** the researcher asks where it is used, **Then** the
    experiments, reviews, models, and manuscripts that rely on it are listed, by version.
11. **Given** a methodology with steps, **When** the researcher turns it into the pipeline of
    an experiment, **Then** a step is created for each, to be marked automated or manual.
12. **Given** methodologies, **When** the researcher lists them filtered by origin, kind of
    work, or status, or searches them, **Then** only matching ones are shown.
13. **Given** a methodology in use, **When** the researcher deletes it, **Then** the tool
    refuses and lists what uses it; it can be retired instead.
14. **Given** a missing name or purpose, a step without text, or a methodology adapted from
    itself, **When** the researcher saves, **Then** the tool rejects it.

---

### User Story 3 - Be shown what applies, and record departures (Priority: P3)

When a researcher is about to do something — register a dataset, start an experiment,
submit a manuscript — they ask "what applies here?" and see the conventions, methodology,
assumptions, and checklists that concern it. When they knowingly do otherwise, they record
the departure and why. Later, anyone can see, for any piece of work, which rules it
followed and where it did not.

**Why this priority**: A convention nobody sees at the right moment is not followed, and a
departure nobody recorded looks like a mistake. This is what turns a written handbook into
something that shapes the work and can be reported honestly.

**Independent Test**: With a convention applying to datasets and a methodology linked to an
experiment, ask what applies to a new dataset and to the experiment; record a departure
from the convention for one dataset with a reason; and list all departures in a project.

**Acceptance Scenarios**:

1. **Given** conventions that apply to a kind of record, **When** the researcher asks what
   applies to that kind, **Then** they are listed, the binding ones first, each with its
   origin.
2. **Given** a particular record, **When** the researcher asks what applies to it, **Then**
   the conventions for its kind, project, and topics, the methodology it follows, the
   assumptions under it, and the checklists applied to it are shown together.
3. **Given** conventions that apply to a kind of record, **When** the researcher creates a
   record of that kind, **Then** the tool mentions how many apply and how to see them,
   without slowing the command or asking anything.
4. **Given** two conventions that apply to the same thing and say different things, **When**
   they are shown, **Then** the tool marks them as conflicting and says which takes
   precedence and why.
5. **Given** a convention or a methodology step, **When** the researcher records that a
   piece of work departs from it, with the reason and what was done instead, **Then** the
   departure is stored with that work.
6. **Given** a departure from something whose origin is the institution, a venue, or a
   funder, **When** it is recorded, **Then** the tool asks who approved it and warns when
   nobody is named.
7. **Given** a piece of work, **When** the researcher views it, **Then** its departures are
   shown with it.
8. **Given** a project or the whole workspace, **When** the researcher lists departures,
   **Then** each is shown with the work, what was departed from, the reason, and the date.
9. **Given** a convention many pieces of work depart from, **When** the researcher asks which
   conventions are most often departed from, **Then** they are listed — candidates for
   changing the convention.
10. **Given** a convention, **When** the researcher marks that they have read it, **Then**
    the date is recorded, and conventions adopted or changed since they last read them can
    be listed.
11. **Given** a departure whose reason is missing, or that names something that does not
    apply to the work, **When** the researcher saves, **Then** the tool rejects it.
12. **Given** a convention retired after work departed from it, **When** the departures are
    listed, **Then** they remain, marked as concerning a retired convention.

---

### User Story 4 - Record assumptions and decisions (Priority: P4)

A researcher writes down what the work takes for granted and what was chosen along the way.
For an assumption: what is assumed, why it is reasonable, what would follow if it were
false, and how it could be checked. For a decision: the question, the options considered,
the one chosen, and why. Months later, when a reviewer or a successor asks "why did you do
it this way?", the answer is there.

**Why this priority**: Assumptions and decisions are where research most often cannot
account for itself afterwards. They are fewer than conventions and attached to particular
work, so they follow the first three stories.

**Independent Test**: Record an assumption under an experiment with a way to check it, mark
it checked and holding with a result as evidence; record a decision with three alternatives
and a rationale; and list the unchecked assumptions of a project.

**Acceptance Scenarios**:

1. **Given** a piece of work, **When** the researcher records an assumption under it with a
   statement, **Then** it is stored with the status "held".
2. **Given** an assumption, **When** the researcher records why it is reasonable, what would
   follow if it were false, and how it could be checked, **Then** these are shown with it.
3. **Given** an assumption, **When** the researcher records that it was checked and holds,
   was checked and does not hold, or cannot be checked, with the evidence, **Then** the
   status and date are recorded.
4. **Given** an assumption found not to hold, **When** it is recorded, **Then** the tool
   lists the experiments, results, and manuscripts that rest on it, for the researcher to
   look at again.
5. **Given** a project, **When** the researcher lists its assumptions, **Then** each is shown
   with its status, and those never checked whose failure would matter most are first.
6. **Given** a manuscript, **When** the researcher asks for the assumptions under it,
   **Then** those of its experiments, models, and results are gathered into one list.
7. **Given** a question that had more than one answer, **When** the researcher records a
   decision with the question, the options considered, the option chosen, the reason, the
   date, and who decided, **Then** it is stored and linked to what it concerns.
8. **Given** a decision, **When** the researcher records what would make them reconsider
   it, **Then** this is shown, and the decision can be given a date to be looked at again.
9. **Given** a decision that is reversed, **When** the researcher records the new decision
   as replacing it, **Then** both are kept and the earlier one shows what replaced it.
10. **Given** a record, **When** the researcher asks for the decisions that concern it,
    **Then** they are listed in order of date.
11. **Given** something that went wrong or unexpectedly well, **When** the researcher
    records the lesson — what happened, what was learned, what to do next time — **Then** it
    is stored, and can be turned into a convention.
12. **Given** an assumption without a statement, or a decision without a chosen option or a
    reason, **When** the researcher saves, **Then** the tool rejects it.

---

### User Story 5 - Work through checklists and guidelines (Priority: P5)

A researcher keeps the checklists their work must satisfy — a reporting guideline their
field expects for a kind of study, a venue's submission requirements, the lab's own list of
things to verify before sending anything out. They apply a checklist to a manuscript or an
experiment, go through it item by item, saying for each whether it is satisfied and where,
and see what is still missing.

**Why this priority**: Checklists are how a community's expectations are made explicit.
They are the most structured of the implicit things and the last to be needed — at
reporting time.

**Independent Test**: Create a checklist of eight items, two of them optional, apply it to
a manuscript, mark five satisfied with where each is addressed and one not applicable with
a reason, and view the report of what remains.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a checklist with a name, an
   origin, and what kind of work it is for, **Then** it is stored.
2. **Given** a checklist, **When** the researcher adds items in order, each with a
   statement, whether it is required, and optional guidance, grouped under headings,
   **Then** the checklist shows them.
3. **Given** a checklist in a table in a file, **When** the researcher brings it in,
   **Then** each row becomes an item, and invalid rows are reported.
4. **Given** a checklist and a manuscript or an experiment, **When** the researcher applies
   the checklist to it, **Then** every item is listed for that work as unanswered.
5. **Given** an applied checklist, **When** the researcher marks an item satisfied and says
   where it is addressed — a part of the manuscript, a record, a page, or a note — **Then**
   the answer is stored.
6. **Given** an item that does not apply to this work, **When** the researcher marks it not
   applicable with a reason, **Then** it is counted as answered.
7. **Given** an applied checklist, **When** the researcher asks for its state, **Then** the
   number of items satisfied, not applicable, and unanswered is shown, required ones
   distinguished, with the unanswered items listed.
8. **Given** an applied checklist with every required item answered, **When** the researcher
   asks whether it is complete, **Then** the answer is yes in a way another program can act
   on.
9. **Given** an applied checklist, **When** the researcher exports it, **Then** a document
   is produced listing each item with its answer and where it is addressed, in the form
   venues ask for.
10. **Given** a checklist that changes after it was applied, **When** the researcher looks at
    the application, **Then** it keeps the items it was applied with and says a newer
    version exists, with what differs.
11. **Given** a manuscript, **When** its readiness is checked, **Then** required checklist
    items still unanswered are among the findings.
12. **Given** an item marked satisfied without saying where, an item without a statement, or
    a checklist applied twice to the same work, **When** the researcher saves, **Then** the
    tool rejects it, or for the last asks whether a second application is intended.

---

### User Story 6 - Keep and share a handbook (Priority: P6)

Everything written down under the earlier stories is the lab's handbook. A lab head hands
it to a new member as one file; the member brings it into their own workspace, where each
entry arrives marked as coming from the lab and is read before it is relied on. When the
lab revises its handbook, members bring in the new one and see what changed. A researcher
also produces, for a manuscript, a statement of the methods, assumptions, and departures
behind it.

**Why this priority**: The value of making things explicit multiplies when they are passed
on. This comes last because there must be a handbook before it can be shared.

**Independent Test**: Export the lab-origin conventions and methodologies as a handbook,
bring it into another workspace, confirm they arrive with their origin and unread, change
one in the first workspace, export again, bring it in, and see the difference reported.

**Acceptance Scenarios**:

1. **Given** conventions, methodologies, and checklists, **When** the researcher exports a
   handbook — all of them, or those of an origin, a category, or a project — **Then** one
   file is produced that contains them completely.
2. **Given** a handbook file, **When** the researcher brings it in, **Then** the tool shows
   what it contains, and adds each entry with its origin, marked as received and unread.
3. **Given** a handbook brought in again in a newer version, **When** it is brought in,
   **Then** the tool shows what was added, changed, and retired, and applies the changes
   only on acceptance.
4. **Given** an entry received from a handbook, **When** the researcher changes it locally,
   **Then** it is marked as locally changed, and a later handbook does not overwrite it
   without the researcher's choice.
5. **Given** an entry received from a handbook, **When** the researcher disagrees with it for
   their own work, **Then** they record a departure, and the entry itself is left as
   received.
6. **Given** a handbook, **When** the researcher exports it as a document, **Then** a
   readable handbook is produced, organized by category, with each entry's origin,
   strength, rationale, and examples.
7. **Given** a manuscript, **When** the researcher asks for its methods statement, **Then** a
   text is produced listing the methodologies used with their versions and sources, how
   they were adapted, the assumptions made, and the departures from conventions and
   methods, ready to be edited into the manuscript.
8. **Given** a new member, **When** the lab head asks which entries a member-facing handbook
   should start with, **Then** the binding conventions and the methodologies in current use
   are listed first.
9. **Given** entries received from several handbooks — the lab's and a community's — **When**
   two say different things about the same subject, **Then** both are kept, marked as
   conflicting, with which takes precedence.
10. **Given** a file that is damaged or is not a handbook, **When** it is brought in,
    **Then** the tool refuses and adds nothing.
11. **Given** a handbook, **When** it is exported, **Then** private notes, people's contact
    details, and anything marked private are left out.

---

### Edge Cases

- A convention applies to everything (no kind, project, or topic given): it is shown
  whenever "what applies" is asked, and the tool suggests narrowing it.
- A convention is recorded twice in different words: both are kept; the tool shows
  conventions with similar wording so one can be retired in favour of the other.
- A convention's origin is unknown ("it has always been like this"): the origin "lab" or
  "community" may be recorded with the source marked unknown; the entry is listed among
  those worth tracing.
- A binding convention has no rationale: it is accepted, marked as unexplained, and listed
  among entries to complete.
- Two conventions conflict and have the same origin and strength: the tool marks the
  conflict and gives precedence to neither; the researcher must resolve it.
- A methodology from the literature is linked to a reference that is later removed from the
  library: the methodology keeps the citation as text.
- A methodology is adapted from one that is later retired: the lineage is kept and shows the
  retired original.
- A methodology's new version removes a step that a departure referred to: the departure is
  kept, marked as concerning an earlier version.
- An experiment's runs used version 1 of a methodology and later runs version 2: each run
  shows the version it used, and the methods statement lists both.
- An assumption is shared by several experiments: it is one assumption, linked to each.
- An assumption is marked as not holding and later as holding again (the check was wrong):
  both changes are kept in its history with their evidence.
- A decision is recorded long after it was made: the date it was made and the date it was
  recorded are both kept.
- A decision concerns nothing in particular (a general choice of direction): it is recorded
  against the project, or against none.
- A checklist item is satisfied by something outside the workspace (a supplementary file):
  the answer is a note saying where.
- A checklist is applied to a manuscript that is then split in two: the application stays
  with the original; the researcher applies it again to the other.
- A community guideline's wording is under a licence that forbids copying it: the researcher
  records the item numbers and their own short labels, and a link to the guideline.
- A handbook entry received from the lab is retired locally: it stays retired locally when
  the handbook is brought in again, unless the researcher chooses otherwise.
- The same handbook is brought in twice unchanged: nothing is added and the tool says so.
- A departure is recorded and the convention is later changed to match what was done: the
  departure is kept, marked as resolved by the change.
- Text is in any language and script: it is stored and shown as written.

## Requirements *(mandatory)*

### Functional Requirements

#### Origin and common traits

- **FR-001**: Conventions, methodologies, assumptions, decisions, and checklists MUST each
  carry an origin: own research, lab, institution, community, venue or funder, or
  literature; and, where relevant, the reference, document, organization, or person they
  come from. An unknown source MUST be recordable as unknown.
- **FR-002**: Each MUST carry a status and the history of its changes — wording, status,
  and who decided, with dates — and MUST be retirable with a reason and replaceable by
  another, the replaced one remaining viewable.
- **FR-003**: Each MUST be attachable to what it concerns: kinds of record, particular
  records, projects, and topics.
- **FR-004**: Users MUST be able to list each kind filtered by origin, status, category, and
  what it concerns, and search their text; the system MUST show entries of similar wording
  when one is created.
- **FR-005**: Users MUST be able to list entries worth completing: binding conventions
  without a rationale, entries whose source is unknown, and methodologies without steps.

#### Conventions

- **FR-006**: Users MUST be able to create, list, view, update, and delete conventions, each
  with a title unique in the workspace, a statement, a rationale, a category, a strength
  (must, should, may), examples of following and of not following it, and a status
  (proposed, adopted, rejected, retired).
- **FR-007**: The system MUST offer the categories naming, files and folders, notation and
  units, writing, data handling, analysis, authorship, reviewing, and meetings, and accept
  any other.
- **FR-008**: A convention recorded without origin, strength, or status MUST be own
  research, "should", and adopted.
- **FR-009**: Adopting or rejecting a proposed convention MUST record who decided and when.
- **FR-010**: Users MUST be able to turn a captured thought or a recorded lesson into a
  convention.

#### Methodologies

- **FR-011**: Users MUST be able to create, list, view, update, and delete methodologies,
  each with a name unique in the workspace, a purpose, a description, the kind of work it
  is for, when to use it and when not, what it needs beforehand, what it produces, its
  pitfalls, its limitations, and the references that define or describe it.
- **FR-012**: Users MUST be able to write a methodology's procedure as ordered steps, each
  with what to do, an optional check that it was done right, and optional notes.
- **FR-013**: Users MUST be able to record new versions of a methodology with a note of what
  changed; earlier versions MUST remain viewable, and versions MUST be comparable step by
  step.
- **FR-014**: Users MUST be able to adapt a methodology into a new one that records what it
  was adapted from and each difference with its reason; the system MUST show a
  methodology's lineage back to its original and MUST reject a methodology adapted from
  itself, directly or indirectly.
- **FR-015**: Experiments, literature reviews, and models MUST be linkable to the
  methodology they follow, and the version in effect MUST be recorded with the work and
  with each run.
- **FR-016**: The system MUST show where a methodology is used, by version, and MUST refuse
  to delete one that is in use.
- **FR-017**: Users MUST be able to create the steps of an experiment's pipeline from a
  methodology's steps.

#### What applies, and departures

- **FR-018**: Users MUST be able to ask what applies to a kind of record and to a particular
  record; for a record the system MUST show together the conventions for its kind, project,
  and topics, the methodology it follows, the assumptions under it, the decisions that
  concern it, and the checklists applied to it, the binding ones first, each with its
  origin.
- **FR-019**: When a record of a kind that has conventions is created, the system MUST say
  how many apply and how to see them, MUST NOT ask anything, and MUST allow this mention to
  be turned off.
- **FR-020**: The system MUST mark conventions that apply to the same thing and conflict,
  and state which takes precedence: a venue's, funder's, or institution's over the lab's,
  the lab's over the community's, the community's over the literature's and own research,
  and within one origin "must" over "should" over "may". Conflicts it cannot order MUST be
  shown as unresolved.
- **FR-021**: Users MUST be able to record that a piece of work departs from a convention, a
  methodology step, or a checklist item, with the reason, what was done instead, and who
  approved it; a reason MUST be required.
- **FR-022**: Recording a departure from something whose origin is the institution, a venue,
  or a funder MUST ask who approved it and MUST warn when nobody is named.
- **FR-023**: The system MUST show a piece of work's departures with it, list departures for
  a project or the whole workspace, and list the conventions most often departed from.
- **FR-024**: A departure MUST be kept when what it departed from is changed, retired, or
  given a new version, marked accordingly.
- **FR-025**: Users MUST be able to mark an entry as read, and to list entries adopted or
  changed since they last read them.

#### Assumptions, decisions, and lessons

- **FR-026**: Users MUST be able to create, list, view, update, and delete assumptions, each
  with a statement, why it is reasonable, what would follow if it were false, how it could
  be checked, and a status: held, checked and holding, checked and not holding, or cannot
  be checked.
- **FR-027**: Changing an assumption's status MUST record the date and the evidence; when an
  assumption is found not to hold, the system MUST list the experiments, models, results,
  and manuscripts that rest on it.
- **FR-028**: An assumption MUST be linkable to several pieces of work, and the system MUST
  gather, for a manuscript or a project, the assumptions under everything it rests on,
  unchecked ones whose failure would matter most first.
- **FR-029**: Users MUST be able to create, list, view, update, and delete decisions, each
  with the question, the options considered, the option chosen, the reason, the date it was
  made, who decided, what would make them reconsider, and what it concerns.
- **FR-030**: A decision MUST be replaceable by a later one, both being kept; and MUST be
  givable a date to be looked at again, which appears in the workspace's view of what is
  due.
- **FR-031**: Users MUST be able to record lessons — what happened, what was learned, and
  what to do next time — linked to what they concern, and to turn a lesson into a
  convention.
- **FR-032**: The system MUST list, for any record, the assumptions, decisions, and lessons
  that concern it, in order of date.

#### Checklists

- **FR-033**: Users MUST be able to create, list, view, update, and delete checklists, each
  with a name, an origin, the kind of work it is for, a version, and ordered items grouped
  under headings; each item has a statement, whether it is required, and optional guidance.
- **FR-034**: Users MUST be able to bring a checklist in from a table in a file, with each
  invalid row reported, and to export one.
- **FR-035**: Users MUST be able to apply a checklist to a manuscript, an experiment, a
  literature review, or a project; the application MUST keep the items it was applied with,
  and MUST say when a newer version of the checklist exists and what differs.
- **FR-036**: For each item of an applied checklist users MUST be able to answer satisfied
  (saying where it is addressed), not applicable (with a reason), or not satisfied (with a
  note); "satisfied" without saying where MUST be rejected.
- **FR-037**: The system MUST show an applied checklist's state — satisfied, not applicable,
  not satisfied, and unanswered, required items distinguished — and MUST report whether it
  is complete in a way another program can act on.
- **FR-038**: Users MUST be able to export an applied checklist as a document listing each
  item, its answer, and where it is addressed.
- **FR-039**: Required items still unanswered or not satisfied in checklists applied to a
  manuscript MUST appear among the findings of that manuscript's readiness check.

#### Handbook

- **FR-040**: Users MUST be able to export conventions, methodologies, and checklists — all,
  or those of an origin, a category, or a project — as one handbook file, and as a readable
  document organized by category with each entry's origin, strength, rationale, and
  examples.
- **FR-041**: Users MUST be able to bring a handbook file in; the system MUST show what it
  contains, add each entry with its origin and the handbook it came from, and mark it as
  received and unread.
- **FR-042**: Bringing in a newer version of a handbook MUST show what was added, changed,
  and retired, and MUST apply changes only on acceptance; an entry changed or retired
  locally MUST NOT be overwritten without the user's choice.
- **FR-043**: Entries received from several handbooks that conflict MUST all be kept, marked
  as conflicting, with precedence as in FR-020.
- **FR-044**: A handbook MUST leave out private notes, contact details of people, and
  anything marked private.
- **FR-045**: The system MUST refuse a file that is damaged or is not a handbook, and add
  nothing; bringing in the same handbook unchanged MUST add nothing and say so.
- **FR-046**: The system MUST produce, for a manuscript, a methods statement listing the
  methodologies used with their versions and sources, how each was adapted, the assumptions
  made with their status, and the departures from conventions, methods, and checklists.

#### Common behavior

- **FR-047**: Every value a user supplies for conventions, methodologies, steps,
  assumptions, decisions, lessons, checklists, answers, and departures — typed or brought
  in from a file — MUST be validated before anything is stored; invalid input MUST change
  nothing and MUST be reported per value, all together, with what is expected. Free text
  MUST be accepted as written, in any language.
- **FR-048**: Every record type here MUST support tags, notes, and links, belong to projects
  and topics as other records do, and have every creation, change, adoption, retirement,
  departure, and answer recorded in the workspace's audit trail.
- **FR-049**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-050**: Every command MUST have built-in help, and conventions, methodologies, what
  applies and departures, assumptions and decisions, checklists, and the handbook MUST each
  have a usage guide with examples.

### Key Entities *(include if feature involves data)*

- **Origin**: Where an entry comes from — own research, lab, institution, community, venue
  or funder, literature — with its source.
- **Convention**: An agreed way of doing something: statement, rationale, category,
  strength, examples, status, and what it applies to.
- **Methodology**: How a kind of work is done: purpose, when to use, prerequisites, ordered
  steps with checks, outputs, pitfalls, limitations, references, versions, and what it was
  adapted from.
- **Methodology Step**: One instruction of a procedure, with an optional check.
- **Adaptation**: The relation between a methodology and the one it was adapted from, with
  each difference and its reason.
- **Assumption**: Something taken as true: statement, justification, consequence if false,
  how to check, status, and the work that rests on it.
- **Decision**: A choice among options: question, options, the one chosen, reason, date,
  who decided, what would reopen it, and what it concerns.
- **Lesson**: What an event taught: what happened, what was learned, what to do next time.
- **Checklist**: An ordered list of items a kind of work must satisfy, with an origin and a
  version.
- **Checklist Application**: One checklist applied to one piece of work, with an answer for
  each item and where it is addressed.
- **Departure**: The record that a piece of work knowingly does not follow a convention, a
  methodology step, or a checklist item, with the reason, what was done instead, and who
  approved it.
- **Handbook**: Conventions, methodologies, and checklists exported together, to be brought
  into another workspace or read as a document.
- **Read Mark**: When the researcher last read an entry.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can write down a convention with its origin and strength in under 1
  minute.
- **SC-002**: A user can find out everything that applies to a record — conventions,
  methodology, assumptions, checklists — in a single view in under 5 seconds.
- **SC-003**: For any result, a user can state the methodology and version behind it, where
  that methodology came from, and how it was adapted, in under 1 minute.
- **SC-004**: For any piece of work, 100% of its recorded departures are shown with it, each
  with a reason.
- **SC-005**: When an assumption is marked as not holding, 100% of the experiments, models,
  results, and manuscripts recorded as resting on it are listed.
- **SC-006**: A user can answer "why was this done this way?" for any recorded decision in
  under 30 seconds, including the options that were rejected.
- **SC-007**: A user can see what remains to satisfy in a 30-item checklist applied to a
  manuscript in under 5 seconds, and 100% of unanswered required items appear in that
  manuscript's readiness check.
- **SC-008**: A handbook exported from one workspace and brought into another yields
  identical entries, each marked with its origin and as unread.
- **SC-009**: When a newer handbook is brought in, 100% of added, changed, and retired
  entries are reported, and 0 locally changed entries are overwritten without the user's
  choice.
- **SC-010**: A new lab member can find the binding conventions that apply to their first
  dataset or experiment in under 2 minutes, without asking anyone.
- **SC-011**: A methods statement for a manuscript with 3 methodologies and 5 departures is
  produced in under 30 seconds, and names every one of them.
- **SC-012**: 100% of invalid inputs are rejected before any data changes, each with the
  invalid value named.
- **SC-013**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns methodologies.** The methodology part of User Story 7 of
  `specs/001-research-workspace` (FR-039) is replaced by this one; datasets stay there.
  Experiments (`specs/004-experiments`), literature reviews (`specs/005-literature`), and
  models (`specs/009-simulations`) keep linking to a methodology exactly as before.
- **"The meta parts of research that are in general implicit" is read as five kinds of
  record**: conventions, methodologies, assumptions, decisions (with lessons), and
  checklists. Other implicit things already have a home — definitions of terms in the
  glossary of `specs/002-research-lifecycle`, ideas and topics in
  `specs/013-ideas-questions`, who does what in `specs/012-people`.
- **The tool records and shows; it does not enforce.** It cannot tell whether a file was
  named according to a convention or a step was carried out as written. Following is the
  researcher's act; the tool makes the rule visible at the right moment and makes
  departures recordable. Automatic checking of particular conventions is a possible later
  addition.
- **Precedence between origins is a stated default** (venue, funder, institution over lab
  over community over literature and own research). It reflects who is able to waive a
  rule, and can be argued; the researcher resolves any conflict it leaves.
- **Model assumptions of `specs/009-simulations` are assumptions in the sense used here**;
  that specification keeps its own rules for them and they appear in the lists gathered
  here.
- **No guidelines or handbooks are shipped.** The tool comes with no community reporting
  guideline, since their texts are often under licences that restrict copying, and with no
  conventions of its own. Researchers bring in or type what applies to them.
- **A handbook is shared as a file**, like template packs and learning roadmaps. A lab that
  keeps one authoritative handbook passes each new version to its members; live sharing
  arrives with collaboration in `specs/002-research-lifecycle` and sync in
  `specs/010-integrations`.
- **"Who approved" a departure is a name on record**, not a verified approval. Where an
  ethics body or a funder must approve, their own process governs.
- **The methods statement is a draft for the researcher to edit**, not finished prose. It
  can be placed in a manuscript through a template (`specs/007-templates`).
- **Similar wording is detected by shared words** and may miss two entries that say the same
  thing differently.
- **Other specifications are used, not redefined**: references (`specs/005-literature`),
  manuscripts and their readiness check (`specs/008-manuscripts`), experiments, pipelines,
  runs, and results (`specs/004-experiments`), people (`specs/012-people`), the inbox and
  topics (`specs/013-ideas-questions`), the "private" marking (`specs/006-reports`), and
  links, tags, notes, and the audit trail (`specs/001-research-workspace`).
- **Single researcher**, as elsewhere: "who decided" and "who approved" name people; they do
  not act in the workspace.
