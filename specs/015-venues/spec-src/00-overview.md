# Feature Specification: TRCLI Venues — Places to Publish

**Feature Branch**: `015-venues`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add a new spec for places to publish (events, conferences, journals)"

## Overview

A finished piece of research has to go somewhere, and where it goes matters: for who reads
it, for how it counts, for how long it takes, and for what it costs. Researchers carry this
knowledge in their heads and in bookmarks — which journals fit their topic, which
conference has a deadline in six weeks, which venue took nine months last time, which one
their programme does not count. Every deadline missed and every paper sent to the wrong
place is this knowledge failing.

This specification gives **venues** a place in the workspace: the journals, conferences,
and other places where the researcher publishes or might publish, what is known about each,
their calls and deadlines, the choice of venue for a manuscript, what experience has
taught, and the events attended and talks given.

It **takes over** the venue content of an earlier specification, which now points here:

| Came from | What |
|-----------|------|
| `specs/002-research-lifecycle`, User Story 12 | Venues, calls with deadlines, talks and posters |

### The words

| Term | Meaning in TRCLI |
|------|------------------|
| **Venue** | A place that publishes or presents research, considered as something lasting: a journal, a conference series, a workshop series, a book series, a preprint server. |
| **Event** | One occurrence of a venue that meets: this year's edition of a conference, with its dates and place. A journal has no events. |
| **Call** | An invitation to submit, with its deadlines: a conference's call for papers, a journal's special issue. A journal that always accepts submissions has an open, permanent call. |
| **Talk** | Something the researcher presented at an event: a talk, a poster, a tutorial, a demonstration. |

### The concepts, in one picture

```text
 Venue (journal · conference · workshop · …) ── ranked in ──▶ Ranking schemes
   │  profile: scope, review model, costs, policies, my interest
   │
   ├── has ──▶ Event (edition: year, dates, place) ── has ──▶ Call ── deadlines ──▶ What is due (003)
   │                    │                                       │
   │                    └── I attend / present ──▶ Talk         └── a Manuscript is aimed at it
   │
   ├── candidate for ──▶ Manuscript (008) ── shortlist, comparison, target
   └── history ◀── Submissions and decisions (002) ── what experience taught
```
