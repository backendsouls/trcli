### User Story 4 - Learn from experience with each venue (Priority: P4)

Each time a manuscript goes to a venue, something is learned: how long the decision took,
what the reviews were like, whether the process was fair, whether the editors answered. A
researcher sees, for each venue, their own history there — what was sent, what happened,
how long it took — and writes down their impressions, so that the next choice is made from
experience rather than memory.

**Why this priority**: A researcher's own record is the most reliable information they have
about a venue, and it accumulates only if kept. It needs submissions to have happened, so
it follows the choice of venue.

**Independent Test**: With two past submissions to a venue, one accepted and one rejected,
view the venue's history with the time to decision of each, add an impression with a
rating, and list venues ordered by the researcher's own acceptance record.

**Acceptance Scenarios**:

1. **Given** a venue with past submissions, **When** the researcher views its history,
   **Then** each submission is listed with the manuscript, the dates, the decision, the
   number of review rounds, and the time from submission to first decision.
2. **Given** a venue's history, **When** it is summarized, **Then** the number of
   submissions, acceptances, and rejections, and the shortest, typical, and longest time to
   decision are shown, computed from the researcher's own submissions only.
3. **Given** a venue, **When** the researcher adds an impression — a dated note with an
   optional rating of the quality of reviews, the speed, and the handling — **Then** it is
   stored with the venue, private by default.
4. **Given** venues with history, **When** the researcher lists them ordered by their own
   time to decision or acceptance record, **Then** the order reflects their experience.
5. **Given** a submission still awaiting a decision, **When** the researcher looks at it,
   **Then** the time waited is shown beside the researcher's typical time at that venue,
   and it is marked when it has waited longer than their longest.
6. **Given** a venue with no submissions by the researcher, **When** its history is asked
   for, **Then** the tool says there is none and shows what the researcher recorded about
   its stated times.
7. **Given** a venue, **When** the researcher records what the venue itself states — its
   acceptance rate, its typical time to decision — with the date noted, **Then** these are
   shown separately from the researcher's own figures and never mixed with them.
8. **Given** impressions, **When** venues are exported for a colleague, **Then** impressions
   are left out unless the researcher explicitly includes them.
9. **Given** a rating outside its scale, **When** the researcher saves, **Then** the tool
   rejects it.

---
