### User Story 7 - Manage methodologies and datasets (Priority: P7)

> **Moved in part (2026-10-08)**: methodologies are now specified in
> `specs/014-conventions-methods`, with their steps, origin, versions, and adaptations.
> Datasets remain specified here; scenarios 1 and 2 below are superseded by that
> specification.

A researcher records the methodologies they use (name, description, procedure, references)
and the datasets they work with (name, description, origin, license, location, version,
integrity fingerprint), and links both to the experiments that use them.

**Why this priority**: Methods and data explain how a result was obtained. They are
reusable across experiments and are required before reproducibility can be assessed.

**Independent Test**: Create a methodology and a dataset, link both to an experiment, and
view the experiment showing which method and which dataset version it uses.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher creates a methodology with a name and a
   description, **Then** it is stored and can be listed, viewed, changed, and removed.
2. **Given** a methodology, **When** the researcher links papers that describe it, **Then**
   the methodology shows those references.
3. **Given** a workspace, **When** the researcher registers a dataset with a name and a
   location, **Then** it is stored with a version and an integrity fingerprint.
4. **Given** a registered dataset whose content has changed, **When** the researcher asks
   the tool to verify it, **Then** the tool reports that it no longer matches and offers to
   record a new version.
5. **Given** an experiment, **When** the researcher links a methodology and a dataset
   version, **Then** runs of that experiment record which ones were used.
6. **Given** a dataset whose location does not exist, **When** the researcher tries to
   register it, **Then** the tool rejects it with a clear message.

---
