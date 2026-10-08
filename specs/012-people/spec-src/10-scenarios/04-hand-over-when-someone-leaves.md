### User Story 4 - Hand over when someone leaves (Priority: P4)

When a member leaves the group, the lab head sees everything that person holds — the
projects they are on, the experiments and datasets they are responsible for, their open
tasks, the manual steps waiting for them, the manuscripts in progress — and goes through
it: give each to someone else, close it, or leave it with the person as an outside
collaborator. The person then becomes a former member; their name stays on everything they
did.

**Why this priority**: Knowledge and responsibility leave with people. A short, complete
handover is the minimum that protects the lab's work, and it needs the earlier stories to
know what a person holds.

**Independent Test**: For a person with a project, two open tasks, a dataset, and a paused
experiment step, start the handover, reassign two items, close one, leave one with them,
complete it, and confirm they are a former member with nothing left unassigned.

**Acceptance Scenarios**:

1. **Given** a member, **When** the lab head starts their handover, **Then** everything they
   currently hold is listed by kind: projects, experiments, manual steps waiting for them,
   open tasks, datasets, manuscripts in progress, and people they supervise.
2. **Given** an item in the handover, **When** the lab head gives it to another person,
   **Then** the assignment moves, and the item records from whom it came and when.
3. **Given** an item, **When** the lab head closes it or leaves it with the departing
   person, **Then** the choice is recorded.
4. **Given** a handover, **When** the lab head adds notes for an item — where things are,
   what the next step was — **Then** the notes are kept with the item for whoever takes it.
5. **Given** a handover with items not yet decided, **When** the lab head tries to complete
   it, **Then** the tool lists them and asks for confirmation.
6. **Given** a completed handover, **When** it is confirmed, **Then** the person becomes a
   former member with their leaving date, or an outside collaborator if they keep working
   with the group.
7. **Given** a former member, **When** past records are viewed — runs they performed,
   manuscripts they authored, audit entries, meetings — **Then** their name remains.
8. **Given** a former member, **When** the lab head lists the group, **Then** they are not
   shown unless former members are asked for.
9. **Given** a handover, **When** the lab head exports it, **Then** a document listing what
   was handed to whom, with the notes, is produced.
10. **Given** a person who returns, **When** the lab head records a new period in the
    group, **Then** they are a current member again and their earlier period is kept.
11. **Given** a person, **When** the lab head asks to remove them from the register
    entirely, **Then** the tool explains that their name will remain where history needs
    it, removes their contact details and private notes, and requires confirmation.

---
