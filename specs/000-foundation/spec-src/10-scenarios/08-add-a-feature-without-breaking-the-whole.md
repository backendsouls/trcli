### User Story 8 - Add a feature without breaking the whole (Priority: P8)

A contributor adds a new kind of record — or a whole new feature — to TRCLI. They declare
what is particular to it, and it arrives with everything records have in common: a short
name, the shared actions, tags, notes, links, guarded deletion, input checking, both output
forms, an entry in the record of what happened, a place among projects. Before their work
is accepted, the same checks that every part passed are applied to it, on every supported
system.

**Why this priority**: TRCLI is specified as many features delivered over time by more than
one person. The foundation must make the consistent thing the easy thing, and must catch
what is inconsistent before it reaches a researcher. It is last because the contributor is
served once the researcher-facing ground exists.

**Independent Test**: Add a trivial new kind of record following the contributor guide,
and confirm that — without writing code for them — it can be listed, tagged, noted, linked,
deleted with confirmation, shown in both output forms, and that its changes appear in the
record of what happened; then confirm the acceptance checks reject a change that lacks
tests, documentation, or a usage guide.

**Acceptance Scenarios**:

1. **Given** a new kind of record, **When** a contributor defines what is particular to it,
   **Then** it has the behaviour shared by all records without that behaviour being written
   again.
2. **Given** a new command, **When** it is added, **Then** it follows the one command
   grammar, the shared options, the output forms, and the meanings of how a command ends.
3. **Given** a feature a researcher will use, **When** it is proposed, **Then** it is
   accepted only if every acceptance scenario of its specification is an automated check
   run against the tool as a researcher would run it.
4. **Given** any input a new command accepts, **When** the feature is proposed, **Then** it
   is accepted only if there are checks that invalid forms of that input are rejected.
5. **Given** new code, **When** it is proposed, **Then** it is accepted only if every part
   of it is documented for a human reader, and the feature has its usage guide.
6. **Given** a change, **When** it is checked, **Then** the full set of checks runs on every
   supported system, and the change is accepted only if all pass on all of them.
7. **Given** the rules of the research itself — what a valid record is, what may follow
   what — **When** code is organized, **Then** those rules depend on nothing about how
   records are stored, shown, or invoked, and this separation is checked automatically.
8. **Given** a part of the tool that reaches outside it — storage, files, the clock, another
   program, a service — **When** it is used by a feature, **Then** it can be replaced by a
   stand-in for testing without changing the feature.
9. **Given** a new version that changes how workspaces are stored, **When** it is prepared,
   **Then** it includes the step that brings an existing workspace forward, and a check
   that a workspace from each earlier version is brought forward without loss.
10. **Given** two features developed separately, **When** both are present, **Then** neither
    needs to know how the other is built: they refer to each other's records only by kind
    and name.

Two further expectations of contributors are rules rather than scenarios, because no
automated check can observe them: that tests are written and seen to fail before the code
(FR-073), and that a newcomer can build, check, and add a simple command within an hour
(FR-076, SC-015).

---
