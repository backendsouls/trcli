## Assumptions

- **This specification owns courses.** User Story 11 of `specs/001-research-workspace`
  (FR-056 and FR-057) is replaced by this one; that specification keeps a short pointer.
- **"Curricular matrix" is rendered as "curriculum"**, with "matrix" kept for its view as a
  grid by term. It is the programme's official structure (*matriz curricular*), as the
  researcher records it.
- **"Graduate roadmap" is read as the path through a graduate programme**: its requirements
  besides courses and their time limits. **"Roadmap" alone is read as a personal learning
  roadmap.** They are two record types because one is imposed by a programme and the other
  chosen by the researcher.
- **The researcher enters their programme's rules; the tool does not know them.** It ships
  no curricula of real institutions and does not claim to know any programme's regulations.
  Proposed graduate requirements are generic starting points to edit:
  - *Master's*: minimum credits; language proficiency; qualifying exam or proposal defense;
    dissertation submitted; defense; final version deposited.
  - *Doctoral*: minimum credits; proficiency in one or two languages; qualifying exam;
    proposal defense; teaching internship; a paper submitted or published; thesis
    submitted; defense; final version deposited.
- **The tool is the student's own record, not the institution's system.** It is not an
  official transcript, does not enrol anyone, and does not exchange data with academic
  systems. Its totals help the student; the institution's are the ones that count.
- **Undergraduate programmes are covered by the same curriculum, progress, and plan
  stories.** The graduate roadmap is offered for master's and doctoral programmes; an
  undergraduate may still create a roadmap of requirements of their own (internship,
  complementary hours, capstone).
- **Project milestones and graduate requirements are related but distinct.** A milestone
  (`specs/003-research-projects`) is a dated checkpoint of the research project that the
  researcher plans; a requirement is an obligation of the programme. A requirement can take
  a milestone as evidence. The typical milestones proposed per project type there and the
  requirements proposed here overlap by design and are linked, not merged.
- **Tasks, the view of what is due, and projects** are those of
  `specs/003-research-projects`; **references** those of `specs/005-literature`;
  **manuscripts** and their stages those of `specs/008-manuscripts`; **staff**, links, and
  the audit trail those of `specs/001-research-workspace`.
- **Averages follow the commonest rule** — weighted by credits within one grading scheme.
  Institutions with another rule should treat the figure as indicative.
- **Proposed term plans are simple and explainable**: earliest term that respects
  prerequisites, offering, and load. The tool does not optimize for workload balance,
  timetable clashes, or instructor.
- **Courses taught are recorded for the researcher's own account** of their teaching; class
  lists, attendance, and grading of students are out of scope.
- **Skill levels are the researcher's self-assessment** on a four-step scale.
- **Single researcher.** Supervisors and programme offices receive exported documents; they
  do not use the workspace.
