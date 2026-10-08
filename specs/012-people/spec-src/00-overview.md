# Feature Specification: TRCLI People and Lab Management

**Feature Branch**: `012-people`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for staff an people management, just the minimum for a head of a lab handle the lab for example"

## Overview

Research is done by people, and someone — a lab head, a group leader, a supervisor with a
handful of students — has to keep them in mind: who is in the group, since when and until
when, what each is working on, who supervises whom, when they last talked, and what happens
to the work when somebody leaves.

This specification covers that, and deliberately **only the minimum**. It is a lab head's
notebook about their people, not a personnel system.

It **takes over** the staff content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 9 | The staff register and the assignment of people to work |

It **adds** four small things a lab head needs: each person's position and period in the
group, an overview of who is doing what, supervision with a record of meetings, and an
orderly handover when someone leaves.

### What this is not

| Not included | Why |
|--------------|-----|
| Salaries, contracts, leave, evaluations | That is the institution's personnel system |
| Accounts, logins, permissions | People here are records, not users of the workspace |
| Time sheets, hours worked | Out of scope across TRCLI |
| Recruitment, applications | Outside the life of the lab's research |

### The concepts, in one picture

```text
 Person ── has a ──▶ Position in the group (role, from, until, funding, status)
   │
   ├── supervised by ──▶ Person
   ├── assigned to ───▶ Project · Manuscript · Experiment · Step · Task · Dataset
   ├── met in ────────▶ Meeting ── agreed ──▶ Action (a task)
   └── on leaving ────▶ Handover: everything they hold, reassigned or closed
```
