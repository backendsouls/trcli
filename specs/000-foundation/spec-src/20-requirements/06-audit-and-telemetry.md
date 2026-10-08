#### Audit trail and local telemetry

- **FR-046**: The system MUST record an audit entry for every creation, change, and
  deletion of any record, and for every other action a specification names (imports,
  exports, runs, confirmations), stating the action, the record with its name at the time,
  the actor, the moment, and what changed.
- **FR-047**: A change and its audit entry MUST be made together or not at all.
- **FR-048**: Audit entries MUST NOT be editable or removable through the tool, and the
  system MUST be able to detect that the trail has been altered or shortened outside the
  tool, reporting where the first problem is.
- **FR-049**: Users MUST be able to query the audit trail by record, kind of record, actor,
  action, and range of dates, newest first, and to export the result as a report.
- **FR-050**: The actor MUST be the name the user set for themselves, or otherwise the name
  they are known by on their machine; when neither can be determined a stated placeholder
  MUST be recorded.
- **FR-051**: The order of the audit trail MUST NOT depend on the machine's clock alone.
- **FR-052**: The system MUST record local telemetry about its own use — which commands were
  used, how long they took, whether they succeeded — MUST let users view it, and MUST let
  users turn it off, after which none is recorded while the audit trail continues.
- **FR-053**: Telemetry MUST stay inside the workspace and MUST never be transmitted.
- **FR-054**: Secrets MUST never appear in the audit trail, in telemetry, in messages, or
  in any output.
