# Feature Specification: TRCLI Research Projects, Milestones, and Tasks

**Feature Branch**: `003-research-projects`

**Created**: 2026-10-08

**Status**: Draft

**Amended**: 2026-10-08 — added the to-do list (User Story 7, FR-053 to FR-072, SC-013 to
SC-018), a view of projects, milestones, and tasks as one list.

**Input**: User description: "add concepts of projects, tasks, millistones, projects have a types Free Research (any better name), PhD, Master, Undergraduation, TCC(in portuguese trabalho de conclusao de curso, use english), etc"

## Overview

A researcher rarely does one piece of research at a time, and each piece has a shape set by
what it is for: a doctorate has a qualifying exam and a defense, a capstone project has a
proposal and a final presentation, independent research has neither. This specification
adds the **project** as the unit that organizes a researcher's work inside a workspace, gives
every project a **type** that reflects its purpose, and gives each project **milestones**
and **tasks** to plan and follow it.

A workspace (see `specs/001-research-workspace`) holds everything a researcher has. A
project is one research undertaking within it. Papers, drafts, experiments, datasets, and
every other record can belong to one or more projects, so a paper read for a master's
dissertation can be reused in the doctorate that follows.

### Project types

| Type | What it is |
|------|------------|
| Independent Research | Research not tied to a degree or programme, done on the researcher's own initiative. (Named "Free Research" in the request.) |
| Undergraduate Research | Research done during an undergraduate degree under a supervisor, outside the final-year requirement. |
| Capstone Project | The final project required to complete an undergraduate degree; also called an undergraduate thesis or final-year project. (TCC, *Trabalho de Conclusão de Curso*, in Portuguese.) |
| Master's | Research leading to a master's degree and its dissertation. |
| Doctoral (PhD) | Research leading to a doctorate and its thesis. |
| Postdoctoral | Research done in a postdoctoral position. |
| Funded Project | Research defined by a grant or an institution rather than by a degree. |
| Custom | Any other type the researcher defines. |
