<!-- GENERATED FILE: do not edit. Edit the parts in spec-src/ and run scripts/build-spec.sh -->

# Feature Specification: TRCLI Integrations and Google Drive

**Feature Branch**: `010-integrations`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add integrations spec, this feature has the core for integrations and one concrete integration google drive (it is possible to sync directly via google oauth?)"

## Overview

Until now TRCLI has kept everything on one machine and has sent nothing anywhere. That is
safe, and it is also a limit: a researcher works on a laptop and a lab workstation, wants a
copy of years of work somewhere other than one disk, and keeps datasets too large to carry
around. This specification lets a workspace reach **outside services**, deliberately and
under the researcher's control.

It has two parts:

1. **The integration core** — what every connection to an outside service has in common:
   how the researcher sees what is available, connects, sees exactly what access was
   granted, checks that it works, sees everything that was sent and received, and
   disconnects. The core defines a small set of **capabilities** a service may offer, so
   that the rest of TRCLI uses any service the same way.
2. **One integration built on it: Google Drive** — signing in with a Google account and
   using the researcher's Drive to synchronize a workspace between their machines, to hold
   the files a workspace refers to, and to keep backups.

### Capabilities an integration may offer

| Capability | What it gives the researcher | Google Drive |
|------------|------------------------------|--------------|
| **Workspace sync** | The same workspace on several of their machines, kept in step | yes |
| **File storage** | A place for the files records refer to (datasets, figures, manuscripts), fetched where needed | yes |
| **Backup destination** | Somewhere off the machine to keep backups | yes |

Later integrations — another storage service, an institutional repository, a reference
manager — declare which of these they offer, or add new capabilities, without changing how
the researcher works.

### The answer to the question in the request

Yes: a workspace can synchronize directly with Google Drive after the researcher signs in
with their Google account. The researcher approves access in their browser; TRCLI never
sees their password; and TRCLI asks only for access to the files it creates itself, not to
the rest of the Drive. What this specification requires of that sign-in and sync is below;
the technical findings behind the answer are recorded in
[notes/google-sign-in.md](./notes/google-sign-in.md) for planning.

### The concepts, in one picture

```text
 Researcher ── connects ──▶ Connection ── to ──▶ Integration (Google Drive, …)
                               │  access granted, account, health        │ offers
                               │                                         ▼
                               │                                   Capabilities
        ┌──────────────────────┼───────────────────────────┐
        ▼                      ▼                           ▼
  Workspace sync          File storage              Backup destination
  machine A ⇄ remote ⇄ B  record's file ⇄ remote    backup ──▶ remote
        │                      │                           │
        └───────── every transfer ──▶ Transfer log ◀───────┘
```

## User Scenarios & Testing *(mandatory)*

### User Story 1 - See, connect, and control integrations (Priority: P1)

A researcher looks at which outside services TRCLI can work with, what each one offers, and
what access each would need. They connect one, see that the connection works and exactly
what it is allowed to do, and can disconnect it at any time, after which nothing more is
sent and the access is given back.

**Why this priority**: Every integration starts here, and a researcher will not connect
research data to an outside service without being able to see and undo exactly what they
agreed to. This is the foundation the rest stands on.

**Independent Test**: List the available integrations, connect one, view the connection's
status and granted access, disconnect it, and confirm that a command needing it afterwards
explains that it is not connected.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher lists integrations, **Then** each is shown
   with its name, what it offers, the access it needs, and whether it is connected.
2. **Given** an integration, **When** the researcher asks about it before connecting,
   **Then** the tool explains in plain words what will be sent to the service, what will be
   read from it, and what will be stored on the machine.
3. **Given** an integration, **When** the researcher connects it, **Then** they are taken
   through the service's own approval, and on success the connection is stored with the
   account it belongs to and the access granted.
4. **Given** a connection, **When** the researcher asks for its status, **Then** the tool
   shows the account, the access granted, when it was last used, whether it currently
   works, and the space used and available where the service reports it.
5. **Given** a connection, **When** the researcher disconnects it, **Then** the stored
   credentials are removed from the machine, the service is asked to withdraw the access,
   and the tool says whether anything remains on the service and how to remove it.
