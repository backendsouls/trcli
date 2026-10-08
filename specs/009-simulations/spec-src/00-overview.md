# Feature Specification: TRCLI Simulations

**Feature Branch**: `009-simulations`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add simulation concept as part of experiment but different spec"

## Overview

Some experiments are not carried out on the world but on a **model** of it: a queue, an
epidemic, a market, a material, a network. The researcher builds a model, feeds it
**scenarios**, lets a simulator compute what happens, repeats each scenario many times with
different random numbers, and draws conclusions from the spread of outcomes. This is still
an experiment — it tests a hypothesis and produces results — but it has concepts an
ordinary experiment does not: the model and its assumptions, the scenario, the random seed,
the replication, simulated time, and the question every reader asks first, "why should I
believe this model?".

This specification adds those concepts **on top of** experiments. A simulation is an
experiment of the kind "simulation" (`specs/004-experiments`); everything specified there —
pipelines, runs, parameters, metrics, results, conclusions, replication — applies
unchanged. What is added here attaches to it:

| Experiment concept (004) | What simulation adds (here) |
|--------------------------|-----------------------------|
| Experiment | A **model** it simulates, at a stated version |
| Condition | A **scenario**: a named, complete setting of the model's inputs, with a time horizon |
| Run | A **replication**: one run of one scenario with one recorded random seed |
| Replicate number | The seed policy that makes replications independent and repeatable |
| Metric | **Output series** over simulated time, and summaries across replications with their uncertainty |
| Software | The **simulator** that computes the model |
| — | **Credibility**: checks that the model is computed correctly and resembles what it models |
| Sweep | **Sensitivity analysis** and **sampled ensembles** over uncertain inputs |

### The concepts, in one picture

```text
 Hypothesis ◀── tests ── Experiment (kind: simulation) ── simulates ──▶ Model ── version
                              │                                           │  inputs, outputs,
                              │ has                                       │  assumptions
                              ▼                                           ▼
                           Scenario ── sets values of ──────────────▶ Model inputs
                           (baseline, variants; horizon, warm-up)
                              │ run N times
                              ▼
                           Replication = Run + seed ── produces ──▶ Output series, summaries
                              │                                        │
                              └── aggregated per scenario ──▶ mean, spread, interval ──▶ Result
 Credibility: repeatability check · verification cases · validation against observed Dataset
 Exploration: sensitivity analysis · sampled ensemble
```

TRCLI does not simulate anything itself. The researcher's own simulator does the computing;
TRCLI tells it which scenario and seed to use, records what was run, collects what came out,
and keeps the account of why the model can be trusted.
