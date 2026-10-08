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
