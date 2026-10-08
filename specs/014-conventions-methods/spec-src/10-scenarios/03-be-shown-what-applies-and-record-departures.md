### User Story 3 - Be shown what applies, and record departures (Priority: P3)

When a researcher is about to do something — register a dataset, start an experiment,
submit a manuscript — they ask "what applies here?" and see the conventions, methodology,
assumptions, and checklists that concern it. When they knowingly do otherwise, they record
the departure and why. Later, anyone can see, for any piece of work, which rules it
followed and where it did not.

**Why this priority**: A convention nobody sees at the right moment is not followed, and a
departure nobody recorded looks like a mistake. This is what turns a written handbook into
something that shapes the work and can be reported honestly.

**Independent Test**: With a convention applying to datasets and a methodology linked to an
experiment, ask what applies to a new dataset and to the experiment; record a departure
from the convention for one dataset with a reason; and list all departures in a project.

**Acceptance Scenarios**:

1. **Given** conventions that apply to a kind of record, **When** the researcher asks what
   applies to that kind, **Then** they are listed, the binding ones first, each with its
   origin.
2. **Given** a particular record, **When** the researcher asks what applies to it, **Then**
   the conventions for its kind, project, and topics, the methodology it follows, the
   assumptions under it, and the checklists applied to it are shown together.
3. **Given** conventions that apply to a kind of record, **When** the researcher creates a
   record of that kind, **Then** the tool mentions how many apply and how to see them,
   without slowing the command or asking anything.
4. **Given** two conventions that apply to the same thing and say different things, **When**
   they are shown, **Then** the tool marks them as conflicting and says which takes
   precedence and why.
5. **Given** a convention or a methodology step, **When** the researcher records that a
   piece of work departs from it, with the reason and what was done instead, **Then** the
   departure is stored with that work.
6. **Given** a departure from something whose origin is the institution, a venue, or a
   funder, **When** it is recorded, **Then** the tool asks who approved it and warns when
   nobody is named.
7. **Given** a piece of work, **When** the researcher views it, **Then** its departures are
   shown with it.
8. **Given** a project or the whole workspace, **When** the researcher lists departures,
   **Then** each is shown with the work, what was departed from, the reason, and the date.
9. **Given** a convention many pieces of work depart from, **When** the researcher asks which
   conventions are most often departed from, **Then** they are listed — candidates for
   changing the convention.
10. **Given** a convention, **When** the researcher marks that they have read it, **Then**
    the date is recorded, and conventions adopted or changed since they last read them can
    be listed.
11. **Given** a departure whose reason is missing, or that names something that does not
    apply to the work, **When** the researcher saves, **Then** the tool rejects it.
12. **Given** a convention retired after work departed from it, **When** the departures are
    listed, **Then** they remain, marked as concerning a retired convention.

---
