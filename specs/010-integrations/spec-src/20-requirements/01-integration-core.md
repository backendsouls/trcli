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
