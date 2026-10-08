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
