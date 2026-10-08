# Feature Specification: TRCLI Courses, Curricula, and Roadmaps

**Feature Branch**: `011-courses-roadmaps`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for courses, curricular matrix, roadmaps, graduate roadmaps etc"

## Overview

A researcher in a degree programme is on two roads at once. One is the research itself,
which the other specifications cover. The other is the **programme**: the courses to take,
the credits to earn, the exams to pass, the things to deliver, each by a certain time. Lose
track of the second and the first cannot be finished, however good it is. Beyond any
programme there is a third road, the researcher's own learning: the things they decided to
study to be able to do the work.

This specification gathers that side of academic life into one place.

It **takes over** the course content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 11 | Courses taken and taught, linked to research records |

It **adds** the programme and its curriculum, progress against it, planning of terms, the
graduate roadmap of requirements beyond courses, and personal learning roadmaps.

### The words in the request

| Term | Meaning in TRCLI |
|------|------------------|
| **Course** | One course the researcher takes or teaches in one term, with its credits, grade, and status. |
| **Curriculum** *(curricular matrix)* | The programme's official list of what must and may be studied: each component with its credits, whether it is mandatory or elective, the term it is suggested for, and what must be completed before it. |
| **Graduate roadmap** | Everything a graduate programme requires besides passing courses — a minimum of credits, a language exam, a qualifying exam, a proposal defense, a teaching internship, a publication, the thesis — each with a time limit counted from enrolment. |
| **Roadmap** *(learning roadmap)* | The researcher's own plan toward a goal they chose — "be able to do Bayesian analysis" — as ordered stages of things to study and do. |

### The concepts, in one picture

```text
 Programme ── has ──▶ Curriculum ── lists ──▶ Component ── requires ──▶ Component
     │                (matrix)                  │ (credits, category, suggested term)
     │                                          ▲ fulfils
     │ enrolled since                           │
     │                                   Course (taken, in a Term, with grade)
     │                                          │
     ├── has ──▶ Graduate Roadmap ── lists ──▶ Requirement ◀── evidence ── Course / Manuscript /
     │           (time limits from enrolment)   (status, due by)            Milestone / Document
     │
     └── belongs to a ──▶ Project (doctoral, master's, …)                     (003)

 Learning Roadmap ── has ──▶ Stage ── has ──▶ Item (course, reference, skill, task) ── depends on ──▶ Item
```
