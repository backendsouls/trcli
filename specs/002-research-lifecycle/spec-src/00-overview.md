# Feature Specification: TRCLI Research Lifecycle Extensions

**Feature Branch**: `002-research-lifecycle`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "Capabilities missing from the TRCLI research workspace: reading notes and annotations; tasks and deadlines; backup, export and migration; submissions and peer review; funding and grants; ethics and compliance; lab notebook; code and software; parameters and metrics for runs; pre-registration; collaboration; publishing outputs; venues, conferences and talks; instruments, samples and materials; glossary and concepts; integrations; dashboard. (Research questions and hypotheses, results/figures/tables, and online metadata lookup were added to specs/001-research-workspace instead.)"

## Overview

The research workspace specified in `specs/001-research-workspace` covers the path from
literature to results. This specification adds what surrounds that path in a real project:
the day-to-day working records (reading annotations, tasks, a lab notebook), the rigor around
experiments (parameters and metrics, code versions, pre-registration), the obligations of a
project (peer review, ethics, funding), the safety of the workspace itself (backup, restore,
upgrade), and the ways the work reaches other people (venues and talks, publishing outputs,
integrations, collaboration).

Every record type added here behaves like those of the base workspace: it can be created,
listed, viewed, updated, deleted, tagged, linked to other records, and is audited. The
stories are ordered in three tiers — needed early, needed for real projects, and later —
and each is usable on its own once the base workspace it builds on exists.
