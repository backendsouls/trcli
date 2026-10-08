# Feature Specification: TRCLI Research Workspace

**Feature Branch**: `001-research-workspace`

**Created**: 2026-10-08

**Status**: Draft

**Amended**: 2026-10-08 — added research questions and hypotheses (User Story 2), results,
figures and tables (User Story 6), and online lookup of paper details (User Story 1).
Existing stories were renumbered; requirement numbers were kept and new ones appended.
Experiments, pipelines, runs, results, figures, and tables (User Stories 5 and 6) were then
moved to `specs/004-experiments`; pointers remain in their place. Papers, citations, online
lookup, and bibliographic research (User Stories 1 and 3) were moved to
`specs/005-literature`; User Story 1 now covers the workspace itself. Draft papers (User
Story 4) were moved to `specs/008-manuscripts`, and courses (User Story 11) to
`specs/011-courses-roadmaps`. Staff (User Story 9) moved to `specs/012-people`,
and research questions and hypotheses (User Story 2) to `specs/013-ideas-questions`.
Methodologies (part of User Story 7) moved to `specs/014-conventions-methods`.
The workspace itself, the behaviour shared by all records, input validation, and the audit
trail (User Stories 1 and 10) moved to `specs/000-foundation`. What remains specified here
is datasets (User Story 7) and environment and reproducibility (User Story 8).

**Input**: User description: "Develop trcli, a researcher tool for modern research; it can create/crud and track papers, save citations, crud drafts papers, crud experiments(with pipelines, including manual steps), crud methodologies, crud data/datasets, crud staffs, telemetry, audit, environment, reproducibility, crud bibliographic research, courses, etc"

## Overview

TRCLI (The Research CLI) is a command-line workspace in which a researcher keeps everything
that makes up a research project in one place and in one consistent shape: the questions
they are trying to answer, the papers they read, the citations they save, the literature
reviews they run, the papers they are writing, the experiments they execute and the results
those produce, the methodologies and datasets those experiments rely on, the
people involved, and the courses that feed the work. Every record can be created, viewed,
changed, and removed, and records can be linked to one another so that a claim in a draft
can be traced back to the result, experiment, dataset, method, and environment that produced
it, and forward to the research question it helps answer.

The product is delivered as a series of independently usable slices, ordered below by
priority. Each slice is valuable on its own.
