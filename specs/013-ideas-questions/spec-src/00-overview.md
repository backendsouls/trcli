# Feature Specification: TRCLI Ideas, Topics, Questions, and Hypotheses

**Feature Branch**: `013-ideas-questions`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for questions, hypothesis, ideas, topics so that is not forgotten"

## Overview

Research begins as a thought: "what if the effect disappears with more data?", "someone
should compare these two methods", "I keep running into this topic". Most such thoughts are
lost — they arrive in the middle of something else, are written on whatever is at hand, and
are never seen again. The ones that survive are not necessarily the best ones, only the
ones that happened to be remembered.

This specification is about **not forgetting**. It gives every thought a place to land in
seconds, a moment at which it is sorted, a path by which the good ones grow into research
questions and testable hypotheses, and a habit by which nothing sits unseen for long.

It **takes over** the content on questions and hypotheses of the first specification, which
now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 2 | Research questions, sub-questions, hypotheses, and linking records to them |

It **adds** ideas with an inbox, the sorting of that inbox, topics, and the review that
brings things back before they are forgotten.

### Four kinds of thought

| Kind | What it is | Example |
|------|------------|---------|
| **Idea** | Anything worth not losing; unformed is fine | "Try the method from the vision paper on our audio data" |
| **Topic** | An area of interest that things gather around | "Robustness to label noise" |
| **Research question** | Something the research commits to answering | "Does pre-training reduce the labelled data needed?" |
| **Hypothesis** | A testable claim that would answer a question | "Pre-training halves the labelled data needed for 90% accuracy" |

### How a thought travels

```text
 capture ──▶ Inbox ── sort ──┬──▶ Idea (kept, developing) ──┬──▶ Research Question ──▶ Hypothesis ──▶ Experiment (004)
 (seconds)                   │         │                    ├──▶ Task / Project (003)        │
                             │         └── grouped by ──▶ Topic ◀── also groups ── References, Questions, …
                             ├──▶ merged into an existing one
                             ├──▶ parked until a date ──▶ comes back by itself
                             └──▶ discarded, with a reason (kept, not erased)

 Review: anything not looked at for too long, or due to come back, is brought before the researcher again.
```
