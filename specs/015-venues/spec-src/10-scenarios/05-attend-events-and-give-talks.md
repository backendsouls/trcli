### User Story 5 - Attend events and give talks (Priority: P5)

A researcher records the events they plan to attend or have attended, and what they
presented there: a talk, a poster, a tutorial — its title, its date, its material, and the
manuscript or results it was based on. They keep the practical dates of attending — early
registration, travel — and afterwards have a list of everything they have presented, in the
form a CV or a funder's report asks for.

**Why this priority**: Presenting is part of publishing, and the list of talks is asked for
as often as the list of papers. It is the least urgent story because nothing else depends
on it.

**Independent Test**: Mark an event as one to attend with a registration deadline, record a
talk and a poster given there linked to a manuscript, and produce the list of presentations
for the year.

**Acceptance Scenarios**:

1. **Given** an event, **When** the researcher marks that they plan to attend, attended, or
   decided not to, **Then** events can be listed by that state.
2. **Given** an event to attend, **When** the researcher records practical dates — the end of
   early registration, a visa or travel-grant deadline — and costs with their currency,
   **Then** the dates appear in the workspace's view of what is due and the costs are shown
   with the event.
3. **Given** an event, **When** the researcher records something they presented — a talk, a
   poster, a tutorial, a demonstration, a panel — with its title, date, co-presenters, and
   the location of its material, **Then** it is stored under the event.
4. **Given** a presentation, **When** the researcher links the manuscript, results, or
   project it was based on, and the grant that paid for it, **Then** each shows the other.
5. **Given** a presentation not tied to a recorded event — an invited seminar at another
   university — **When** the researcher records it with the host and the place, **Then** it
   is stored without an event.
6. **Given** presentations, **When** the researcher asks for their list, **Then** they are
   listed newest first, grouped by year or by kind, in a chosen citation style, and can be
   filtered by period, kind, project, and grant.
7. **Given** a list of presentations, **When** the researcher exports it, **Then** a
   document is produced.
8. **Given** a presentation that was invited, or received an award, **When** the researcher
   records it, **Then** it is marked in the list.
9. **Given** an event, **When** the researcher adds notes — people met, talks worth
   following up — **Then** they are kept with the event, and a follow-up can be captured as
   a thought in the inbox.
10. **Given** a presentation dated outside its event's dates, a cost that is negative, or a
    missing title, **When** the researcher saves, **Then** the tool rejects it, warning
    rather than rejecting for the date.

---
