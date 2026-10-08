### User Story 10 - Manage funding and grants (Priority: P10)

A researcher records funders and grants through their life (idea, in preparation,
submitted, awarded, rejected, active, closed), with amounts, periods, budget lines, and
reporting deadlines, and links each grant to the outputs it paid for, so that a funder
report and an acknowledgement text can be produced.

**Why this priority**: Funding sustains the work and carries reporting duties, but nothing
else in the workspace depends on it.

**Independent Test**: Record a grant with two budget lines and a reporting deadline, link a
draft and a dataset to it, and produce the list of outputs for a reporting period.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a grant with a title, a funder, an
   amount with its currency, and a period, **Then** it is stored at the stage "idea" unless
   another is given.
2. **Given** a grant, **When** the researcher adds budget lines and records spending against
   them, **Then** the grant shows planned, spent, and remaining amounts per line.
3. **Given** a grant, **When** the researcher adds reporting deadlines, **Then** they appear
   in the "what's due" view.
4. **Given** a grant, **When** the researcher links drafts, datasets, experiments, and staff
   to it, **Then** each shows the grant that supports it.
5. **Given** a grant and a period, **When** the researcher requests an output report,
   **Then** every linked output with activity in that period is listed.
6. **Given** a draft supported by grants, **When** the researcher requests its
   acknowledgement, **Then** a text naming each funder and grant reference is produced.
7. **Given** a negative amount, an unknown currency, or an end date before the start date,
   **When** the researcher saves the grant, **Then** the tool rejects it.

---
