### User Story 1 - Keep a register of venues (Priority: P1)

A researcher records the places where they publish or might: each venue's name, what kind
it is, what it covers, who publishes it, how it reviews, what it costs, what it allows —
and their own view of it: a target, one to watch, or one to avoid. They find venues by
topic, kind, or standing, and share their list with a colleague.

**Why this priority**: The register is what everything else refers to: deadlines belong to
venues, manuscripts are aimed at them, experience accumulates on them. With only this story
a researcher has their own, searchable map of where their field publishes.

**Independent Test**: Record a journal and a conference with their profiles and a ranking
each, mark one as a target and one to avoid with a reason, find venues by a topic word, and
export the list for a colleague.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a venue with a name and a kind —
   journal, conference, workshop, symposium, book series, preprint server, or other —
   **Then** it is stored.
2. **Given** a venue, **When** the researcher records its short name or acronym, publisher
   or organizer, web address, identifiers, the language it publishes in, and the topics it
   covers, **Then** these are saved and shown.
3. **Given** a venue, **When** the researcher records how it reviews — whether reviewers
   know the authors, whether authors know the reviewers, whether reviews are published —
   **Then** this is shown with it.
4. **Given** a venue, **When** the researcher records its costs — a charge to publish, a
   registration fee, with currency and the date the figure was noted — and its policies on
   open access and on preprints, **Then** these are shown.
5. **Given** a venue, **When** the researcher records its typical limits and requirements —
   length, format, anonymity, the template to use — **Then** these are shown, and are
   available when a manuscript is aimed at it.
6. **Given** a venue, **When** the researcher records its standing in one or more ranking
   schemes, each with the scheme's name, the value, and the year, **Then** venues can be
   listed and ordered by a scheme.
7. **Given** a venue, **When** the researcher sets their interest in it — target, watching,
   neutral, or avoid, with a reason for avoid — **Then** venues can be listed by interest,
   and a venue to avoid warns when a manuscript is aimed at it.
8. **Given** venues, **When** the researcher lists them filtered by kind, topic, ranking,
   interest, language, or open-access policy, or searches their names and topics, **Then**
   only matching venues are shown.
9. **Given** a venue, **When** the researcher views it, **Then** its profile, its next
   deadline, the manuscripts aimed at it, and a summary of past experience are shown.
10. **Given** a venue with the same name or identifier as an existing one, **When** it is
    added, **Then** the tool reports the likely duplicate and asks whether to add or cancel.
11. **Given** a venue that changes its name, or is absorbed by another, **When** the
    researcher records the new name or the successor, **Then** the earlier name is kept and
    still finds it.
12. **Given** venues, **When** the researcher exports some or all of them, **Then** a file is
    produced with their profiles and without the researcher's private notes and experience;
    brought into another workspace, they are added and duplicates reported.
13. **Given** a missing name, an unknown kind, a web address or identifier in an invalid
    form, or a negative cost, **When** the researcher saves, **Then** the tool rejects it
    and reports every problem together.
14. **Given** a venue that manuscripts, events, or submissions refer to, **When** the
    researcher deletes it, **Then** the tool lists them and refuses; it can be marked as no
    longer active instead.

---
