### User Story 2 - Work with any record the same way (Priority: P2)

Whatever a researcher is working with — a reference, an experiment, a task, a person — the
same handful of actions works the same way: add one, list them, look at one, change it,
remove it; give it tags and notes; link it to any other record. Each record has a short
name they can type, and typing just the beginning is enough. Once they have learned one
kind of record, they know them all.

**Why this priority**: Consistency is what makes a tool with dozens of record kinds
learnable. It is also what keeps the workspace whole: links that are visible from both
ends, and deletions that never leave something pointing at nothing.

**Independent Test**: With two different kinds of record, add one of each, refer to each by
the start of its short name, tag one, add a note to the other, link them, list each kind
with a filter and a search, and delete one — confirming the link is listed first and gone
afterwards.

**Acceptance Scenarios**:

1. **Given** any kind of record, **When** the researcher adds one, **Then** it receives a
   short name that says what kind it is, is unique in the workspace, and never changes.
2. **Given** a record's short name, **When** the researcher types only its beginning,
   **Then** the record is found if the beginning matches exactly one record.
3. **Given** a beginning that matches several records, **When** it is used, **Then** the
   tool lists the matches and changes nothing.
4. **Given** a short name that matches nothing, **When** it is used, **Then** the tool says
   it was not found and suggests close matches where there are any.
5. **Given** any kind of record, **When** the researcher lists them, **Then** they can
   filter, sort, search by words in the main text, and limit how many are shown, in the
   same way for every kind.
6. **Given** any record, **When** the researcher changes it, **Then** only what they named
   is changed.
7. **Given** any record, **When** the researcher adds tags or a dated note, **Then** these
   are shown with the record, and records of any kind can be found by tag.
8. **Given** any two records, of the same or different kinds, **When** the researcher links
   them, optionally saying how they relate, **Then** the link is visible from both.
9. **Given** a record that other records refer to, **When** the researcher deletes it,
   **Then** the tool lists what refers to it and requires explicit confirmation.
10. **Given** a confirmed deletion, **When** it completes, **Then** no link, tag, or note
    still points to the deleted record.
11. **Given** a deletion that a feature forbids — because history would be lost — **When**
    it is attempted, **Then** the tool refuses, says what stands in the way, and offers the
    alternative that feature provides.
12. **Given** a command that asks for confirmation, **When** it is run where nobody can
    answer, **Then** it fails at once without changing anything, unless the researcher
    stated beforehand that the answer is yes.
13. **Given** the same action on two kinds of record, **When** the researcher compares how
    it is invoked, **Then** the words, the order, and the options are the same.
14. **Given** a list with nothing in it, **When** it is shown, **Then** the tool says so in
    one line and does not treat it as a failure.

---
