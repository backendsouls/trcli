#### Learning roadmaps

- **FR-050**: Users MUST be able to create, list, view, update, and delete learning
  roadmaps, each with a title, a goal, a status (active, paused, completed, abandoned), and
  ordered stages with a title and an optional target date.
- **FR-051**: Users MUST be able to add items to a stage, of the kinds course, reference,
  skill, task, and free item with an optional web address; each item has a status (to do,
  in progress, done, skipped), an optional estimate of effort, and notes.
- **FR-052**: An item linked to a course, a reference, or a task MUST take its status from
  that record; when the record is deleted the item MUST remain as a free item.
- **FR-053**: Users MUST be able to make items depend on other items; a dependent item MUST
  be shown as blocked until the others are done or skipped, and the system MUST reject
  dependencies that form a loop.
- **FR-054**: The system MUST show, for a roadmap, the items done per stage, overall
  progress, the items that can be worked on next, and stages past their target date.
- **FR-055**: Users MUST be able to record, for an item, the time it took and what was
  learned, and for a skill item their level before and after (none, basic, working,
  proficient).
- **FR-056**: Users MUST be able to export a roadmap's stages, items, and dependencies
  without personal progress, and bring such a file in as a new roadmap that is not started;
  the system MUST list references the file names that are not in the library and offer to
  add them.
- **FR-057**: When every item of a roadmap is done or skipped, the system MUST offer to mark
  it completed with a closing note.
