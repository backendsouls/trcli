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
