### User Story 14 - Track instruments, samples, and materials (Priority: P14)

A researcher doing laboratory or field work records instruments with their calibration
history, samples with their origin and where they are stored, and materials with their lot
and expiry, and records which of them each run used.

**Why this priority**: Essential for bench and field research but irrelevant to much
computational work; it extends runs that must already exist.

**Independent Test**: Register an instrument with a calibration due date, a sample, and a
material lot, record their use in a run, and view the run listing all three.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher registers an instrument with its
   identifier and calibration dates, **Then** it is stored and its next calibration appears
   in the "what's due" view.
2. **Given** a workspace, **When** the researcher registers a sample with its origin,
   collection date, and storage place, **Then** it is stored, and a sample derived from
   another shows its parent.
3. **Given** a workspace, **When** the researcher registers a material with its supplier,
   lot, quantity, and expiry, **Then** it is stored.
4. **Given** a run, **When** the researcher records the instruments, samples, and materials
   used, **Then** the run lists them and each lists the runs it was used in.
5. **Given** an instrument past its calibration date or a material past its expiry, **When**
   it is recorded as used in a run, **Then** the tool warns and records the acknowledgement.

---
