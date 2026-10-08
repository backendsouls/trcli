# Feature Specification: TRCLI Manuscripts

**Feature Branch**: `008-manuscripts`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "separate the concept of papers, reports, etc. and crud operations on it from the first spec to its own spec"

## Overview

Research ends in writing: an article, a conference paper, a thesis, a technical report, a
proposal. Each is a piece of work in its own right, with authors, a destination, a
deadline, a history of versions, and a long road from idea to publication. This
specification gathers everything TRCLI knows about **what the researcher writes** into one
place, under one name: the **manuscript**.

It **takes over** the writing content of the first specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 4 | Draft papers: create, list, view, update, delete; stages; versions; links to citations and evidence |

It **adds** what was not specified before: kinds of manuscript other than papers, authors'
contributions, a manuscript's parts (chapters and sections) with their own progress,
manuscripts made of other manuscripts, and the record of publication.

### The word "manuscript"

A **manuscript** is anything the researcher writes for others to read: a paper, a report, a
thesis. The first specification called this a "draft"; that word is kept as another name
for it, and wherever another specification says "draft" it means a manuscript.

A manuscript is a **record about** a piece of writing — what it is, who wrote it, where it
stands. The writing itself lives in files the researcher edits with their own tools; the
manuscript knows where those files are.

### Kinds of manuscript

| Kind | Examples |
|------|----------|
| Paper | Journal article, conference paper, workshop paper, short paper, preprint, review article |
| Thesis | Doctoral thesis, master's dissertation, capstone project, thesis chapter |
| Report | Technical report, lab report, project report, white paper |
| Proposal | Research proposal, grant application, thesis proposal |
| Book | Book, book chapter |
| Presentation | Abstract, poster, slides |
| Other | Essay, note, anything else the researcher writes |

### Three things that are easily confused

| | What it is | Specified in |
|---|-----------|--------------|
| **Manuscript** | What *you* write, tracked from idea to publication | here |
| **Reference** | What *others* wrote, which you read and cite | `specs/005-literature` |
| **Activity report** | What *the tool* produces from the workspace's history | `specs/006-reports` |

A "report" in this specification is a document the researcher writes — a technical report —
not the tool's account of the week.

### The concepts, in one picture

```text
 Project ── contains ──▶ Manuscript ── has ──▶ Parts (chapters, sections)
                           │  │  │                  └─ may be another Manuscript
        Staff ── authors ──┘  │  └── has ──▶ Versions
        (order, contribution) │
                              ├── cites ──▶ Citations / Bibliography        (005)
                              ├── reports ─▶ Results, Figures, Tables        (004)
                              ├── answers ─▶ Research Questions              (001)
                              ├── written in ▶ Document from a Template      (007)
                              ├── sent as ──▶ Submissions and reviews        (002)
                              └── becomes ──▶ Publication ── also a ──▶ Reference (005)
```
