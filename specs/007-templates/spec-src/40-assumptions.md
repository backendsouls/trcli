## Assumptions

- **Templates are for documents.** "Templates for papers, reports, etc." is read as
  skeletons for the documents a researcher writes or the tool produces. Reusable shapes for
  *records* already exist under other names and stay where they are: milestones proposed
  per project type (`specs/003-research-projects`), copying a pipeline or replicating an
  experiment (`specs/004-experiments`), named report definitions (`specs/006-reports`).
- **Documents are plain text in a writing format the researcher already uses.** The tool
  creates and updates text files; it does not typeset them, convert between formats, or
  produce word-processor files. Which writing formats the provided templates come in is
  decided at planning time, with at least two in common use among researchers.
- **The tool is still not an editor.** The researcher writes in their own editor. The tool
  reads a document only to refresh managed parts and to check structure and counts.
- **Word counts are approximate** in the way all word counts are: they count the words of
  the text and ignore the marks of the writing format as far as the format allows. They are
  a guide to a venue's limit, not a guarantee of its own count.
- **Publishers' and institutions' official templates are not shipped.** Their licences
  vary. The provided templates give common structures; a researcher brings an official
  template in as their own, and adds placeholders to it if they want it filled.
- **"Not revealing the authors"** is checked only against names, affiliations, and
  identifiers recorded in the workspace. It cannot detect self-citation phrasing or other
  indirect clues.
- **Managed parts are marked with ordinary comments of the writing format**, so a document
  remains valid and readable without the tool, and can be sent to co-authors who do not
  use it.
- **This specification depends on** `specs/001-research-workspace` (drafts, staff, audit),
  and, for each document it lays out, on the specification that owns it:
  `specs/006-reports` (activity reports and definitions), `specs/004-experiments`
  (experiment reports, results, figures), `specs/005-literature` (reference lists, review
  exports, reading notes), `specs/003-research-projects` (project details, progress), and
  `specs/002-research-lifecycle` (response letters, notebook, pre-registration, funders).
  A template for a document works once that document's owner exists.
- **Those specifications keep ownership of content.** They decide what a document contains
  and what is withheld; templates decide only how it is arranged.
- **Personal templates are kept with the user's settings**, outside any workspace; workspace
  templates are part of the workspace and are included in its backups and exports.
- **No template marketplace or online catalogue.** Sharing is by handing over a file.
- **Single researcher**, as elsewhere: there are no permissions on templates.
