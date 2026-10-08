### User Story 4 - Gather things under topics (Priority: P4)

A researcher keeps a list of the topics they care about — the areas their ideas, questions,
and reading keep returning to. They place ideas, questions, references, and any other
record under one or more topics, arrange topics under broader ones, and open a topic to see
everything they have on it in one place.

**Why this priority**: Ideas are remembered by what they are about. Topics are how a
researcher finds the idea from last spring when the subject comes up again, and how they
see that a topic has many ideas and no question yet.

**Independent Test**: Create a topic with a narrower topic beneath it, place two ideas, a
question, and three references under them, open the broader topic, and confirm everything
under both is shown, grouped by kind.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a topic with a name and a
   description, **Then** it is stored.
2. **Given** a topic, **When** the researcher places it under a broader topic, **Then** it
   is shown beneath it, and the broader topic's page includes what the narrower one holds.
3. **Given** a topic, **When** the researcher places ideas, questions, references, or any
   other record under it, **Then** the topic lists them and each record lists its topics.
4. **Given** a record, **When** it is placed under several topics, **Then** it appears under
   each and exists once.
5. **Given** a topic, **When** the researcher opens it, **Then** everything under it and
   under its narrower topics is shown, grouped by kind, with counts, and with the most
   recent activity first.
6. **Given** topics, **When** the researcher lists them, **Then** each is shown with the
   number of ideas, open questions, and references under it, and when something was last
   added.
7. **Given** a topic, **When** the researcher marks it as one they are actively pursuing,
   watching, or have set aside, **Then** topics can be listed by that state.
8. **Given** topics, **When** the researcher asks for those with ideas but no research
   question, **Then** they are listed — candidates for a question worth asking.
9. **Given** two topics that turn out to be the same, **When** the researcher merges them,
   **Then** one remains holding everything of both.
10. **Given** a topic with things under it, **When** the researcher deletes it, **Then** the
    tool lists them and requires confirmation; the records themselves are kept.
11. **Given** a topic with the name of an existing one, a topic placed beneath itself, or a
    missing name, **When** the researcher saves, **Then** the tool rejects it.
12. **Given** a tag already used on many records, **When** the researcher turns it into a
    topic, **Then** a topic is created holding those records.

---
