### User Story 9 - Manage ethics and compliance (Priority: P9)

A researcher records ethics approvals (the body, the reference, the dates of validity, the
conditions), the consent basis under which data was collected, and the project's
data-management plan. Datasets are flagged when they hold personal or sensitive data, and
the tool warns when work touches such data without a valid approval.

**Why this priority**: These are obligations with consequences. They attach to datasets and
experiments that must already exist, and matter most once a project involves people.

**Independent Test**: Record an approval with an expiry date, flag a dataset as personal
data, link them, and see a warning when an experiment uses the dataset after the approval
has expired.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records an ethics approval with its body,
   reference, validity dates, and conditions, **Then** it is stored with the status derived
   from its dates (pending, valid, expiring soon, expired).
2. **Given** a dataset, **When** the researcher flags it as holding personal or sensitive
   data with a sensitivity level, **Then** the flag is shown wherever the dataset appears.
3. **Given** a flagged dataset, **When** the researcher records its consent basis and links
   an approval, **Then** both are shown with the dataset.
4. **Given** a flagged dataset with no valid approval, **When** an experiment using it is
   run, **Then** the tool warns and requires explicit acknowledgement, which is recorded.
5. **Given** approvals nearing expiry, **When** the researcher asks what is due, **Then**
   the expiring approvals are listed.
6. **Given** a workspace, **When** the researcher records a data-management plan with its
   sections and links datasets to it, **Then** the plan lists the datasets it covers and
   the datasets not covered by any plan can be listed.
7. **Given** a flagged dataset, **When** a reproducibility package or export that includes
   it is produced, **Then** the tool warns that sensitive data is referenced.

---
