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
