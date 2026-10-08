### User Story 7 - Record publication and list my output (Priority: P7)

When a manuscript is published, the researcher records where and when: the venue, the
date, the volume and pages, its persistent identifier, its licence, and whether it is
openly accessible. The published work becomes a reference in their own library, so they can
cite it like any other. They can then list everything they have published, in the form a
CV, a programme, or a funder asks for.

**Why this priority**: Publication is the end of the road, and the list of publications is
what a researcher is most often asked to produce. It comes last because it needs
manuscripts that have travelled the whole road.

**Independent Test**: Record the publication of a manuscript, confirm a matching reference
exists in the library and is linked to it, and produce a list of publications by year in a
citation style.

**Acceptance Scenarios**:

1. **Given** an accepted manuscript, **When** the researcher records its publication with a
   venue and a date, **Then** its stage becomes "published" and the details are shown.
2. **Given** a publication, **When** the researcher records its volume, issue, pages,
   persistent identifier, web address, licence, and whether it is openly accessible,
   **Then** they are saved.
3. **Given** a persistent identifier and a network connection, **When** the researcher asks
   the tool to fill in the publication details from it, **Then** the details are fetched,
   shown, and saved on acceptance, as for any reference.
4. **Given** a recorded publication, **When** it is saved, **Then** a reference for the
   published work exists in the library, linked to the manuscript; if one already exists,
   the tool links it instead of creating a second.
5. **Given** a manuscript first shared as a preprint and later published, **When** both are
   recorded, **Then** the manuscript shows both, and they are versions of the same work.
6. **Given** published manuscripts, **When** the researcher asks for their publication list,
   **Then** they are listed in a chosen citation style, grouped by year or by kind, newest
   first, with the researcher's own name marked.
7. **Given** a publication list, **When** the researcher filters it by period, kind,
   project, or grant, **Then** only matching publications are shown.
8. **Given** a publication list, **When** the researcher asks to include work in progress,
   **Then** submitted and accepted manuscripts are added under their own headings.
9. **Given** a publication list, **When** the researcher exports it, **Then** a document, or
   a bibliography file, is produced.
10. **Given** a publication date in the future, an identifier in an invalid form, or a
    publication for a manuscript that is abandoned, **When** the researcher saves, **Then**
    the tool rejects it.
11. **Given** a published manuscript, **When** the researcher records a correction or a
    retraction with its date and note, **Then** it is shown with the publication and in
    the publication list.

---
