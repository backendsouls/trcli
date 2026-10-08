#### Reference library

- **FR-001**: Users MUST be able to create, list, view, update, and delete references, each
  with a title, a kind, ordered authors, a year, and an abstract.
- **FR-002**: The system MUST support these kinds of reference: journal article, conference
  paper, preprint, book, book chapter, thesis or dissertation, report, web page, dataset,
  software, standard, patent, and other.
- **FR-003**: Users MUST be able to record the details particular to each kind, including
  venue, volume, issue, pages, publisher, edition, editors, institution, degree, web
  address, and date accessed.
- **FR-004**: Users MUST be able to record a reference's persistent identifiers, web
  address, language, keywords, and the location of a local copy, without the tool taking a
  copy of the file.
- **FR-005**: Users MUST be able to filter references by kind, author, year or range of
  years, venue, tag, language, reading status, rating, and bibliography, sort them, and
  search by words in title, authors, abstract, keywords, and notes; search and sorting MUST
  ignore letter case and accents.
- **FR-006**: The system MUST detect a likely duplicate when a reference is added, imported,
  or looked up — equal persistent identifier, or equal title, first author, and year after
  ignoring case, accents, and punctuation — and let the user add, merge, or cancel.
- **FR-007**: Users MUST be able to list groups of likely duplicates across the library.
- **FR-008**: Merging two references MUST leave one reference carrying the details of both,
  MUST NOT replace a filled detail with an empty one, and MUST move every citation,
  annotation, tag, note, link, relation, bibliography entry, and review entry to it.
- **FR-009**: Users MUST be able to list every reference by an author regardless of how the
  name was written, and to state that two written names are the same person.
- **FR-010**: Before deleting a reference, the system MUST list every bibliography, review,
  draft, and other record that refers to it and MUST require explicit confirmation.
