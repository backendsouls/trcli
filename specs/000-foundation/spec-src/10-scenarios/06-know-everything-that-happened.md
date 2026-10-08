### User Story 6 - Know everything that happened (Priority: P6)

Everything that changes in a workspace is written down as it happens: what was done, to
what, by whom, when, and what exactly changed. A researcher — or a supervisor, a reviewer,
an auditor — can look through this record, check that nobody has tampered with it, and hand
over a report of it. Separately, the tool keeps simple telemetry about its own use for the
researcher's benefit, which stay on their machine and can be turned off.

**Why this priority**: The record of what happened is what makes research in the tool
accountable, and several features are built on it (reports, sync, reproducibility). It is
written from the first change onward, so it belongs to the foundation; looking through it
becomes valuable once there is something to look at.

**Independent Test**: Make a known series of changes, look up the record for one item and
for a range of dates and confirm every change is there with what changed; alter one entry
outside the tool and confirm the check reports it; turn telemetry off and confirm none
are recorded while the record of changes continues.

**Acceptance Scenarios**:

1. **Given** any record is created, changed, or deleted, **When** the action completes,
   **Then** an entry records the action, the record, who acted, when, and what changed.
2. **Given** a change that fails or is refused, **When** it ends, **Then** neither the change
   nor an entry for it exists; a change and its entry are made together or not at all.
3. **Given** the record of what happened, **When** the researcher filters it by item, kind
   of item, who acted, kind of action, or range of dates, **Then** only matching entries
   are shown, newest first.
4. **Given** an entry about a record that has since been deleted, **When** it is shown,
   **Then** it still says what the record was called.
5. **Given** the record of what happened, **When** an entry is altered or removed outside
   the tool, **Then** a check reports that it has been tampered with and where the first
   problem is.
6. **Given** the tool, **When** the researcher looks for a way to change or remove an entry,
   **Then** there is none.
7. **Given** the record of what happened, **When** the researcher exports a range of it,
   **Then** a report is produced that can be handed to someone else.
8. **Given** use of the tool, **When** telemetry is on, **Then** which commands were
   used, how long they took, and whether they succeeded are recorded locally, and the
   researcher can view them.
9. **Given** telemetry, **When** the researcher turns it off, **Then** no more is
   recorded, and the record of what happened continues unchanged.
10. **Given** telemetry, **When** anything is recorded, **Then** it stays in the workspace
    and is never sent anywhere.
11. **Given** something secret — a credential, a passphrase — **When** anything is recorded,
    **Then** it is never included.
12. **Given** who acted, **When** it is recorded, **Then** it is the name the researcher set
    for themselves, or otherwise the name they are known by on their machine.

---
