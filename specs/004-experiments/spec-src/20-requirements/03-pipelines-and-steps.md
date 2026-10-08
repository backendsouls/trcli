#### Pipelines and steps

- **FR-012**: Users MUST be able to define a pipeline for an experiment as steps, each with
  a key unique within the pipeline, a name, the steps it depends on, and a kind: automated
  or manual.
- **FR-013**: An automated step MUST state what the machine carries out; a manual step MUST
  carry instructions for the person and MAY name the person who performs it.
- **FR-014**: Users MUST be able to declare the files a step is expected to produce.
- **FR-015**: The system MUST reject a pipeline whose dependencies form a loop or refer to a
  step that does not exist, naming the steps involved, and MUST refuse to remove a step
  that other steps depend on.
- **FR-016**: The system MUST show a pipeline's steps in the order they will be carried out,
  and users MUST be able to copy a pipeline from another experiment.
