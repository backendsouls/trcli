### User Story 1 - Record the courses I take and teach (Priority: P1)

A researcher records each course they take: its name and code, the institution, the term,
its credits and hours, who teaches it, and — as it goes — its status and final grade. They
record the courses they teach in the same way. They can see everything for a term, and
their whole history as a transcript.

**Why this priority**: Courses are the unit everything else here is built from, and a plain
record of what was taken, when, and with what grade is useful by itself — it is what a
researcher is asked for in every report and application.

**Independent Test**: Record three courses in two terms with credits, complete two with
grades, record one course taught, and view the transcript with totals per term.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher records a course they take with a title
   and a term, **Then** it is stored with the status "planned".
2. **Given** a course, **When** the researcher records its code, institution, credits,
   hours, instructor, schedule, and notes, **Then** they are saved and shown.
3. **Given** a course, **When** the researcher changes its status (planned, enrolled, in
   progress, completed, failed, dropped, exempted), **Then** the status and date are
   recorded.
4. **Given** a completed course, **When** the researcher records its final grade, **Then**
   the grade is stored in the grading scheme the researcher uses, and is rejected if it is
   outside that scheme.
5. **Given** a course, **When** the researcher records assessments within it — an exam, an
   assignment — each with a date, a weight, and a result, **Then** they are listed, and
   those with a date in the future appear in the view of what is due.
6. **Given** courses in several terms, **When** the researcher lists them filtered by term,
   status, institution, or whether taken or taught, **Then** only matching courses are
   shown.
7. **Given** courses with grades, **When** the researcher asks for their transcript,
   **Then** courses are listed by term with credits and grades, with credits attempted and
   earned and the average grade per term and overall.
8. **Given** a course the researcher teaches, **When** they record it with its term, hours,
   number of students, and their role (lecturer, teaching assistant, intern), **Then** it is
   stored and appears in a list of teaching, separate from the transcript.
9. **Given** a course, **When** the researcher links references, methodologies, manuscripts,
   or research questions to it, **Then** the course lists them and each lists the course.
10. **Given** a course whose end is before its start, credits that are negative, or a term
    in an invalid form, **When** the researcher saves, **Then** the tool rejects it and
    reports every problem together.
11. **Given** a course failed and taken again, **When** both are recorded, **Then** both
    appear in the transcript, and only the passed one counts toward credits earned.
12. **Given** a transcript, **When** the researcher exports it, **Then** a document is
    produced.

---
