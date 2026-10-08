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
