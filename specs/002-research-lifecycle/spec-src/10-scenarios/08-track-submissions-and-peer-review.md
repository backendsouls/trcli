### User Story 8 - Track submissions and peer review (Priority: P8)

A researcher records each time a draft is submitted to a venue: which version was sent,
when, and the decision. For each round they record the reviewers' comments one by one,
write a response to each, track which have been addressed, and produce the response letter.
A rejection followed by submission elsewhere stays part of the same draft's history.

**Why this priority**: A draft's stage says where it is; it does not hold the venues tried,
what reviewers asked, or what was promised in reply — the content of months of work.

**Independent Test**: Submit a draft version to a venue, record a "major revision" decision
with four reviewer comments, respond to each, and generate the response letter.

**Acceptance Scenarios**:

1. **Given** a draft with a version, **When** the researcher records a submission to a venue
   with a date, **Then** it is stored and the draft's stage becomes "submitted".
2. **Given** a submission, **When** the researcher records the decision (accepted, minor
   revision, major revision, rejected, withdrawn) with its date, **Then** it is stored and
   the draft's stage is updated to match.
3. **Given** a decision, **When** the researcher records reviewer comments, each attributed
   to a reviewer label, **Then** each is stored with the status "open".
4. **Given** a comment, **When** the researcher writes a response and links the draft
   version that addresses it, **Then** the comment is shown as addressed.
5. **Given** a review round, **When** the researcher requests the response letter, **Then**
   a document lists every comment with its response, and unanswered comments are flagged.
6. **Given** a rejected submission, **When** the researcher submits the draft to another
   venue, **Then** both submissions appear in the draft's history in order.
7. **Given** a decision dated before its submission, **When** the researcher saves it,
   **Then** the tool rejects it.

---
