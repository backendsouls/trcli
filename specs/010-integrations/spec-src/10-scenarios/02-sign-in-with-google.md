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