6. **Given** a connection whose access has expired or was withdrawn at the service, **When**
   a command needs it, **Then** the tool says so and offers to connect again, without
   losing any local work.
7. **Given** no network connection, **When** a command needs an integration, **Then** the
   tool says the service cannot be reached, every other command keeps working, and nothing
   is lost.
8. **Given** the same integration, **When** the researcher connects a second account under
   another name, **Then** both connections exist and each command says which one it uses.
9. **Given** a connection, **When** the researcher limits what it may be used for (for
   example backups only), **Then** commands outside that use are refused.
10. **Given** a workspace, **When** it has no connection, **Then** TRCLI behaves exactly as
    before this specification and sends nothing anywhere.
11. **Given** a command that would send data to a service, **When** it is the first time for
    that connection and kind of data, **Then** the tool says what will be sent and asks for
    confirmation.
12. **Given** an integration name that does not exist, **When** the researcher connects it,
    **Then** the tool rejects it and lists those available.

---

### User Story 2 - Sign in with Google (Priority: P2)

A researcher connects their Google Drive. TRCLI opens Google's own sign-in page in their
browser; they choose their account and approve; TRCLI receives permission to work with the
files it creates in their Drive — and nothing else. On a machine without a browser, such as
a lab server reached remotely, they approve from their phone or another computer by typing a
short code.

**Why this priority**: Google Drive is the one concrete integration, and nothing else in it
can work before sign-in does. It must be both easy and visibly safe.

**Independent Test**: Connect Google Drive from a desktop with a browser and confirm the
connection shows the account and the limited access; connect from a machine without a
browser using a code; then confirm that a file in the Drive not created by TRCLI cannot be
read by it.

**Acceptance Scenarios**:

1. **Given** a machine with a browser, **When** the researcher connects Google Drive,
   **Then** Google's sign-in page opens, and after approval the tool confirms the connection
   and names the account.
2. **Given** a machine without a browser, **When** the researcher connects Google Drive,
   **Then** the tool shows a web address and a short code to enter on any other device, and
   completes the connection once they approve there.
3. **Given** the sign-in, **When** it takes place, **Then** the researcher's password is
   entered only on Google's page and is never seen, asked for, or stored by TRCLI.
4. **Given** the approval page, **When** it lists what TRCLI asks for, **Then** it is access
   to the files and folders TRCLI itself creates, and not to the researcher's other files.
5. **Given** a connected Drive, **When** TRCLI is asked to read a file it did not create,
   **Then** it cannot, and says that its access does not include that file.
6. **Given** a successful sign-in, **When** credentials are kept for later use, **Then**
   they are kept in the machine's protected store for secrets, not in the workspace, not in
   settings files, and never shown on screen or in logs.
7. **Given** a connection, **When** the researcher uses TRCLI days or months later, **Then**
   it keeps working without signing in again, until the researcher or Google withdraws the
   access.
8. **Given** the researcher closes the browser or denies the approval, **When** the sign-in
   ends, **Then** nothing is stored and the tool says the connection was not made.
9. **Given** a sign-in that is not completed within a stated time, **When** the time passes,
   **Then** the tool stops waiting, says so, and stores nothing.
10. **Given** a connected Drive, **When** the researcher chooses where TRCLI keeps its
    content, **Then** it is a folder TRCLI creates, visible to the researcher in their Drive
    under a name they choose.
11. **Given** an institution that manages its members' Google accounts and does not allow
    the access, **When** the researcher tries to connect, **Then** the tool reports that the
    institution blocked it and what to ask the administrator.
12. **Given** a machine with no protected store for secrets, **When** the researcher
    connects, **Then** the tool says so and asks whether to keep the credentials in a file
    readable only by them, or not to connect.

---

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

### User Story 4 - Keep the files records refer to in the Drive (Priority: P4)

Records in a workspace refer to files — a dataset, a figure, a manuscript, a local copy of
a paper. A researcher sends chosen files to their Drive, so that on another machine the
record's file can be fetched when it is needed instead of being copied by hand. The tool
checks every transfer against the file's fingerprint.

**Why this priority**: Syncing records without their files leaves the second machine with
references to files that are not there. This completes sync, and matters most for data too
large to keep everywhere.

