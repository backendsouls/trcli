<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Literature, References, and Bibliography

**Feature Branch**: `005-literature`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add concepts of bibliography/references/literature as a separated spec on its own"

## Overview

Every piece of research stands on what others have written. This specification gathers
everything TRCLI knows about that body of work into one place: the references a researcher
collects, how they are brought in and kept clean, how they are cited, how they are read and
annotated, how they relate to one another, and how a formal literature review is conducted
and synthesized.

It **takes over** the literature content previously spread across two specifications, which
now point here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 1 | Papers, citations, import and export, duplicates, online lookup |
| `specs/001-research-workspace`, User Story 3 | Bibliographic research (literature reviews) |
| `specs/002-research-lifecycle`, User Story 1 | Reading annotations and structured summaries |

It **adds** what was not specified before: references of kinds other than papers, named
bibliographies, a reading queue, relations between references, two-stage screening with a
flow summary, and a synthesis matrix.

### The three words in the request

| Term | Meaning in TRCLI |
|------|------------------|
| **Reference** | One work the researcher may read or cite: an article, a book, a thesis, a web page, a dataset, and so on. The record that holds its details. |
| **Literature** | The references taken together as a body of knowledge: what has been read, what it says, how the works relate, and what a review of them concludes. |
| **Bibliography** | A named, ordered list of references assembled for a purpose — a chapter, a paper, a course — and written out in a citation style. |

Elsewhere in TRCLI the word "paper" is used for what is called a **reference** here; a
paper is the most common kind of reference, and every rule that mentions a paper applies to
any reference.

### The concepts, in one picture

