### User Story 5 - Connect a manuscript to the research behind it (Priority: P5)

A researcher ties a manuscript to what it rests on: the research questions it answers, the
references it cites, the results, figures, and tables it reports, the experiments,
datasets, and methods behind them. They can then ask of any manuscript "what does this
stand on?" and, before sending it, whether anything it stands on has changed or is missing.

**Why this priority**: These links are what make a manuscript traceable — the reason to keep
writing and research in the same workspace. They need the other specifications' records to
exist.

**Independent Test**: Link a manuscript to a research question, a bibliography, two results,
and a figure; view what it rests on; then change one result with a newer run and confirm
the readiness check reports it.

**Acceptance Scenarios**:

1. **Given** a manuscript, **When** the researcher links the research questions and
   hypotheses it addresses, **Then** each shows the manuscript and the manuscript shows
   them.
2. **Given** a manuscript, **When** the researcher links citations or a whole bibliography
   to it, **Then** the manuscript lists what it cites and can export its reference list.
3. **Given** a manuscript, **When** the researcher links the results, figures, and tables it
   reports, **Then** the manuscript lists each with the run it came from.
4. **Given** a manuscript, **When** the researcher asks what it rests on, **Then** the
   questions, citations, results, figures, tables, experiments, datasets, methodologies,
   and grants behind it are shown, grouped by kind.
5. **Given** any record, **When** the researcher asks which manuscripts use it, **Then**
   those manuscripts are listed.
6. **Given** a manuscript, **When** the researcher asks whether it is ready, **Then** the
   tool reports: reported results that a newer run has replaced, figures or tables whose
   files have changed, citations whose reference lacks required details, authors without
   affiliation, a missing abstract or corresponding author, and an approaching or passed
   deadline — or says that nothing was found.
7. **Given** a readiness check with nothing found, **When** it ends, **Then** its outcome
   can be used by another program to allow a next step.
8. **Given** a manuscript written in a document created from a template, **When** readiness
   is checked, **Then** the document's own check is included.
9. **Given** a result linked to a manuscript, **When** the result is deleted, **Then** the
   tool lists the manuscripts reporting it and requires confirmation.
10. **Given** a record that does not exist, **When** the researcher links it, **Then** the
    tool rejects the link.

---
