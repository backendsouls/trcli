### User Story 3 - Keep a workspace in step across my machines (Priority: P3)

A researcher publishes their workspace to their Drive from the laptop, then obtains it on
the lab workstation. From then on they send their changes when they finish working on one
machine and receive them when they start on the other. If they changed the same thing on
both without syncing in between, the tool shows both versions and lets them choose; it never
decides silently and never damages the workspace.

**Why this priority**: This is what the request asks for — sync — and the capability
researchers with two machines need most. It depends on the connection existing.

**Independent Test**: Publish a workspace from machine A, obtain it on machine B, add a
reference on A and a task on B, sync both, and confirm both machines have both; then change
the same title on both machines, sync, and confirm the conflict is shown and resolved by
choice.

**Acceptance Scenarios**:

1. **Given** a workspace and a connection that offers sync, **When** the researcher
   publishes the workspace, **Then** a remote copy is created and the workspace is linked to
   it.
2. **Given** a published workspace, **When** the researcher obtains it on another machine,
   **Then** a complete local workspace is created there, linked to the same remote copy.
3. **Given** a linked workspace with local changes, **When** the researcher sends them,
   **Then** the remote copy receives them and the tool reports how many changes were sent.
4. **Given** a linked workspace and changes sent from another machine, **When** the
   researcher receives them, **Then** the local workspace is updated and the tool reports
   what changed, by kind of record.
5. **Given** changes on both machines to different records, **When** both sync, **Then**
   both workspaces end up identical with every change present.
6. **Given** changes on both machines to the same detail of the same record, **When** they
   sync, **Then** the conflict is shown with both values, their machines, and their times,
   and nothing is overwritten until the researcher chooses.
7. **Given** a record deleted on one machine and changed on the other, **When** they sync,
   **Then** this is shown as a conflict like any other.
8. **Given** records that are only ever added to — audit entries, notebook entries, frozen
   pre-registrations — **When** machines sync, **Then** entries from every machine are kept
   and none is altered.
9. **Given** a linked workspace, **When** the researcher asks for its sync status, **Then**
   the tool shows what is waiting to be sent, whether the remote copy has changes to
   receive, when the last sync was, and any open conflicts.
10. **Given** a transfer that is interrupted — the network drops, the machine sleeps —
    **When** it stops, **Then** both the local workspace and the remote copy remain valid,
    and the transfer can be repeated safely.
11. **Given** a linked workspace with changes not yet sent, **When** the researcher finishes
    a command, **Then** the tool can remind them, and, if they chose automatic sync, sends
    the changes by itself.
12. **Given** a machine that has not synced for a long time, **When** it syncs, **Then** it
    receives everything since, however much there is.
13. **Given** a workspace, **When** the researcher unlinks it from its remote copy, **Then**
    the local workspace stays complete and usable, and the remote copy is left as it is
    unless they ask to delete it.
14. **Given** a remote copy made by a newer version of TRCLI than the one on this machine,
    **When** the researcher syncs, **Then** the tool refuses, explains, and changes nothing.
15. **Given** two machines syncing at the same moment, **When** both send, **Then** one
    succeeds, the other is told to receive first, and nothing is lost.

---
