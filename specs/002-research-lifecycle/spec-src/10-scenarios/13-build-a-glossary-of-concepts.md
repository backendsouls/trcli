### User Story 13 - Build a glossary of concepts (Priority: P13)

A researcher records the terms and concepts of their field with definitions, synonyms, and
the papers that define or use them, and links concepts to one another as broader, narrower,
or related.

**Why this priority**: A shared vocabulary helps writing and onboarding, and nothing else
depends on it.

**Independent Test**: Add two concepts, relate one as narrower than the other, link a
defining paper, and look a concept up by a synonym.

**Acceptance Scenarios**:

1. **Given** a workspace, **When** the researcher adds a concept with a term and a
   definition, **Then** it is stored.
2. **Given** a concept, **When** the researcher adds synonyms and looks one up, **Then** the
   concept is found.
3. **Given** two concepts, **When** the researcher relates them as broader, narrower, or
   related, **Then** the relation is visible from both.
4. **Given** a concept, **When** the researcher links papers to it, **Then** the concept
   lists them and each paper lists the concept.
5. **Given** a term that already exists, **When** the researcher adds it again, **Then** the
   tool reports the existing concept.

---
