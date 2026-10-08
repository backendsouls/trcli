#### Results, figures, and tables

- **FR-038**: Users MUST be able to record a result from a run with a name, a value, a unit,
  an optional uncertainty, and a description, and to create a result from a recorded
  metric; a result's name MUST be unique within its run.
- **FR-039**: Users MUST be able to record a figure or a table from a run with a title, a
  caption, the location of its file, and an integrity fingerprint, without the tool taking
  a copy of the file.
- **FR-040**: Every result, figure, and table MUST be permanently tied to the run that
  produced it, and the system MUST show for each one the run, its parameters and condition,
  the pipeline as run, the dataset versions, the methodology, the software versions, and
  the environment.
- **FR-041**: Users MUST be able to link results to hypotheses with a stance (supports,
  contradicts, neutral), and link results, figures, and tables to drafts.
- **FR-042**: The system MUST flag a draft when a result it reports has been followed by a
  newer result of the same name from the same experiment, and let the user keep the
  reported one or switch.
- **FR-043**: Users MUST be able to verify a figure or table file against its recorded
  fingerprint.
- **FR-044**: The system MUST refuse to delete a run while any result, figure, or table
  depends on it, and MUST require confirmation to record a finding from a run that did not
  succeed.