```text
                      ┌──── cites / extends / contradicts / version of ────┐
                      ▼                                                    │
 Online catalogue ─▶ Reference ──── has ──▶ Citation (key) ──── in ──▶ Bibliography
 Bibliography file ─▶   │  │                    ▲                          │
                        │  └─ has ─▶ Annotation │ used by                  ▼
                        │            Summary    │                  written in a Citation Style
                        │                     Draft
                        └─ candidate in ─▶ Literature Review ─▶ Search
                                               │                Screening Decision
                                               └─ extracts ──▶ Synthesis Matrix
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Build a reference library (Priority: P1)

A researcher adds the works they come across — articles, preprints, books, chapters,
theses, reports, web pages, datasets, software — records their details, tags them, and
finds them again by searching and filtering. The library warns when the same work is added
twice and can merge the two.

**Why this priority**: Collecting references is the first and most repeated activity of
research. A workspace that only does this is already a usable reference manager, and every
other story here builds on these records.

**Independent Test**: Add references of three different kinds, tag them, search and filter
them, add one a second time to see the duplicate warning, and merge the two.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher adds a reference with a title, a kind,
   authors, and a year, **Then** it is stored with a unique identifier and the reading
   status "to read".
2. **Given** a reference of a given kind, **When** the researcher records the details that
   kind has (for an article: journal, volume, issue, pages; for a book: publisher, edition;
   for a thesis: institution, degree; for a web page: address and date accessed), **Then**
   they are saved and shown.
3. **Given** a reference, **When** the researcher records its persistent identifiers, its
   web address, its language, its keywords, and the location of a local copy, **Then** they
   are saved.
4. **Given** stored references, **When** the researcher lists them filtered by kind,
   author, year or range of years, venue, tag, language, or reading status, **Then** only
   matching references are shown.
5. **Given** stored references, **When** the researcher searches for words, **Then**
   references whose title, authors, abstract, keywords, or notes contain them are shown.
6. **Given** a reference with the same persistent identifier as a stored one, or the same
   title, first author, and year, **When** the researcher adds it, **Then** the tool reports
   the likely duplicate and asks whether to add, merge, or cancel.
7. **Given** two references that are the same work, **When** the researcher merges them,
   **Then** one reference remains with the details of both, details the researcher entered
   are never overwritten by empty ones, and every citation, annotation, tag, link, and
   review entry of the other now belongs to it.
8. **Given** a reference, **When** the researcher asks for possible duplicates across the
   whole library, **Then** groups of likely duplicates are listed.
9. **Given** a reference that other records refer to, **When** the researcher deletes it,
   **Then** the tool lists what refers to it and requires explicit confirmation.
10. **Given** a missing title, a year that is not a valid year, a malformed identifier, or a
    local copy that does not exist, **When** the researcher saves, **Then** the tool rejects
    it and explains every problem together.
11. **Given** an author, **When** the researcher asks for that author's works, **Then** all
    references by that author are listed, whichever way the name was written.

---

### User Story 2 - Bring references in and out (Priority: P2)

A researcher fills the library without typing: by importing a bibliography file from
another tool, or by giving only an identifier and letting the tool fetch the details from
public catalogues. They can also complete references that are missing details, and export
any part of the library to a file other tools can read.

**Why this priority**: Nobody starts from an empty library, and nobody wants to retype
metadata. Import and lookup are what make the library practical beyond a handful of
entries.

**Independent Test**: Import a bibliography file containing valid and invalid entries, add
a reference by identifier alone, complete a reference with missing details, and export a
filtered set to a file that another reference tool opens.

**Acceptance Scenarios**:

1. **Given** a bibliography file from another tool, **When** the researcher imports it,
   **Then** each valid entry becomes a reference with its citation, and each invalid entry
   is reported individually with its reason without stopping the import.
2. **Given** an import, **When** it finishes, **Then** the tool reports how many entries
   were imported, were invalid, and were duplicates.
3. **Given** an import containing entries already in the library, **When** the researcher
   chose beforehand to skip, add, or merge duplicates, **Then** that choice is applied to
   every duplicate.
4. **Given** an import, **When** the researcher asks for a preview, **Then** the tool shows
   what would be imported and changes nothing.
5. **Given** a network connection, **When** the researcher adds a reference by giving only
   its persistent identifier (for an article, a preprint, or a book), **Then** the tool
   fetches the details, shows them, and stores the reference once the researcher accepts.
6. **Given** no network connection or an identifier no catalogue recognizes, **When** the
   researcher adds a reference by identifier, **Then** the tool says which of the two
   happened, stores nothing by itself, and offers to continue by entering details by hand.
7. **Given** a stored reference with missing details, **When** the researcher asks the tool
   to complete it from its identifier, **Then** fetched details are shown next to the stored
   ones and nothing the researcher entered is overwritten without confirmation.
8. **Given** several references with missing details, **When** the researcher asks to
   complete them all, **Then** each is looked up in turn, progress is shown, and a summary
   says which were completed, unchanged, or not found.
9. **Given** stored references, **When** the researcher exports all of them or a filtered
   set, **Then** a standard bibliography file is produced containing every selected one.
10. **Given** a file in an unknown or damaged form, **When** the researcher imports it,
    **Then** the tool says it cannot be read and changes nothing.

---

### User Story 3 - Cite and assemble bibliographies (Priority: P3)

A researcher gives each reference a citation key to use in their writing, sees how a
reference looks in a chosen citation style, and assembles named bibliographies — "Chapter
2", "Related work for the journal paper" — that they export as a file for their writing
tool or as a formatted reference list.

**Why this priority**: Citing is the purpose of collecting. Keys and bibliographies are what
connect the library to the papers the researcher writes.

**Independent Test**: Give three references citation keys, show one in two styles, create a
bibliography with the three in a chosen order, and export it both as a file for a writing
tool and as a formatted list.

**Acceptance Scenarios**:

1. **Given** a reference, **When** the researcher saves a citation for it, **Then** the
   citation receives a key that is unique in the workspace, proposed from the author, year,
   and title unless the researcher gives one.
2. **Given** two references that would receive the same key, **When** the second is saved,
   **Then** the tool makes the key unique and tells the researcher.
3. **Given** a citation key already used in the researcher's writing, **When** the
   reference's details change, **Then** the key does not change unless the researcher asks.
4. **Given** a reference, **When** the researcher asks to see it in a citation style,
   **Then** it is shown as that style formats a reference-list entry and an in-text
   citation.
5. **Given** a workspace, **When** the researcher creates a bibliography with a name and a
   purpose and adds references to it, **Then** the bibliography lists them and each
   reference lists the bibliographies it is in.
6. **Given** a bibliography, **When** the researcher sets its citation style and its
   ordering (by author, by year, by order of addition, or by hand), **Then** it is shown
   and exported that way.
7. **Given** a bibliography, **When** the researcher exports it, **Then** they can obtain a
   file for a writing tool or a formatted reference list in the bibliography's style.
8. **Given** a draft paper, **When** the researcher links a bibliography to it, **Then** the
   draft's citations are those of the bibliography.
9. **Given** a reference missing a detail its citation style requires, **When** it is
   formatted, **Then** the tool formats what it can and names the missing detail.
10. **Given** a bibliography, **When** the researcher asks for a check, **Then** the tool
    lists references with missing required details, without a citation key, or likely to
    be duplicates of one another.
11. **Given** a citation style the tool does not know, **When** the researcher supplies a
    style definition file, **Then** the style can be used like a built-in one.

---

### User Story 4 - Read and annotate (Priority: P4)

A researcher manages what to read next and captures what they learn while reading: they set
each reference's reading status, priority, and their own rating; record quotes with their
page numbers, highlights with a comment, and a structured summary of the work; and later
search across everything they have annotated and pull a quote, with its page and citation,
into their writing.

**Why this priority**: A library that is only collected is not literature that is known.
Page-anchored quotes and consistent summaries are what make reading reusable months later.

**Independent Test**: Put five references in the reading queue with priorities, mark one as
being read, add three quotes with page numbers and a structured summary to it, search
annotations across the library for a word, and export one quote with its page and citation.

**Acceptance Scenarios**:

1. **Given** a reference, **When** the researcher changes its reading status (to read,
   reading, read, discarded), **Then** the status and the date are recorded.
2. **Given** references to read, **When** the researcher sets a priority on them and asks
   for the reading queue, **Then** unread references are listed by priority, then by date
   added.
3. **Given** a reference that has been read, **When** the researcher records their rating
   and its relevance to their work, **Then** both are saved and can be filtered on.
4. **Given** a reference, **When** the researcher adds a quote with its page, **Then** the
   quote is stored under that reference with the page and the date.
5. **Given** a reference, **When** the researcher adds a highlight with a comment and a kind
   (key claim, method, finding, limitation, question, idea), **Then** it is stored and can
   be filtered by kind.
6. **Given** a reference, **When** the researcher fills in its structured summary — the
   problem, the method, the findings, the limitations, and their own assessment — **Then**
   the summary is shown when the reference is viewed.
7. **Given** annotations on many references, **When** the researcher searches annotations
   for a word or filters by kind or tag, **Then** matching annotations are listed with
   their reference and page.
8. **Given** a quote, **When** the researcher exports it, **Then** the output contains the
   quoted text, the page, and the reference's citation in a chosen style.
9. **Given** a reference, **When** the researcher exports its reading notes, **Then** a
   document with the summary and all annotations in page order is produced.
10. **Given** a page that is not a positive number or page range, or a quote with no text,
    **When** the researcher saves it, **Then** the tool rejects it and explains why.
11. **Given** a period, **When** the researcher asks for their reading activity, **Then**
    the number of references added, started, and read in that period is shown.

---

### User Story 5 - Relate references to one another (Priority: P5)

A researcher records how works relate: this one cites that one, extends it, contradicts it,
replicates it, or is another version of it (a preprint and its published article). They can
then look at one reference and see the works around it, and see which references in their
library are most cited by the others.

**Why this priority**: Literature is a web, not a list. Relations are what let a researcher
follow an argument across works, but they are only worth recording once the library exists.

**Independent Test**: Relate four references with three kinds of relation, view one
reference's neighbourhood, and list the references most cited within the library.

**Acceptance Scenarios**:

1. **Given** two references, **When** the researcher records that one cites, extends,
   contradicts, replicates, or reviews the other, with an optional comment, **Then** the
   relation is shown from both references, each from its own point of view.
2. **Given** a preprint and its published version, **When** the researcher records them as
   versions of the same work and names the preferred one, **Then** each shows the other,
   and citing the work uses the preferred version unless the researcher chooses otherwise.
3. **Given** a reference, **When** the researcher asks for its neighbourhood, **Then** the
   works it relates to and the works that relate to it are listed by kind of relation.
4. **Given** a library with relations, **When** the researcher asks which references are
   most cited within it, **Then** they are listed with their counts.
5. **Given** a reference, **When** the researcher asks which works in the library
   contradict it, **Then** they are listed with the comments recorded.
6. **Given** a relation from a reference to itself, or a relation that already exists,
   **When** the researcher saves it, **Then** the tool rejects it.
7. **Given** a reference with relations, **When** it is merged into another, **Then** its
   relations move to the remaining reference and none is duplicated.

---

### User Story 6 - Conduct a literature review (Priority: P6)

A researcher conducts a structured review: they state the review question and the
inclusion and exclusion criteria, record every search with its search string, source, and
date, collect the candidate references found, and screen them in two stages — first on
title and abstract, then on the full text — recording a reason for every exclusion. At any
moment they can produce the flow summary that reviewers expect: how many were found,
removed as duplicates, screened, excluded at each stage and why, and finally included.

**Why this priority**: A review turns a pile of references into a defensible basis for the
research. It is the most structured use of the library and depends on all of it.

**Independent Test**: Create a review with criteria, record two searches, attach candidates
including a duplicate, screen them through both stages, and produce the flow summary with
counts that add up.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a review with a title, a
   question, and a kind (narrative, systematic, scoping, mapping), **Then** it is stored
   with the status "planning".
2. **Given** a review, **When** the researcher records its inclusion and exclusion criteria,
   each with a short label, **Then** they are shown with the review and offered as reasons
   during screening.
3. **Given** a review, **When** the researcher records a search with its search string,
   source, date, any limits applied, and the number of results, **Then** the search is kept
   as part of the review.
4. **Given** a review, **When** the researcher attaches references as candidates, saying
   which search found each, **Then** each candidate is shown as "not yet screened".
5. **Given** candidates that are duplicates of one another, **When** they are attached,
   **Then** the tool reports them, counts them as duplicates removed, and keeps one.
6. **Given** a candidate, **When** the researcher screens it on title and abstract as
   included or excluded with a reason, **Then** the decision, reason, stage, and time are
   recorded.
7. **Given** a candidate included on title and abstract, **When** the researcher screens it
   on the full text, **Then** the second decision is recorded separately from the first.
8. **Given** a candidate marked excluded without a reason, **When** the researcher submits
   the decision, **Then** the tool rejects it and asks for a reason.
9. **Given** a review, **When** the researcher asks what remains to screen at a stage,
   **Then** the unscreened candidates of that stage are listed and can be screened one
   after another.
10. **Given** a review with decisions, **When** the researcher requests the flow summary,
    **Then** it shows the counts found per source, duplicates removed, screened and excluded
    at each stage grouped by reason, and included, and the counts are consistent with one
    another.
11. **Given** a review, **When** the researcher changes a decision, **Then** the new
    decision replaces the old and the change is kept in the review's history.
12. **Given** a review, **When** the researcher marks it completed, **Then** the tool warns
    if any candidate is still unscreened, and a completed review can no longer be changed
    until reopened.
13. **Given** a completed review, **When** the researcher exports it, **Then** a document is
    produced with the question, criteria, searches, flow summary, and the list of included
    references.
14. **Given** a search dated in the future or with a negative number of results, **When**
    the researcher saves it, **Then** the tool rejects it.

---

### User Story 7 - Synthesize what the literature says (Priority: P7)

For the references a review includes, a researcher defines what to extract from each —
"sample size", "method", "main finding", "limitations" — fills in those items reference by
reference, and views the outcome as a matrix: references down the side, extracted items
across the top. They also group references by theme and see which themes are well covered
and which are gaps.

**Why this priority**: Synthesis is the result of a review. It is the last step and needs
included references to exist.

**Independent Test**: In a review with three included references, define three items to
extract, fill them in, view and export the matrix, tag the references with two themes, and
see the count per theme.

**Acceptance Scenarios**:

1. **Given** a review, **When** the researcher defines the items to extract, each with a
   name, a kind of value (text, number, yes/no, one of a list), and whether it is required,
   **Then** the items are stored with the review.
2. **Given** an included reference, **When** the researcher records a value for an item,
   optionally with the page it was found on, **Then** the value is stored for that
   reference and item.
3. **Given** a review, **When** the researcher asks what remains to extract, **Then**
   included references with missing required items are listed.
4. **Given** extracted values, **When** the researcher views the matrix, **Then** included
   references are rows, items are columns, and empty cells are visibly empty.
5. **Given** the matrix, **When** the researcher exports it, **Then** a table is produced
   that a spreadsheet or a writing tool can open, with each reference's citation.
6. **Given** a review, **When** the researcher defines themes and assigns included
   references to them, **Then** each theme lists its references and a reference may belong
   to several themes.
7. **Given** themes, **When** the researcher asks for coverage, **Then** each theme shows
   its number of references, and themes with none are shown as gaps.
8. **Given** a value of the wrong kind for its item, **When** the researcher saves it,
   **Then** the tool rejects it and states what is expected.
9. **Given** an item that already has values, **When** the researcher changes its kind or
   removes it, **Then** the tool shows how many values are affected and requires
   confirmation.
10. **Given** an annotation or a structured summary on a reference, **When** the researcher
    extracts an item for it, **Then** they can copy the value from the annotation or
    summary, and the link to its source is kept.

---

### Edge Cases

- The same work is added twice, by hand, by import, or by lookup: the likely duplicate is
  reported and the researcher chooses to add, merge, or cancel.
- A reference has no author, no year, or an organization as its author: it is accepted;
  the citation key and the formatted citation are built from what is available.
- An author's name is written in several ways across references ("Silva, A.", "Ana
  Silva"): the works are found together, and the researcher can state that two names are
  the same person.
- A title or name contains characters outside the Latin alphabet, accents, or mathematics:
  it is stored and shown as written, and sorting and duplicate detection ignore accents and
  letter case.
- Two citations would receive the same key: the tool makes the key unique and says so.
- A citation key is changed while drafts and bibliographies use it: the tool lists where it
  is used and requires confirmation.
- An import file is partly invalid: valid entries are imported, invalid ones are reported
  one by one, and a final count of both is shown.
- An import file is very large: progress is shown and the import can be interrupted,
  leaving what was already imported in place and reported.
- An online lookup returns details that conflict with what the researcher entered: both are
  shown and the researcher chooses, field by field or all at once.
- An online lookup is slow or the catalogue is unavailable: the tool gives up after a
  short, stated wait, says so, and the rest of the tool keeps working.
- An online lookup returns incomplete or malformed details: they are validated like any
  other input, and only valid details are offered.
- A reference's local copy is moved or deleted outside the tool: the reference is kept, and
  the tool reports the copy as missing when it is next needed.
- A quote is added to a reference that has no local copy: it is accepted; the page number
  is the researcher's responsibility.
- A reference is deleted while it is in bibliographies, reviews, or drafts: the tool lists
  each of them and requires confirmation; the review's counts are updated.
- A reference is removed from the library after being included in a completed review: the
  tool refuses until the review is reopened.
- A candidate is excluded on title and abstract and later screened on full text: the tool
  refuses, since only candidates included at the first stage reach the second.
- A candidate found by two searches is attached twice: it is one candidate, and both
  searches are recorded as having found it.
- A review has no searches, or its recorded result counts are lower than its number of
  candidates: the flow summary is still produced and states the inconsistency.
- A bibliography is empty: it can be exported, and the tool says it is empty.
- Versions of the same work are both cited in one bibliography: the check reports it.
- A reference is in a language the citation style does not handle specially: it is
  formatted with the style's general rules.

## Requirements *(mandatory)*

### Functional Requirements

#### Reference library

- **FR-001**: Users MUST be able to create, list, view, update, and delete references, each
  with a title, a kind, ordered authors, a year, and an abstract.
- **FR-002**: The system MUST support these kinds of reference: journal article, conference
  paper, preprint, book, book chapter, thesis or dissertation, report, web page, dataset,
  software, standard, patent, and other.
- **FR-003**: Users MUST be able to record the details particular to each kind, including
  venue, volume, issue, pages, publisher, edition, editors, institution, degree, web
  address, and date accessed.
- **FR-004**: Users MUST be able to record a reference's persistent identifiers, web
  address, language, keywords, and the location of a local copy, without the tool taking a
  copy of the file.
- **FR-005**: Users MUST be able to filter references by kind, author, year or range of
  years, venue, tag, language, reading status, rating, and bibliography, sort them, and
  search by words in title, authors, abstract, keywords, and notes; search and sorting MUST
  ignore letter case and accents.
- **FR-006**: The system MUST detect a likely duplicate when a reference is added, imported,
  or looked up — equal persistent identifier, or equal title, first author, and year after
  ignoring case, accents, and punctuation — and let the user add, merge, or cancel.
- **FR-007**: Users MUST be able to list groups of likely duplicates across the library.
- **FR-008**: Merging two references MUST leave one reference carrying the details of both,
  MUST NOT replace a filled detail with an empty one, and MUST move every citation,
  annotation, tag, note, link, relation, bibliography entry, and review entry to it.
- **FR-009**: Users MUST be able to list every reference by an author regardless of how the
  name was written, and to state that two written names are the same person.
- **FR-010**: Before deleting a reference, the system MUST list every bibliography, review,
  draft, and other record that refers to it and MUST require explicit confirmation.

#### Import and export

- **FR-011**: Users MUST be able to import references and citations from, and export them
  to, the bibliography file formats in common use among researchers.
- **FR-012**: An import MUST process every entry, store the valid ones, report each invalid
  one with its reason, and finish with counts of imported, invalid, and duplicate entries.
- **FR-013**: Users MUST be able to choose, before an import, how duplicates are handled
  (skip, add, or merge), and to preview an import without changing anything.
- **FR-014**: Users MUST be able to export the whole library, a filtered set, a
  bibliography, or the included references of a review.
- **FR-015**: A file that cannot be read as any supported format MUST be rejected with an
  explanation, and nothing MUST change.

#### Online lookup

- **FR-016**: Users MUST be able to add a reference by giving only a persistent identifier
  for an article, a preprint, or a book, and have its details fetched from public
  catalogues.
- **FR-017**: Users MUST be able to complete the missing details of one stored reference, or
  of many at once, from their identifiers; details the user entered MUST NOT be overwritten
  without confirmation.
- **FR-018**: Fetched details MUST be shown to the user and validated like any other input
  before anything is stored.
- **FR-019**: Online lookup MUST happen only when the user asks for it, MUST send nothing
  other than the identifier being looked up, and MUST be possible to turn off for a
  workspace.
- **FR-020**: When a lookup cannot be completed, the system MUST say whether the cause was
  no connection, an unavailable catalogue, or an unrecognized identifier, MUST store
  nothing by itself, and MUST offer manual entry. Every other capability in this
  specification MUST work without a network connection.

#### Citations and bibliographies

- **FR-021**: Users MUST be able to save a citation for a reference, with a citation key
  unique within the workspace; the system MUST propose a key from the author, year, and
  title, following a configurable pattern, and MUST make colliding keys unique.
- **FR-022**: A citation key MUST NOT change when the reference's details change; changing a
  key MUST list where it is used and require confirmation.
- **FR-023**: Users MUST be able to see any reference as a reference-list entry and as an
  in-text citation in a chosen citation style.
- **FR-024**: The system MUST provide the citation styles most used across disciplines and
  MUST accept additional styles supplied by the user as style definition files.
- **FR-025**: Users MUST be able to create, list, view, update, and delete bibliographies,
  each with a name, a purpose, a citation style, and an ordering (by author, by year, by
  order of addition, or by hand), and add references to and remove them from a
  bibliography.
- **FR-026**: Users MUST be able to export a bibliography as a file for a writing tool and
  as a formatted reference list in its style.
- **FR-027**: Users MUST be able to link a bibliography to a draft so that the draft's
  citations are those of the bibliography.
- **FR-028**: Users MUST be able to check a bibliography; the check MUST report references
  missing a detail the style requires, references without a citation key, likely
  duplicates, and two versions of the same work cited together.

#### Reading and annotations

- **FR-029**: The system MUST track each reference's reading status (to read, reading, read,
  discarded) and the date each status was set.
- **FR-030**: Users MUST be able to set a reading priority on a reference and view a reading
  queue of unread references ordered by priority, then by date added.
- **FR-031**: Users MUST be able to record their rating of a reference and its relevance to
  their work, and filter on both.
- **FR-032**: Users MUST be able to add to a reference quotes with a page or page range, and
  highlights with a comment and a kind (key claim, method, finding, limitation, question,
  idea).
- **FR-033**: Users MUST be able to record one structured summary per reference covering the
  problem, the method, the findings, the limitations, and their own assessment.
- **FR-034**: Users MUST be able to search and filter annotations across all references by
  text, kind, tag, and reference.
- **FR-035**: Users MUST be able to export a quote with its page and its reference's
  citation in a chosen style, export all reading notes of a reference as a document, and
  see their reading activity for a period.

#### Relations between references

- **FR-036**: Users MUST be able to record that one reference cites, extends, contradicts,
  replicates, or reviews another, with an optional comment; the relation MUST be visible
  from both references.
- **FR-037**: Users MUST be able to record that references are versions of the same work and
  name the preferred version, which is used for citing unless the user chooses otherwise.
- **FR-038**: The system MUST show, for a reference, the works it relates to and the works
  that relate to it, grouped by kind of relation, and MUST list the references most cited
  within the library.
- **FR-039**: The system MUST reject a relation from a reference to itself and a relation
  that already exists.

#### Literature reviews

- **FR-040**: Users MUST be able to create, list, view, update, and delete literature
  reviews, each with a title, a question, a scope, a kind (narrative, systematic, scoping,
  mapping), and a status (planning, searching, screening, extracting, completed).
- **FR-041**: Users MUST be able to record a review's inclusion and exclusion criteria, each
  with a short label.
- **FR-042**: Users MUST be able to record each search carried out: search string, source,
  date, limits applied, and number of results.
- **FR-043**: Users MUST be able to attach references to a review as candidates and record
  which searches found each; a reference MUST be a candidate of a review at most once.
- **FR-044**: The system MUST detect duplicate candidates within a review, keep one, and
  count the others as duplicates removed.
- **FR-045**: Users MUST be able to screen each candidate in two stages — title and
  abstract, then full text — recording at each stage a decision (included or excluded), a
  reason that is required for exclusion, and the time; only candidates included at the
  first stage MUST reach the second.
- **FR-046**: Users MUST be able to list what remains to screen at a stage and screen
  candidates one after another; a changed decision MUST replace the earlier one and be kept
  in the review's history.
- **FR-047**: The system MUST produce a flow summary of a review showing the counts found
  per source, duplicates removed, screened and excluded at each stage grouped by reason,
  and included, and MUST state any inconsistency between those counts.
- **FR-048**: Users MUST be able to mark a review completed, which warns of unscreened
  candidates and prevents further change until it is reopened, and to export a review as a
  document with its question, criteria, searches, flow summary, and included references.

#### Synthesis

- **FR-049**: Users MUST be able to define, for a review, the items to extract from each
  included reference, each with a name, a kind of value (text, number, yes/no, one of a
  list), and whether it is required.
- **FR-050**: Users MUST be able to record a value for each item and included reference,
  with the page it was found on, and to copy a value from an annotation or structured
  summary while keeping the link to its source.
- **FR-051**: The system MUST present extracted values as a matrix of included references by
  items, list the references with missing required items, and export the matrix as a table
  with each reference's citation.
- **FR-052**: Users MUST be able to define themes for a review, assign included references
  to one or more themes, and see the number of references per theme, with themes that have
  none shown as gaps.

#### Common behavior

- **FR-053**: Every value a user supplies — typed, imported from a file, or fetched from a
  catalogue — MUST be validated before anything is stored; invalid input MUST change
  nothing and MUST be reported per value, all together, with what is expected.
- **FR-054**: References, citations, bibliographies, annotations, and reviews MUST support
  tags, notes, and links to any other record, and every creation, change, deletion, merge,
  import, and export MUST be recorded in the workspace's audit trail.
- **FR-055**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-056**: Every command MUST have built-in help, and references, import and export,
  lookup, citations, bibliographies, reading and annotations, relations, reviews, and
  synthesis MUST each have a usage guide with examples.

### Key Entities *(include if feature involves data)*

- **Reference**: One work the researcher may read or cite. Has a kind, a title, authors,
  a year, details particular to its kind, identifiers, a reading status, priority, rating,
  and relevance. Called a "paper" elsewhere in TRCLI.
- **Author**: A person or organization that wrote a reference, with the ways the name is
  written.
- **Citation**: The means of citing a reference in writing, identified by a citation key.
- **Citation Style**: The rules by which a reference is written out, built-in or supplied
  by the user.
- **Bibliography**: A named, ordered list of references assembled for a purpose, with its
  citation style. May be linked to drafts.
- **Annotation**: A quote or highlight on a reference, with page, kind, and comment.
- **Reference Summary**: One structured summary per reference: problem, method, findings,
  limitations, assessment.
- **Relation**: How one reference stands to another: cites, extends, contradicts,
  replicates, reviews, or is a version of.
- **Literature Review**: A structured review with a question, criteria, searches,
  candidates, screening decisions, extraction items, and themes. Called "bibliographic
  research" in the original request.
- **Criterion**: A labelled rule for including or excluding candidates.
- **Search**: One query run during a review: its string, source, date, limits, and result
  count.
- **Candidate**: A reference under consideration in one review, with the searches that
  found it.
- **Screening Decision**: The verdict on one candidate at one stage, with a reason.
- **Extraction Item**: Something to record from every included reference of a review.
- **Extracted Value**: The value of one item for one reference, with its page and source.
- **Theme**: A topic within a review to which included references are assigned.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A new user can add their first reference and save a citation for it in under
  2 minutes without consulting anything other than the built-in help.
- **SC-002**: A user can add a reference from its identifier alone in under 30 seconds, and
  at least 95% of valid identifiers for articles and preprints return complete details.
- **SC-003**: A user can import a bibliography of 1,000 entries in under 1 minute and
  receives a per-entry reason for every entry that was not imported.
- **SC-004**: In a library of 10,000 references, listing, filtering, and searching return
  results in under 2 seconds, and any annotation among 50,000 is found in under 2 seconds.
- **SC-005**: At least 95% of true duplicates in an imported file are reported as likely
  duplicates, and after a merge 0 citations, annotations, or review entries are lost.
- **SC-006**: A bibliography exported for a writing tool is accepted by that tool without
  manual correction in at least 95% of cases.
- **SC-007**: A user can record a quote with its page in under 20 seconds.
- **SC-008**: A user can screen a candidate on title and abstract in under 15 seconds of
  their own time, excluding reading.
- **SC-009**: The counts in a review's flow summary are consistent with one another in 100%
  of reviews, or the inconsistency is stated.
- **SC-010**: A user can produce and export the synthesis matrix of a review with 50
  included references in under 1 minute once the values are entered.
- **SC-011**: With no network connection, 100% of capabilities other than online lookup
  work, and a lookup attempt fails with a clear explanation in under 10 seconds.
- **SC-012**: 100% of invalid inputs, whether typed, imported, or fetched, are rejected
  before any data changes, each with the invalid value named.
- **SC-013**: 90% of first-time users complete the primary task of each story on their
  first attempt using only that story's usage guide.

## Assumptions

- **This specification owns literature.** User Stories 1 and 3 of
  `specs/001-research-workspace` and User Story 1 of `specs/002-research-lifecycle` are
  replaced by this one; those specifications keep a short pointer in their place.
- **It depends on the base workspace** (`specs/001-research-workspace`) for the workspace
  itself, tags, notes, links, drafts, research questions, the audit trail, and common
  behavior. Creating the workspace is specified there.
- **"Reference" is the general word; "paper" is its most common kind.** Other
  specifications that say "paper" mean any reference.
- **"Literature review" is the name used for what the original request called
  "bibliographic research".** The two-stage screening and the flow summary follow the
  practice systematic reviews are expected to report; narrative reviews may skip stages and
  extraction.
- **One screener.** As in the base workspace, one researcher uses the tool, so there is no
  independent double screening or reconciling of disagreements between reviewers.
- **Searches are recorded, not run.** The researcher searches the sources themselves and
  records what they did; the tool does not query bibliographic databases by keyword. Online
  lookup fetches the details of one known identifier at a time.
- **Relations are entered by the researcher.** The tool does not read reference lists out of
  documents or fetch citation counts from outside; "most cited" counts relations recorded
  within the library.
- **Full texts are referenced, not stored or downloaded.** The tool records where a local
  copy is; it does not fetch, store, or read the content of documents, and annotations are
  typed by the researcher, not extracted from a file.
- **Online lookup uses freely available public catalogues** that need no account, contacts
  them only when asked, and sends only the identifier.
- **Citation styles follow the open standard for style definitions** in common use, so
  styles published for other tools can be supplied; which styles are built in is decided at
  planning time and includes at least one author–date and one numeric style.
- **Synchronizing continuously with another reference manager** is specified with
  integrations in `specs/002-research-lifecycle`; this specification covers import and
  export on request.
- **Concepts and glossary terms** linked to references remain in
  `specs/002-research-lifecycle`.
- **Projects** (`specs/003-research-projects`) scope references like any other record; a
  reference may belong to several projects and is stored once.
