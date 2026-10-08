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
