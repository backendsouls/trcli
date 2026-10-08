# Feature Specification: TRCLI Literature, References, and Bibliography

**Feature Branch**: `005-literature`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add concepts of bibliography/references/literature as a separated spec on its own"

## Overview

Every piece of research stands on what others have written. This specification gathers
everything TRCLI knows about that body of work into one place: the references a researcher
collects, how they are brought in and kept clean, how they are cited, how they are read and
annotated, how they relate to one another, and how a formal literature review is conducted
and synthesized.

It **takes over** the literature content previously spread across two specifications, which
now point here:

| Came from | What |
|-----------|------|
| `specs/001-research-workspace`, User Story 1 | Papers, citations, import and export, duplicates, online lookup |
| `specs/001-research-workspace`, User Story 3 | Bibliographic research (literature reviews) |
| `specs/002-research-lifecycle`, User Story 1 | Reading annotations and structured summaries |

It **adds** what was not specified before: references of kinds other than papers, named
bibliographies, a reading queue, relations between references, two-stage screening with a
flow summary, and a synthesis matrix.

### The three words in the request

| Term | Meaning in TRCLI |
|------|------------------|
| **Reference** | One work the researcher may read or cite: an article, a book, a thesis, a web page, a dataset, and so on. The record that holds its details. |
| **Literature** | The references taken together as a body of knowledge: what has been read, what it says, how the works relate, and what a review of them concludes. |
| **Bibliography** | A named, ordered list of references assembled for a purpose — a chapter, a paper, a course — and written out in a citation style. |

Elsewhere in TRCLI the word "paper" is used for what is called a **reference** here; a
paper is the most common kind of reference, and every rule that mentions a paper applies to
any reference.

### The concepts, in one picture

```text
                      ┌──── cites / extends / contradicts / version of ────┐
                      ▼                                                    │
 Online catalogue ─▶ Reference ──── has ──▶ Citation (key) ──── in ──▶ Bibliography
 Bibliography file ─▶   │  │                    ▲                          │
                        │  └─ has ─▶ Annotation │ used by                  ▼
                        │            Summary    │                  written in a Citation Style
                        │                     Draft
                        └─ candidate in ─▶ Literature Review ─▶ Search
                                               │                Screening Decision
                                               └─ extracts ──▶ Synthesis Matrix
```