**Independent Test**: Send a dataset's files to the Drive from machine A, sync, fetch them
on machine B, and confirm the dataset verifies against its recorded fingerprint there.

**Acceptance Scenarios**:

1. **Given** a record with a local file and a connection that offers file storage, **When**
   the researcher sends the file, **Then** it is stored remotely and the record shows that
   a remote copy exists.
2. **Given** a record whose file exists remotely and not locally, **When** the researcher
   fetches it, **Then** the file is placed where the record expects it and checked against
   its fingerprint.
3. **Given** a fetched file that does not match its fingerprint, **When** the check fails,
   **Then** the file is not put in place, and the tool reports the mismatch.
4. **Given** records with files, **When** the researcher asks which files are where,
   **Then** each is shown as local only, remote only, both and identical, or both and
   different.
5. **Given** several records, **When** the researcher sends or fetches the files of all
   records of a kind, of a project, or matching a tag, **Then** all are transferred and a
   summary says what was done.
6. **Given** a large file, **When** it is transferred, **Then** progress is shown, the
   transfer can be interrupted, and repeating it continues rather than starts again.
7. **Given** a file already identical on the other side, **When** it is sent or fetched
   again, **Then** nothing is transferred.
8. **Given** a file that changed locally after being sent, **When** the researcher asks,
   **Then** it is shown as different, and sending it keeps the earlier remote version
   retrievable for the dataset version that used it.
9. **Given** a dataset flagged as holding personal or sensitive data, **When** the
   researcher sends its files, **Then** the tool warns, requires explicit confirmation, and
   records the acknowledgement.
10. **Given** a file to send larger than the space left in the Drive, **When** the
    researcher sends it, **Then** the tool says so before transferring anything.
11. **Given** a command that needs a file that is remote only (verifying a dataset, starting
    a run), **When** it runs, **Then** the tool says the file must be fetched first and
    offers to fetch it.
12. **Given** a record whose remote file was deleted in the Drive by hand, **When** the
    researcher fetches it, **Then** the tool reports it as missing remotely and changes
    nothing locally.
13. **Given** files, **When** the researcher frees local space for files that have an
    identical remote copy, **Then** only those are removed locally, after confirmation.

---

### User Story 5 - Back up to the Drive (Priority: P5)

A researcher keeps backups of the workspace somewhere other than their own disk: they
create a backup directly in their Drive, see the backups kept there, keep only the most
recent ones, and restore from one on any machine.

**Why this priority**: An off-machine backup is what protects years of work from a lost
laptop. It is simpler than sync and is useful even to someone who works on one machine, but
it builds on the backup already specified elsewhere.

**Independent Test**: Create a backup in the Drive, list remote backups, restore the
workspace from it on another machine into an empty location, and confirm it is identical.

**Acceptance Scenarios**:

1. **Given** a connection that offers a backup destination, **When** the researcher creates
   a backup there, **Then** the backup is made, verified, sent, and verified again after
   arrival.
2. **Given** remote backups, **When** the researcher lists them, **Then** each is shown with
   its date, size, the workspace it is of, and the version of the tool that made it.
3. **Given** a remote backup, **When** the researcher restores from it into an empty
   location, **Then** the workspace obtained is identical to the one backed up.
4. **Given** a number of backups to keep, **When** a new backup succeeds, **Then** the
   oldest beyond that number are removed remotely, and the tool says which.
5. **Given** a backup that fails to send or to verify, **When** it fails, **Then** no
   earlier backup is removed and the tool says the backup was not made.
6. **Given** a remote backup, **When** the researcher asks to verify it, **Then** it is
   checked without restoring.
7. **Given** a researcher who chose to protect backups with a passphrase, **When** a backup
   is made, **Then** it cannot be read in the Drive without the passphrase, and restoring
   asks for it.
8. **Given** a lost passphrase, **When** the researcher tries to restore, **Then** the tool
   says plainly that the backup cannot be recovered without it.
9. **Given** a backup that includes the files records refer to, **When** the researcher
   asks for it, **Then** those files are included, and without asking they are not.
10. **Given** a workspace that is also synced, **When** a backup is made, **Then** it is
    independent of sync and can be restored even if the remote copy is gone.

---

