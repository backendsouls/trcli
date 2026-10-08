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
