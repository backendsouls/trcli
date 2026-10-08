# Feature Specification: TRCLI Templates

**Feature Branch**: `007-templates`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add the concept of templates for papers, reports, etc. on its own spec"

## Overview

Researchers write the same shapes of document again and again: an article with
introduction, methods, results, and discussion; a thesis with its front matter and
chapters; a weekly report to a supervisor; a response to reviewers; a research proposal.
Each has a structure someone expects, and much of its content — title, authors,
affiliations, abstract, reference list, tables of results — is already recorded in the
workspace.

This specification adds the **template**: a named, reusable skeleton for a document, made
of sections, guidance for the writer, and placeholders the tool fills from the workspace.
A researcher starts a document from a template instead of from a blank page, keeps the
parts that come from the workspace up to date as the research changes, and checks that the
document still has the shape its template requires.

### Kinds of template

| Kind | Used for | Examples provided |
|------|----------|-------------------|
| Paper | A manuscript for a draft | Research article (introduction, methods, results, discussion), conference paper, short paper, review article |
| Thesis | A degree document | Monograph thesis, thesis by articles, capstone project, single chapter |
| Report | An account of work done | Activity report (personal, supervisor, funder), experiment report, progress report |
| Proposal | A plan submitted for approval | Research proposal, pre-registration |
| Protocol | A plan for a structured activity | Literature review protocol |
| Letter | Correspondence around a submission | Response to reviewers, cover letter |
| Notes | Working documents | Reading notes, meeting notes |
| Other | Anything else the researcher defines | — |

### Where templates plug in

```text
                       ┌── start a document ──▶ manuscript of a Draft            (001)
 Template ── applied ──┼── lay out ───────────▶ Activity Report                  (006)
   sections            ├── lay out ───────────▶ Experiment report, Review export (004, 005)
   guidance            ├── lay out ───────────▶ Response letter, Reading notes   (002, 005)
   placeholders ◀── filled from ── workspace records (drafts, staff, references,
                                   results, figures, milestones, …)
```

Other specifications produce documents with a fixed layout; with this one, each of those
layouts becomes a template the researcher can choose or replace.