### User Story 6 - See and limit what leaves my machine (Priority: P6)

A researcher can always answer "what has this tool sent, where, and when?". They look at
the record of every transfer, preview what a command would send before it does, exclude
kinds of record or particular records from ever being sent, and choose to protect everything
sent with a passphrase so that the service holds only unreadable content.

**Why this priority**: Research data is often confidential, unpublished, or about people.
Control is what makes the earlier stories acceptable to use. It refines them and can come
last, but its guarantees shape them.

**Independent Test**: Preview a sync and confirm nothing is sent; exclude a dataset and
confirm sync never sends it; turn on passphrase protection and confirm the remote content
cannot be read without it; review the transfer log for the session.

**Acceptance Scenarios**:

1. **Given** any command that sends or receives, **When** the researcher asks for a preview,
   **Then** what would be transferred is listed and nothing is transferred.
2. **Given** past transfers, **When** the researcher views the transfer log, **Then** each
   entry shows when, which connection, which direction, what kind of content, how much, and
   whether it succeeded, and the log can be filtered and exported.
3. **Given** a workspace, **When** the researcher excludes kinds of record, records with a
   tag, or particular records from leaving the machine, **Then** sync, file storage, and
   backup to a service all leave them out, and say that they did.
4. **Given** records marked private or datasets flagged sensitive, **When** nothing else is
   set, **Then** their files are not sent without explicit confirmation each first time.
5. **Given** passphrase protection turned on for a connection, **When** anything is sent,
   **Then** it is made unreadable before leaving the machine, and names of records and
   files are not readable at the service either.
6. **Given** passphrase protection, **When** another machine obtains the workspace, **Then**
   it asks for the passphrase once and keeps it in the machine's protected store.
7. **Given** a service that asks the tool to slow down or is temporarily failing, **When** a
   transfer is under way, **Then** the tool waits and retries a limited number of times,
   says what it is doing, and stops with a clear message if it cannot continue.
8. **Given** a connection, **When** the researcher asks what is stored at the service,
   **Then** the tool lists it by kind and size, and can delete all of it on request after
   confirmation.
9. **Given** credentials, **When** anything is printed, logged, exported, backed up, or
   synced, **Then** credentials and passphrases are never included.
10. **Given** telemetry, **When** integrations are in use, **Then** telemetry still stays on
    the machine; connecting a service never causes it to be sent.
11. **Given** an exclusion that would leave a synced record pointing at an excluded one,
    **When** it is set, **Then** the tool says which links will be incomplete on other
    machines.

---

### Edge Cases

- The researcher signs in with a different Google account than the one the workspace was
  published with: the tool notices that the remote copy is not there, says which account
  was used before, and changes nothing.
- The researcher deletes or renames TRCLI's folder in their Drive by hand: the next transfer
  reports that the remote copy is missing or moved, offers to publish again or to relink,
  and never recreates it silently.
- The researcher edits a file inside TRCLI's folder by hand: the tool detects that the
  remote content is not what it wrote and refuses to use it.
- The Drive is full: the transfer stops before or as soon as this is known, nothing local is
  affected, and the tool says how much space is needed.
- Access is withdrawn at Google while a transfer is running: the transfer stops, the remote
  copy stays valid, and the tool asks to connect again.
- The machine's clock is wrong: the order of changes does not depend on clocks alone, and
  the tool reports a clock that is clearly wrong.
- The same workspace is published twice from two machines that never synced: the tool
  refuses to merge two unrelated histories and explains the options.
- A machine is lost or stolen: from another machine the researcher disconnects that
  machine's access, and is told that the workspace content already on it cannot be recalled.
- Two workspaces are published to the same connection: each has its own remote copy, and
  listing shows both.
- A workspace is restored from an old backup and then synced: the tool treats the restored
  state as old and receives what is newer, after showing what will change.
- A file's name is not allowed by the service, or two files differ only by letter case: the
  tool stores them under safe names of its own and keeps the real names itself.
- A referenced file is a very large directory with many small files: it is transferred as a
  whole with one progress indication, and fetched as a whole.
- Sync is asked for while an experiment run is in progress: the run's state is sent as it
  is; another machine sees it as running elsewhere and cannot resume it.
