### User Story 6 - Explore uncertain inputs (Priority: P6)

A researcher asks which inputs matter. In a **sensitivity analysis** they vary one input at
a time across its range, holding the others at the baseline, and see how much each moves an
output, ranked from most to least influential. In a **sampled ensemble** they state how
uncertain each input is, let the tool draw many combinations, run them all, and see the
resulting spread of an output.

**Why this priority**: These are what a simulation study does after its main comparison:
they say how robust the conclusion is. They reuse scenarios, replications, and summaries,
and can come last.

**Independent Test**: Run a sensitivity analysis over three inputs at five levels each and
obtain the ranking for an output; then define an ensemble of fifty samples over two
uncertain inputs and obtain the distribution of the output.

**Acceptance Scenarios**:

1. **Given** a baseline scenario, **When** the researcher starts a sensitivity analysis
   naming the inputs to vary, the number of levels, and the replications per level,
   **Then** the tool shows how many runs this will take, asks for confirmation, and creates
   the scenarios and replications, grouped as one analysis.
2. **Given** a sensitivity analysis, **When** levels are chosen, **Then** each input is
   varied across the range the model states, or a range the researcher gives within it.
3. **Given** a finished sensitivity analysis, **When** the researcher asks for its outcome
   for an output, **Then** each input is shown with the output at its lowest and highest
   level, the swing between them, and its rank by swing.
4. **Given** a sensitivity analysis, **When** an input's swing is smaller than the interval
   of the baseline, **Then** it is marked as indistinguishable from noise.
5. **Given** a model, **When** the researcher states the uncertainty of some inputs — a
   range with every value equally likely, a most likely value with a spread, or a list of
   values with weights — **Then** these are stored for an ensemble.
6. **Given** stated uncertainties, **When** the researcher starts an ensemble with a number
   of samples, **Then** the tool draws that many combinations using a recorded seed, shows
   the number of runs, asks for confirmation, and runs them, grouped as one ensemble.
7. **Given** a finished ensemble, **When** the researcher asks for its outcome for an
   output, **Then** the mean, spread, chosen percentiles, minimum, and maximum of the output
   across samples are shown.
8. **Given** an ensemble or a sensitivity analysis that is interrupted, **When** it is
   resumed, **Then** finished runs are kept and only the rest are run.
9. **Given** an ensemble, **When** it is repeated with the same seed for drawing samples,
   **Then** the same combinations are drawn.
10. **Given** an outcome of either kind, **When** the researcher records it as a result or
    exports it, **Then** it is tied to the analysis and all its runs.
11. **Given** a number of levels below two, a number of samples below one, an uncertainty
    whose range falls outside the model's range, or weights that are negative, **When** the
    researcher saves, **Then** the tool rejects it.
12. **Given** an analysis that would take more runs than a stated limit, **When** it is
    started, **Then** explicit confirmation is always required.

---
