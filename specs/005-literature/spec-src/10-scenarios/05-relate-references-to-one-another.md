### User Story 5 - Relate references to one another (Priority: P5)

A researcher records how works relate: this one cites that one, extends it, contradicts it,
replicates it, or is another version of it (a preprint and its published article). They can
then look at one reference and see the works around it, and see which references in their
library are most cited by the others.

**Why this priority**: Literature is a web, not a list. Relations are what let a researcher
follow an argument across works, but they are only worth recording once the library exists.

**Independent Test**: Relate four references with three kinds of relation, view one
reference's neighbourhood, and list the references most cited within the library.

**Acceptance Scenarios**:

1. **Given** two references, **When** the researcher records that one cites, extends,
   contradicts, replicates, or reviews the other, with an optional comment, **Then** the
   relation is shown from both references, each from its own point of view.
2. **Given** a preprint and its published version, **When** the researcher records them as
   versions of the same work and names the preferred one, **Then** each shows the other,
   and citing the work uses the preferred version unless the researcher chooses otherwise.
3. **Given** a reference, **When** the researcher asks for its neighbourhood, **Then** the
   works it relates to and the works that relate to it are listed by kind of relation.
4. **Given** a library with relations, **When** the researcher asks which references are
   most cited within it, **Then** they are listed with their counts.
5. **Given** a reference, **When** the researcher asks which works in the library
   contradict it, **Then** they are listed with the comments recorded.
6. **Given** a relation from a reference to itself, or a relation that already exists,
   **When** the researcher saves it, **Then** the tool rejects it.
7. **Given** a reference with relations, **When** it is merged into another, **Then** its
   relations move to the remaining reference and none is duplicated.

---