- The remote copy was written by an older version of TRCLI: the tool upgrades it only when
  asked, after a backup, and other machines are told to upgrade before syncing.
- The network is slow or drops repeatedly: transfers resume where they stopped and never
  leave a half-written remote copy in use.
- A command is run by another program, without a person present, and sign-in is needed: it
  fails immediately with a message, and never waits for a browser.
- The passphrase is changed: content already at the service is protected again with the new
  one, or the tool says clearly that old content still needs the old passphrase.
- An integration is removed from a later version of the tool while connections to it exist:
  the connections are shown as unsupported, local data is untouched, and the tool says how
  to retrieve what is at the service.

## Requirements *(mandatory)*

### Functional Requirements

#### Integration core

- **FR-001**: The system MUST present every outside service it can work with as an
  integration with a name, a description, the capabilities it offers, the access it needs,
  and a plain-language statement of what is sent to it, read from it, and stored locally.
- **FR-002**: The system MUST define capabilities that integrations offer — workspace sync,
  file storage, and backup destination — and every feature that uses a capability MUST work
  the same way with any integration that offers it.
- **FR-003**: Users MUST be able to list integrations, connect one, name the connection,
  view its status, limit the capabilities it may be used for, and disconnect it.
- **FR-004**: A connection MUST record the integration, the account, the access granted, the
  capabilities allowed, when it was made, and when it was last used.
- **FR-005**: Connection status MUST show whether the connection currently works and, where
  the service reports them, the space used and available.
- **FR-006**: Disconnecting MUST remove the stored credentials from the machine, ask the
  service to withdraw the access, and tell the user what remains at the service and how to
  remove it. It MUST NOT alter or delete local data.
- **FR-007**: Users MUST be able to hold several connections to the same integration under
  different names, and every command that uses a connection MUST say which one.
- **FR-008**: When a connection's access has expired or been withdrawn, or the service
  cannot be reached, the system MUST say which, MUST offer to connect again where that
  helps, MUST NOT lose local work, and MUST leave every command that does not need the
  service working.
- **FR-009**: With no connection, the system MUST behave exactly as specified elsewhere and
  MUST send nothing to any service.
- **FR-010**: The system MUST be able to gain further integrations and further capabilities
  without change to the commands users already know for existing ones.

#### Credentials and consent

- **FR-011**: The system MUST obtain access to a service only through that service's own
  approval, MUST NOT ask for, see, or store the user's password for the service, and MUST
  request the narrowest access that the capabilities in use need.
- **FR-012**: Credentials MUST be kept in the machine's protected store for secrets. When
  none exists the system MUST say so and let the user choose between a file readable only
  by them and not connecting.
- **FR-013**: Credentials and passphrases MUST NOT appear on screen, in logs, in the audit
  trail, in the transfer log, in exports, in backups, or in anything synced, and MUST NOT
  be stored in the workspace or in settings files.
- **FR-014**: The first time a connection is used to send a given kind of content, the
  system MUST say what will be sent and require confirmation.
- **FR-015**: When it cannot ask a person (not run from a terminal), a command that needs
  sign-in or confirmation MUST fail at once with an explanation and MUST NOT wait.

#### Google Drive

- **FR-016**: The system MUST provide a Google Drive integration offering workspace sync,
  file storage, and backup destination.
- **FR-017**: Connecting Google Drive MUST use Google's sign-in in the user's browser; on a
  machine without a browser the system MUST offer approval from another device by a web
  address and a short code.
- **FR-018**: The Google Drive integration MUST request access only to the files and folders
  the system itself creates in the user's Drive, and MUST NOT be able to read, list, change,
  or delete the user's other files.
- **FR-019**: Everything the system keeps in the Drive MUST be inside one folder it creates,
  named by the user, and visible to them.
- **FR-020**: A connection MUST keep working across sessions without signing in again until
  the user or Google withdraws the access.
- **FR-021**: A sign-in that is denied, abandoned, or not completed within a stated time
  MUST store nothing and MUST be reported as not made.
- **FR-022**: When an institution's policy blocks the access, the system MUST report that
  this is the cause and what to ask the administrator.
