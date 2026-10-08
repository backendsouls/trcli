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
