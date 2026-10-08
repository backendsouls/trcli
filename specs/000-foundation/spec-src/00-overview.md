# Feature Specification: TRCLI Foundation and Architecture

**Feature Branch**: `000-foundation`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec 000-foundation, the first one for the project foundation and architecture"

## Overview

TRCLI (The Research CLI) is specified as many features — literature, experiments,
manuscripts, projects, people, and more — each in its own specification. All of them stand
on the same ground: a workspace to keep things in, one way of naming and finding records,
one way of checking input, one way of answering, one record of everything that happened.
If each feature built that ground for itself, the tool would feel like fifteen tools.

This specification is that ground. It is numbered 000 because it is built first and
everything else depends on it. It has two readers:

- the **researcher**, for whom it defines how TRCLI behaves whatever they are doing in it;
- the **contributor**, for whom it defines what every feature gets for free and what every
  feature must respect.

It **takes over** the foundational content that was written into the first feature
specification before the product was divided, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 1 | Creating and using a workspace; behaviour shared by all records |
| `specs/001-research-workspace`, User Story 10 | The audit trail and local telemetry |
| `specs/001-research-workspace`, FR-001 to FR-015, FR-051 to FR-055 | Workspace, common behaviour, input validation, audit, telemetry |
| `specs/001-research-workspace/contracts` | CLI conventions, output and exit codes, configuration |

### What "architecture" means in this specification

A specification says what a system must do, not how it is built. The architecture appears
here as the **qualities every part of TRCLI must have** and the **rules every feature must
follow** — stated so that they can be tested — and not as a choice of technology. The
technology, the code structure, and the reasons for them belong to this specification's
implementation plan.

### The ground everything stands on

```text
 Researcher ──▶ one command grammar ──▶ any feature (references, runs, tasks, …)
                                             │
            ┌────────────────────────────────┼────────────────────────────────┐
            ▼                                ▼                                ▼
   Input is checked first          The record is changed            The answer comes back
   every value, all problems       all of it or none of it          for people or for programs
   together, nothing changed       with its audit entry             with a meaningful exit code
            │                                │                                │
            └──────────────── in one Workspace, found from where you stand ───┘
                 records with handles · tags · notes · links · guarded deletion
                 settings in layers · help everywhere · local, offline, yours
```