- **FR-023**: The system MUST detect that the signed-in account is not the one a workspace
  was published with, and that its folder is missing, moved, or contains content it did not
  write, and in each case MUST explain and change nothing.

#### Workspace sync

- **FR-024**: Users MUST be able to publish a workspace to a connection that offers sync,
  creating a remote copy linked to the workspace, and to obtain a published workspace on
  another machine as a complete local workspace linked to the same remote copy.
- **FR-025**: Users MUST be able to send local changes, receive remote changes, and do both
  in one step; after machines have exchanged all changes their workspaces MUST be
  identical.
- **FR-026**: The system MUST show sync status: changes waiting to be sent, whether there is
  something to receive, the time of the last sync, and open conflicts.
- **FR-027**: Changes made on different machines to different records, or to different
  details of the same record, MUST be combined without asking.
- **FR-028**: Changes made on different machines to the same detail of the same record, and
  a deletion on one machine with a change on another, MUST be presented as conflicts with
  both versions, their machines, and their times, and MUST NOT be resolved without the
  user's choice. No change may be lost silently.
- **FR-029**: Records that are only added to — audit entries, notebook entries, frozen
  pre-registrations, saved reports — MUST keep every entry from every machine unaltered, and
  the audit trail MUST remain a single verifiable history.
- **FR-030**: A workspace MUST remain fully usable without a network connection; sync MUST
  be able to happen later, however long the gap.
- **FR-031**: An interrupted transfer MUST leave both the local workspace and the remote
  copy valid and MUST be safely repeatable. The working storage of a local workspace MUST
  never itself be placed where a service modifies it.
- **FR-032**: When two machines send at the same moment, one MUST succeed and the other
  MUST be told to receive first; neither machine's changes may be lost.
- **FR-033**: Users MUST be able to be reminded of changes not yet sent, and to turn on
  automatic sync after commands that change the workspace; automatic sync MUST never
  resolve a conflict by itself and MUST never make a command fail.
- **FR-034**: Users MUST be able to unlink a workspace from its remote copy, leaving the
  local workspace complete, and separately to delete the remote copy after confirmation.
- **FR-035**: The system MUST refuse to sync with a remote copy written by a newer version
  than it understands, MUST upgrade a remote copy only when asked and after a backup, and
  MUST refuse to merge two workspaces that do not share a history.
- **FR-036**: The order in which changes are combined MUST NOT depend on machines' clocks
  alone, and the system MUST report a clock that is clearly wrong.
- **FR-037**: Each machine linked to a remote copy MUST be identifiable by a name the user
  can set, shown on conflicts and in the transfer log; users MUST be able to list linked
  machines and remove one's access.

#### File storage

- **FR-038**: Users MUST be able to send the files a record refers to to a connection that
  offers file storage, and to fetch them on another machine to the place the record
  expects.
- **FR-039**: Every sent and fetched file MUST be checked against its fingerprint; a file
  that does not match MUST NOT be put in place, and the mismatch MUST be reported.
- **FR-040**: The system MUST show, for each referenced file, whether it is local only,
  remote only, both and identical, or both and different.
- **FR-041**: Users MUST be able to send or fetch the files of one record, of all records of
  a kind, of a project, or matching a tag, and receive a summary.
- **FR-042**: Transfers MUST show progress, be interruptible, continue rather than restart
  when repeated, and transfer nothing when the two sides are already identical.
- **FR-043**: When a file that was sent has changed locally, sending it again MUST keep the
  earlier remote content retrievable for any dataset version, result, or manuscript version
  that recorded it.
- **FR-044**: Before sending, the system MUST check that enough space is available where the
  service reports it, and MUST say so before transferring anything when there is not.
- **FR-045**: A command that needs a file that is remote only MUST say that it must be
  fetched first and offer to fetch it.
- **FR-046**: Users MUST be able to free local space by removing files that have an
  identical remote copy, after confirmation; no other file may be removed.
- **FR-047**: Sending the files of a dataset flagged as personal or sensitive, or of a
  record marked private, MUST warn, require explicit confirmation, and record the
  acknowledgement.

#### Remote backup

- **FR-048**: Users MUST be able to create a backup of a workspace directly at a connection
  that offers a backup destination; the backup MUST be verified before sending and again
  after arrival.
