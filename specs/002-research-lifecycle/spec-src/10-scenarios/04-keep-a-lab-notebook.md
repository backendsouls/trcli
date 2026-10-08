### User Story 4 - Keep a lab notebook (Priority: P4)

A researcher keeps a dated journal of what they did, observed, and decided. Entries are
added, never rewritten: a mistake is fixed by adding a correction that points to the earlier
entry. Entries can link to any record, and the notebook can be read by day, by record, or
exported for a period.

**Why this priority**: The notebook is the researcher's own narrative of the work and is
often a formal requirement. It differs from the audit trail, which records what the tool
did, not what the researcher thought or saw.

**Independent Test**: Write three entries on different days, link one to an experiment
run, correct one, and export the notebook for the period showing the original and the
correction.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher adds a notebook entry, **Then** it is
   stored with the date, time, and author.
2. **Given** an entry, **When** the researcher tries to change or delete it, **Then** the
   tool refuses and offers to add a correction instead.
3. **Given** an entry, **When** the researcher adds a correction, **Then** both are kept and
   the original is shown as corrected.
4. **Given** an entry, **When** the researcher links it to records, **Then** each of those
   records lists the entry.
5. **Given** a notebook, **When** the researcher reads it for a date range, a record, or a
   word, **Then** matching entries are shown in time order.
6. **Given** a notebook, **When** the researcher exports a period, **Then** a document is
   produced with every entry and correction in order.
7. **Given** a notebook, **When** an entry is altered outside the tool, **Then** an integrity
   check reports it.

---
