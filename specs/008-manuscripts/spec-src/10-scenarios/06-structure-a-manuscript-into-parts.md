### User Story 6 - Structure a manuscript into parts (Priority: P6)

A researcher breaks a long manuscript into parts — the chapters of a thesis, the sections
of a paper — each with a title, a status, a target length, and, if useful, its own deadline
and the person writing it. They see how much of the whole is done. A thesis made of
published articles is assembled by making other manuscripts its parts.

**Why this priority**: A thesis or a long report is not written as one thing, and "how far
along is it?" is answered part by part. Shorter manuscripts do not need this, so it comes
after the stories everyone uses.

**Independent Test**: Give a thesis five chapters with target lengths, set their statuses,
make one chapter an existing paper, and view the progress of the whole.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher adds parts in order, each with a title,
   **Then** the manuscript shows its outline.
2. **Given** a part, **When** the researcher adds parts beneath it, **Then** the outline
   shows them nested, numbered as they will appear.
3. **Given** a part, **When** the researcher sets its status (not started, outlined,
   drafting, drafted, revised, final), target length, deadline, the person writing it, and
   the location of its file, **Then** these are saved.
4. **Given** parts with statuses and targets, **When** the researcher asks for the
   manuscript's progress, **Then** the number of parts at each status, the current length
   against the target for each part and overall, and the parts that are late are shown.
5. **Given** parts whose files can be read as text, **When** progress is shown, **Then**
   current lengths are counted from the files; otherwise the researcher can enter them.
6. **Given** a manuscript and another manuscript, **When** the researcher makes the second a
   part of the first, **Then** the outline shows it with its own stage, and the part's
   status follows that stage.
7. **Given** a part, **When** the researcher moves it to another position or under another
   part, **Then** the outline and the numbering follow.
8. **Given** a manuscript made a part of itself, directly or through other manuscripts,
   **When** the researcher saves, **Then** the tool rejects it.
9. **Given** a part with parts beneath it, **When** the researcher removes it, **Then** the
   tool lists what is beneath and asks whether to remove or keep those one level up.
10. **Given** a manuscript created from a template, **When** its document is created,
    **Then** the template's sections become the manuscript's parts unless parts already
    exist.
11. **Given** a target length that is zero or negative, or a part without a title, **When**
    the researcher saves, **Then** the tool rejects it.

---