- **FR-049**: Users MUST be able to list, verify, restore from, and delete remote backups;
  restoring into an empty location MUST yield a workspace identical to the one backed up.
- **FR-050**: Users MUST be able to set how many remote backups to keep; older ones MUST be
  removed only after a new backup has succeeded and been verified.
- **FR-051**: Remote backups MUST be independent of workspace sync and restorable when the
  synced remote copy no longer exists.
- **FR-052**: A backup MUST include the files records refer to only when the user asks.

#### Visibility and control

- **FR-053**: Every command that sends or receives MUST offer a preview that lists what
  would be transferred and transfers nothing.
- **FR-054**: The system MUST keep a transfer log recording, for every transfer, the time,
  connection, machine, direction, kind of content, amount, and outcome; users MUST be able
  to filter and export it.
- **FR-055**: Users MUST be able to exclude kinds of record, records with given tags, and
  individual records from leaving the machine; sync, file storage, and remote backup MUST
  honour the exclusions and say what was left out, and the system MUST say which links will
  be incomplete on other machines.
- **FR-056**: Users MUST be able to protect everything sent through a connection with a
  passphrase, so that content and the names of records and files are unreadable at the
  service; a machine obtaining the workspace MUST ask for the passphrase and keep it in its
  protected store; the system MUST state plainly that content cannot be recovered without
  the passphrase.
- **FR-057**: Users MUST be able to see what is stored at a service for a connection, by
  kind and size, and to delete all of it after confirmation.
- **FR-058**: When a service asks the system to slow down or fails temporarily, the system
  MUST wait and retry a limited number of times, say what it is doing, and stop with a clear
  message when it cannot continue.
- **FR-059**: Telemetry MUST remain on the machine; no integration may cause it to be sent.

#### Common behavior

- **FR-060**: Every value a user supplies — integration and connection names, account
  choices, folder names, selections of records and files, numbers of backups, passphrases —
  MUST be validated before anything is stored, sent, or received; invalid input MUST change
  nothing and MUST be reported per value, all together, with what is expected.
- **FR-061**: Everything received from a service MUST be validated before it is used, as
  any other input is; content that is not what the system wrote, or that fails its checks,
  MUST be refused.
- **FR-062**: Connecting, disconnecting, publishing, obtaining, unlinking, resolving a
  conflict, and deleting remote content MUST be recorded in the audit trail; individual
  transfers MUST be recorded in the transfer log.
- **FR-063**: Every result MUST be available in a form meant for people and, on request, in
  a structured form meant for other programs.
- **FR-064**: Every command MUST have built-in help, and integrations, Google Drive sign-in,
  workspace sync, file storage, remote backup, and privacy controls MUST each have a usage
  guide with examples, including a plain statement of what is and is not sent.

### Key Entities *(include if feature involves data)*

- **Integration**: An outside service TRCLI can work with, described by the capabilities it
  offers and the access it needs. Google Drive is the first.
- **Capability**: Something an integration can do for a workspace: workspace sync, file
  storage, backup destination.
- **Connection**: A named, authorized link between this machine's user and one account at an
  integration, with the access granted and the capabilities it may be used for.
- **Credentials**: What proves to the service that a connection was approved. Kept in the
  machine's protected store; never part of a workspace.
- **Remote Copy**: The published form of a workspace at a connection, which linked machines
  send to and receive from.
- **Machine**: One computer linked to a remote copy, with a name the researcher gives it.
- **Change**: One thing that happened to one record, exchanged between machines.
- **Conflict**: Two machines' incompatible changes to the same thing, awaiting a choice.
- **Remote File**: The stored content of a file a record refers to, with its fingerprint and
  the versions of it still needed.
- **Remote Backup**: A backup of a workspace kept at a connection.
- **Transfer**: One sending or receiving, recorded in the transfer log.
- **Exclusion**: A rule keeping kinds of record, tagged records, or particular records on
  the machine.
- **Passphrase**: A secret chosen by the researcher that makes what is sent unreadable at
  the service.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: A user can connect their Google Drive in under 2 minutes, on a machine with a
  browser and on one without.
- **SC-002**: In 100% of sign-ins, the user's password is entered only on the service's own
  page, and the tool obtains no access to files it did not create.
