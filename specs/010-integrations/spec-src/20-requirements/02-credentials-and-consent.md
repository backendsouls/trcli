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
