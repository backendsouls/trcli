### User Story 2 - Keep a document in step with the workspace (Priority: P2)

Weeks after starting the paper, an author has been added, three results have changed, and
the reference list has grown. The researcher asks the tool to refresh the document: the
parts that come from the workspace — author list, affiliations, table of results, list of
figures, reference list, acknowledgement of funders — are brought up to date, and every
word the researcher wrote is left exactly as it was.

**Why this priority**: A template that fills a document once goes stale the next day.
Refreshing is what makes the workspace, rather than the manuscript, the place where facts
are kept — and it is what lets a number in a paper be traced to its run.

**Independent Test**: Apply a template to a draft, write text in a section, add an author
and change a reported result in the workspace, refresh the document, and confirm the author
list and result are updated and the written text is unchanged.

**Acceptance Scenarios**:

1. **Given** a document created from a template, **When** it is created, **Then** the parts
   that come from the workspace are visibly marked as managed by the tool, and everything
   else belongs to the researcher.
2. **Given** such a document and changes in the workspace, **When** the researcher
   refreshes it, **Then** every managed part shows the current content and no other part of
   the document is altered.
3. **Given** a document, **When** the researcher asks what a refresh would change, **Then**
   each managed part that would change is shown with its old and new content, and nothing
   is written.
4. **Given** a managed part that the researcher has edited by hand, **When** the document
   is refreshed, **Then** the tool reports the part, shows both versions, and does not
   overwrite it without the researcher's choice.
5. **Given** a managed part the researcher no longer wants updated, **When** they release
   it, **Then** it becomes ordinary text and is never refreshed again.
6. **Given** a document reporting results, **When** it is refreshed, **Then** each value
   shown is the one the draft currently reports, and a result that a newer run has replaced
   is flagged rather than silently changed.
7. **Given** a placeholder whose record has been deleted or has no value, **When** the
   document is refreshed, **Then** the place is left marked as unresolved and the tool
   lists every unresolved place.
8. **Given** a document, **When** the researcher asks whether it is up to date, **Then** the
   tool answers yes or lists what is out of date, without changing anything.
9. **Given** a document that was moved or renamed outside the tool, **When** the researcher
   refreshes it by its new location, **Then** it is recognized as the same document.
10. **Given** a file that was not created from a template, **When** the researcher asks to
    refresh it, **Then** the tool says it has nothing to manage there and changes nothing.

---
