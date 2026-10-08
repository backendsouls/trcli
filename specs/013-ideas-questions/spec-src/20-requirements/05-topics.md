#### Topics

- **FR-035**: Users MUST be able to create, list, view, update, and delete topics, each with
  a name unique in the workspace ignoring case and accents, a description, and a state:
  pursuing, watching, or set aside.
- **FR-036**: Users MUST be able to place a topic under one or more broader topics; the
  system MUST reject a topic placed beneath itself directly or indirectly.
- **FR-037**: Users MUST be able to place any record under one or more topics; a record
  under several topics MUST exist once, and each record MUST show its topics.
- **FR-038**: Opening a topic MUST show everything under it and under its narrower topics,
  grouped by kind, with counts and the most recent activity first.
- **FR-039**: Listing topics MUST show, for each, the number of ideas, open questions, and
  references under it and when something was last added, and MUST be filterable by state.
- **FR-040**: Users MUST be able to list topics that hold ideas but no research question.
- **FR-041**: Users MUST be able to merge two topics into one that holds everything of both,
  and to turn a tag into a topic holding the records that carry the tag.
- **FR-042**: Deleting a topic MUST list what is under it, require confirmation, and keep
  the records themselves.
