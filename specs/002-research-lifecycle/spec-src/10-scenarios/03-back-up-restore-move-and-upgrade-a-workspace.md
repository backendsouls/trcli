### User Story 3 - Back up, restore, move, and upgrade a workspace (Priority: P3)

A researcher makes a complete backup of a workspace as a single file, restores it on the
same or another machine, exports the whole workspace in an open, documented form that can be
read without the tool, and, when a new version of the tool changes how workspaces are kept,
upgrades the workspace safely.

**Why this priority**: A workspace holds years of work. Researchers will not trust it with
that work until they can get everything out, put it back, and survive an upgrade.

**Independent Test**: Back up a populated workspace, restore it to an empty location, and
confirm that every record, link, and audit entry is identical; then upgrade a workspace made
by an older version and confirm nothing is lost.

**Acceptance Scenarios**:

1. **Given** a populated workspace, **When** the researcher requests a backup, **Then** a
   single file is produced containing every record, link, and audit entry, and its
   completeness is verified before success is reported.
2. **Given** a backup file, **When** the researcher restores it to an empty location,
   **Then** the restored workspace is identical to the original at the time of backup.
3. **Given** a backup file and a location that already holds a workspace, **When** the
   researcher restores, **Then** the tool refuses unless the researcher explicitly chooses
   to replace it.
4. **Given** a damaged or incomplete backup file, **When** the researcher restores it,
   **Then** the tool reports the damage and changes nothing.
5. **Given** a workspace, **When** the researcher requests a full export, **Then** the
   records are written in an open, documented form that can be read without the tool.
6. **Given** a workspace created by an older version, **When** a newer version opens it,
   **Then** the tool says an upgrade is needed, changes nothing until asked, and takes a
   backup before upgrading.
7. **Given** an upgrade that fails part-way, **When** the failure occurs, **Then** the
   workspace is returned to its state before the upgrade.
8. **Given** a workspace created by a newer version than the tool in use, **When** the tool
   opens it, **Then** it refuses to modify it and explains why.

---
