# Feature Specification: TRCLI Experiments

**Feature Branch**: `004-experiments`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add concepts of experiment as a separated spec on its own"

## Overview

An experiment is how a researcher turns a hypothesis into evidence. This specification
gathers everything TRCLI knows about experiments into one place: what an experiment is and
how it is designed, the pipeline of steps that carries it out, each run of that pipeline,
the parameters and measurements of a run, the results, figures, and tables a run produces,
the code that ran, and how an experiment is concluded and replicated.

It **takes over** the experiment content previously spread across two specifications, which
now point here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 5 | Experiments, pipelines, manual steps, runs |
| `specs/001-research-workspace`, User Story 6 | Results, figures, and tables |
| `specs/002-research-lifecycle`, User Story 5 | Run parameters, metrics, sweeps, comparison |
| `specs/002-research-lifecycle`, User Story 6 | Code and software versions behind a run |

It **adds** two things that were not specified before: the design of an experiment
(its kind, variables, and conditions), and concluding and replicating an experiment.

### The concepts, in one picture

```text
Research Question ── has ──▶ Hypothesis ◀── tests ── Experiment ── has ──▶ Design
                                 ▲                       │                 (kind, variables,
                                 │ evidence              │ has              conditions)
                                 │ (supports /           ▼
                                 │  contradicts)      Pipeline ── ordered ──▶ Step
                                 │                       │                    (automated | manual)
                              Result ◀── produces ──    Run ── one per ──▶ Step Result
                              Figure                     │
                              Table                      ├── Parameters, Metrics
                                                         ├── Condition, Replicate
                                                         └── Methodology, Dataset versions,
                                                             Software versions, Environment
```

Methodologies, datasets, environment snapshots, and reproducibility checks remain specified
in `specs/001-research-workspace`; research questions and hypotheses likewise. This
specification refers to them and does not redefine them.
