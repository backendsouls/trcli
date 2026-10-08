### User Story 1 - Create and find a workspace (Priority: P1)

A researcher creates a workspace in a directory of their choice and gives it a name. From
then on, wherever they stand inside that directory or beneath it, TRCLI finds the workspace
by itself. Outside any workspace, it says so and explains what to do. They can keep
several workspaces, entirely separate from one another.

**Why this priority**: Nothing can be stored before there is somewhere to store it. This is
the first thing every user does and the first thing that must work.

**Independent Test**: Create a workspace, run a command from a subdirectory three levels
down and confirm the workspace is found; run a command from outside and confirm the
explanation; try to create a second workspace in the same place and confirm it is refused.

**Acceptance Scenarios**:

1. **Given** a directory with no workspace, **When** the researcher creates one there with a
   name, **Then** a workspace is created and reported as ready, with where it is.
2. **Given** a workspace, **When** the researcher runs a command from any directory beneath
   it, **Then** the workspace is found without being named.
3. **Given** a directory outside any workspace, **When** the researcher runs a command that
   needs one, **Then** the tool says there is no workspace here, explains how to create or
   point to one, creates nothing, and changes nothing.
4. **Given** a directory that already holds a workspace, **When** the researcher tries to
   create one there, **Then** the tool refuses and leaves the existing workspace untouched.
5. **Given** a workspace, **When** the researcher views it, **Then** its name, description,
   location, format version, and the number of records of each kind are shown.
6. **Given** a workspace, **When** the researcher changes its name, description, or the name
   under which their own actions are recorded, **Then** the change is saved.
7. **Given** two workspaces in different directories, **When** the researcher works in one,
   **Then** nothing of the other is visible or affected.
8. **Given** a workspace elsewhere, **When** the researcher names it explicitly for one
   command, or for a session, **Then** that workspace is used whatever the current
   directory.
9. **Given** a researcher who wants one workspace reachable from anywhere, **When** they set
   it as their default, **Then** it is used whenever no workspace is found from where they
   stand, and the tool says which workspace it used.
10. **Given** a workspace nested inside another workspace's directory, **When** a command is
    run inside the inner one, **Then** the nearest workspace is used.
11. **Given** a workspace created by a newer version of the tool, **When** an older version
    opens it, **Then** it refuses to change anything and explains why.
12. **Given** a workspace created by an older version, **When** a newer version opens it,
    **Then** the tool says an upgrade is needed, changes nothing until asked, and keeps a
    copy of the workspace as it was before upgrading.
13. **Given** a workspace whose stored data has been damaged or edited by hand into an
    invalid state, **When** it is opened, **Then** the tool reports what is wrong and where,
    and does not overwrite it.
14. **Given** a missing or empty name, or a directory that cannot be written to, **When**
    the researcher creates a workspace, **Then** the tool rejects it and explains why.

---
