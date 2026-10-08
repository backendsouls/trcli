# Feature Specification: TRCLI Conventions, Methodologies, and the Implicit Side of Research

**Feature Branch**: `014-conventions-methods`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for conventions (from lab, from community), methodologies (from the research, lab, community, literature), all the meta parts of a research that in general is implicit"

## Overview

Much of what makes research work is never written down. Files are named a certain way
"because that is how we do it here". Samples are randomized by a procedure a former student
set up. A threshold of 0.05 is used because the field uses it. Outliers are removed by a
rule somebody decided two years ago, for a reason nobody remembers. A new member learns all
this slowly, by breaking it; a reviewer asks about it and the answer takes a week to
reconstruct; and when the person who knew leaves, it is gone.

This specification is for the **implicit side of research**: the conventions, methods,
assumptions, and decisions that shape the work without appearing in it. It gives each a
place to be written down, says **where it comes from** and **how binding it is**, shows it
to the researcher at the moment it applies, and records when the work knowingly departs
from it.

It **takes over** the methodology content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 7 (methodology part), FR-039 | Methodologies: name, description, procedure, references |

### What is made explicit

| Kind | What it is | Example |
|------|------------|---------|
| **Convention** | An agreed way of doing something where other ways were possible | "Dataset folders are named `<year>-<source>-<version>`" |
| **Methodology** | How a kind of work is carried out, step by step | "Five-fold cross-validation with a held-out test set" |
| **Assumption** | Something the research takes as true without showing it | "Annotators were independent of one another" |
| **Decision** | A choice made between alternatives, with its reason | "We excluded sessions shorter than 30 s, because…" |
| **Checklist** | A list of things a piece of work must satisfy | A reporting guideline for a kind of study; the lab's pre-submission list |

### Where it comes from

Every one of these has an **origin**, because "who says so?" decides how freely it may be
changed:

| Origin | Meaning | Who may change it |
|--------|---------|-------------------|
| **Own research** | Chosen by the researcher for this work | The researcher |
| **Lab** | Agreed in the research group | The group; a member records a deviation |
| **Institution** | Required by the university, programme, or ethics body | Not the researcher |
| **Community** | The practice of the field | Nobody in particular; departing from it needs justifying |
| **Venue or funder** | Required by where the work is sent or who pays | Not the researcher |
| **Literature** | Taken from a published work, which is cited | The researcher, by adapting it and saying how |

### The concepts, in one picture

```text
 Origin (own · lab · institution · community · venue/funder · literature ── cites ──▶ Reference)
    │
    ▼
 Convention ── applies to ──▶ kinds of record · projects · topics
 Methodology ── used by ───▶ Experiment · Literature Review · Model      ── adapted from ──▶ Methodology
 Assumption ── underlies ──▶ Experiment · Model · Result · Manuscript
 Decision ──── concerns ───▶ any record            ── chose among ──▶ Alternatives
 Checklist ─── applied to ─▶ Manuscript · Experiment ── item by item ──▶ Addressed where?

 At the moment of work: "what applies here?"      When departing: Deviation, with a reason
 Together: the Handbook ── shared as a pack ──▶ another workspace
```
