### User Story 15 - Publish outputs and record their identifiers (Priority: P15)

A researcher prepares a dataset, a piece of software, or a reproducibility package for
deposit in a public archive, checks it is ready, records the persistent identifier it
receives, and obtains the citation others should use for it.

**Why this priority**: Sharing outputs is increasingly required, but it is the last step in
the life of records that must first exist and be complete.

**Independent Test**: Run a readiness check on a dataset, fix the reported gaps, produce the
deposit bundle, record the identifier received, and export the dataset's citation.

**Acceptance Scenarios**:

1. **Given** an output, **When** the researcher requests a readiness check, **Then** the tool
   lists what is missing for deposit (for example a license, a description, or creators).
2. **Given** an output that passes the check, **When** the researcher requests a deposit
   bundle, **Then** a bundle with the output's description in a widely accepted form is
   produced for upload to an archive.
3. **Given** a deposited output, **When** the researcher records its persistent identifier,
   the archive, and the date, **Then** the output is shown as published.
4. **Given** a published output, **When** the researcher requests its citation, **Then** a
   citation in a chosen style is produced and can be linked to drafts.
5. **Given** an output flagged as sensitive, **When** a deposit bundle is requested, **Then**
   the tool refuses unless the researcher explicitly confirms.

---
