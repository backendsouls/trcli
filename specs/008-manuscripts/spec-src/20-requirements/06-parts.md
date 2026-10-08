#### Parts

- **FR-037**: Users MUST be able to give a manuscript an ordered outline of parts, nested to
  any depth, each with a title, a status (not started, outlined, drafting, drafted, revised,
  final), a target length, a deadline, the person writing it, and the location of its file.
- **FR-038**: Users MUST be able to add, rename, move, and remove parts; removing a part
  that has parts beneath it MUST ask whether to remove those or keep them one level up.
- **FR-039**: Users MUST be able to make another manuscript a part of a manuscript; such a
  part's status MUST follow that manuscript's stage, and the system MUST reject a
  manuscript made a part of itself directly or indirectly.
- **FR-040**: The system MUST show a manuscript's progress: parts at each status, current
  length against target for each part and overall, and parts that are late; lengths MUST be
  counted from files that can be read as text and MUST otherwise be enterable by hand.
- **FR-041**: When a document is created for a manuscript from a template, the template's
  sections MUST become the manuscript's parts unless it already has parts.
