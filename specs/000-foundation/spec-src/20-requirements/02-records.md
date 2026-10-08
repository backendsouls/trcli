#### Records

- **FR-010**: Every record of every kind MUST have a short name that identifies its kind, is
  unique within the workspace, never changes, and is never given to another record.
- **FR-011**: Wherever a record is named, users MUST be able to give any beginning of its
  short name that matches exactly one record of a kind the command accepts, in any letter
  case; an ambiguous beginning MUST list the matches and change nothing; no match MUST say
  so and suggest close ones.
- **FR-012**: For every kind of record, users MUST be able to create, list, view, update,
  and delete records, unless the specification that owns the kind says a record cannot be
  changed or removed — in which case the system MUST say what to do instead.
- **FR-013**: These actions MUST be invoked in the same way for every kind: the same words,
  the same order, the same options for filtering, sorting, searching, and limiting.
- **FR-014**: Updating a record MUST change only what the user named.
- **FR-015**: Users MUST be able to attach tags and dated notes to any record, find records
  of any kind by tag, and see every tag in use with how many records carry it.
- **FR-016**: Users MUST be able to link any two records, optionally stating how they
  relate; a link MUST be visible from both records; a record MUST NOT be linked to itself
  and the same link MUST NOT exist twice.
- **FR-017**: Before deleting a record, the system MUST list the records that refer to it and
  MUST require explicit confirmation. After a deletion no link, tag, or note MUST point to
  the deleted record.
- **FR-018**: The system MUST record, for every record, when it was created and when it was
  last changed.
- **FR-019**: A record of one feature MUST be able to refer to a record of any other feature
  by its kind and identity alone.
