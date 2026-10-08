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
