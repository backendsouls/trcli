## Assumptions

- **This specification owns literature.** User Stories 1 and 3 of
  `specs/001-research-workspace` and User Story 1 of `specs/002-research-lifecycle` are
  replaced by this one; those specifications keep a short pointer in their place.
- **It depends on the base workspace** (`specs/001-research-workspace`) for the workspace
  itself, tags, notes, links, drafts, research questions, the audit trail, and common
  behavior. Creating the workspace is specified there.
- **"Reference" is the general word; "paper" is its most common kind.** Other
  specifications that say "paper" mean any reference.
- **"Literature review" is the name used for what the original request called
  "bibliographic research".** The two-stage screening and the flow summary follow the
  practice systematic reviews are expected to report; narrative reviews may skip stages and
  extraction.
- **One screener.** As in the base workspace, one researcher uses the tool, so there is no
  independent double screening or reconciling of disagreements between reviewers.
- **Searches are recorded, not run.** The researcher searches the sources themselves and
  records what they did; the tool does not query bibliographic databases by keyword. Online
  lookup fetches the details of one known identifier at a time.
- **Relations are entered by the researcher.** The tool does not read reference lists out of
  documents or fetch citation counts from outside; "most cited" counts relations recorded
  within the library.
- **Full texts are referenced, not stored or downloaded.** The tool records where a local
  copy is; it does not fetch, store, or read the content of documents, and annotations are
  typed by the researcher, not extracted from a file.
- **Online lookup uses freely available public catalogues** that need no account, contacts
  them only when asked, and sends only the identifier.
- **Citation styles follow the open standard for style definitions** in common use, so
  styles published for other tools can be supplied; which styles are built in is decided at
  planning time and includes at least one author–date and one numeric style.
- **Synchronizing continuously with another reference manager** is specified with
  integrations in `specs/002-research-lifecycle`; this specification covers import and
  export on request.
- **Concepts and glossary terms** linked to references remain in
  `specs/002-research-lifecycle`.
- **Projects** (`specs/003-research-projects`) scope references like any other record; a
  reference may belong to several projects and is stored once.
