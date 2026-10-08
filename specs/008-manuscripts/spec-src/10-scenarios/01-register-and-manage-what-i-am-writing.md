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
