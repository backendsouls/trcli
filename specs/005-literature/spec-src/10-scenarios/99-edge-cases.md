### Edge Cases

- The same work is added twice, by hand, by import, or by lookup: the likely duplicate is
  reported and the researcher chooses to add, merge, or cancel.
- A reference has no author, no year, or an organization as its author: it is accepted;
  the citation key and the formatted citation are built from what is available.
- An author's name is written in several ways across references ("Silva, A.", "Ana
  Silva"): the works are found together, and the researcher can state that two names are
  the same person.
- A title or name contains characters outside the Latin alphabet, accents, or mathematics:
  it is stored and shown as written, and sorting and duplicate detection ignore accents and
  letter case.
- Two citations would receive the same key: the tool makes the key unique and says so.
- A citation key is changed while drafts and bibliographies use it: the tool lists where it
  is used and requires confirmation.
- An import file is partly invalid: valid entries are imported, invalid ones are reported
  one by one, and a final count of both is shown.
- An import file is very large: progress is shown and the import can be interrupted,
  leaving what was already imported in place and reported.
- An online lookup returns details that conflict with what the researcher entered: both are
  shown and the researcher chooses, field by field or all at once.
- An online lookup is slow or the catalogue is unavailable: the tool gives up after a
  short, stated wait, says so, and the rest of the tool keeps working.
- An online lookup returns incomplete or malformed details: they are validated like any
  other input, and only valid details are offered.
- A reference's local copy is moved or deleted outside the tool: the reference is kept, and
  the tool reports the copy as missing when it is next needed.
- A quote is added to a reference that has no local copy: it is accepted; the page number
  is the researcher's responsibility.
- A reference is deleted while it is in bibliographies, reviews, or drafts: the tool lists
  each of them and requires confirmation; the review's counts are updated.
- A reference is removed from the library after being included in a completed review: the
  tool refuses until the review is reopened.
- A candidate is excluded on title and abstract and later screened on full text: the tool
  refuses, since only candidates included at the first stage reach the second.
- A candidate found by two searches is attached twice: it is one candidate, and both
  searches are recorded as having found it.
- A review has no searches, or its recorded result counts are lower than its number of
  candidates: the flow summary is still produced and states the inconsistency.
- A bibliography is empty: it can be exported, and the tool says it is empty.
- Versions of the same work are both cited in one bibliography: the check reports it.
- A reference is in a language the citation style does not handle specially: it is
  formatted with the style's general rules.