- **SC-003**: A user can publish a workspace of 10,000 references and 1,000 runs in under 5
  minutes on an ordinary home connection, and obtain it on a second machine in under 5
  minutes.
- **SC-004**: After a day's work (about 200 changes), sending or receiving takes under 30
  seconds.
- **SC-005**: After any sequence of changes on two machines followed by sync on both, the
  two workspaces are identical, and 0 changes have been lost without the user choosing to
  discard them.
- **SC-006**: 100% of conflicting changes are presented to the user; 0 are resolved
  silently.
- **SC-007**: After an interruption at any point of any transfer, the local workspace and
  the remote copy are both valid and the transfer can be repeated, in 100% of cases.
- **SC-008**: 100% of fetched files match their recorded fingerprint or are refused.
- **SC-009**: A workspace restored from a remote backup is identical to the one backed up in
  100% of cases.
- **SC-010**: With no connection configured, the tool sends nothing to any service, in 100%
  of commands.
- **SC-011**: Credentials and passphrases appear in 0 logs, exports, backups, synced
  content, or screens.
- **SC-012**: For any period, a user can list everything that was sent and received, to and
  from where, in under 10 seconds.
- **SC-013**: With passphrase protection on, nothing stored at the service — content or
  names — can be read without the passphrase.
- **SC-014**: 100% of excluded records and their files are absent from everything sent.
- **SC-015**: Every command that does not need a service works with no network connection.
- **SC-016**: 90% of first-time users connect, publish, and obtain a workspace on a second
  machine on their first attempt using only the usage guide.

## Assumptions

- **Sync is for one researcher's own machines.** Everyone who syncs a workspace signs in to
  the same account and has full access to it. Sharing a workspace between several people
  with different roles is collaboration, specified in `specs/002-research-lifecycle` (its
  User Story 17), which builds on the sync defined here and adds members and permissions.
- **Sync is deliberate by default.** The researcher sends and receives when they choose;
  automatic sync is an option. It is not live, simultaneous editing.
- **"Sync directly" means through the service, not through a folder on disk.** The tool
  talks to Google Drive itself. Placing a workspace inside a folder that a desktop sync
  program mirrors remains unsupported, because such programs can damage a workspace's
  working storage; this specification is the supported alternative.
- **Google Drive access is limited to what the tool creates.** This is the narrowest access
  Google offers for this purpose; it means TRCLI cannot browse or import the researcher's
  existing Drive files, which is accepted. A researcher who wants a file in the workspace
  adds it locally and sends it.
- **Signing in depends on Google.** The approval pages, their wording, the limits Google
  places on applications, and institutional policies are Google's and the institution's.
  Who registers TRCLI with Google, and whether researchers may use their own registration,
  is decided at planning time; the findings are in `notes/google-sign-in.md`.
- **This specification changes an earlier promise.** `specs/001-research-workspace` states
  that nothing leaves the machine except the identifier of an explicit lookup. With this
  specification, content also leaves the machine when, and only when, the researcher has
  connected a service and runs a command that uses it. Telemetry still never leaves.
- **It builds on other specifications**: backup and restore, and the upgrade of a
  workspace's format, on `specs/002-research-lifecycle`; fingerprints and referenced files
  on `specs/001-research-workspace`; the "private" marking on `specs/006-reports`; the
  "sensitive" flag on `specs/002-research-lifecycle`; the audit trail on
  `specs/001-research-workspace`.
- **It is the "integration" the earlier specifications anticipated.** The connections to
  reference managers, writing tools, and notebooks in `specs/002-research-lifecycle` (its
  User Story 16) exchange files on the machine and need no connection; they are not
  integrations in the sense of this specification, but future service-based versions of
  them would be.
- **The protected store for secrets is the one the operating system provides** on each of
  the supported platforms.
- **Passphrase protection is optional and off by default**, because a forgotten passphrase
  makes the remote content unrecoverable. Local workspaces are not protected by it.
- **Space, speed, and availability are the service's.** The tool reports limits it is told
  about and does not work around them.
- **Sharing files with other people through Drive's own sharing** is done by the researcher
  in Drive; the tool neither sets nor changes sharing on what it creates.
