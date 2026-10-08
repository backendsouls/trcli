### Edge Cases

- A template is applied to a draft that already has a manuscript: the tool says so and asks
  whether to create a second document, replace, or cancel; it never replaces by default.
- A template asks for information and the command cannot ask (it is run by another
  program): required items not supplied beforehand make it fail, listing them all.
- An answer to a template's question is of the wrong kind or outside its allowed values:
  it is rejected with what is expected, and nothing is created.
- A placeholder refers to information that exists for some drafts and not others (a draft
  with no funders): the template's condition decides; without one, the place is left empty
  and reported.
- A list a section repeats over is empty: the section does not appear, and nothing is left
  behind in its place.
- A value placed in a document contains characters that have a special meaning in the
  document's writing format: they are written so that they appear as themselves.
- A title, name, or abstract contains accents, non-Latin characters, or mathematics: it is
  placed as written.
- The researcher deletes the marks around a managed part: the part becomes ordinary text;
  the next refresh reports that a managed part is missing and offers to put it back.
- Two managed parts of the same kind exist in one document: both are refreshed.
- A managed part's content would be identical after refresh: the file is not rewritten.
- A document is refreshed while it is open in an editor with unsaved changes: the tool
  cannot know; it writes only when the file on disk is unchanged since it was read, and
  otherwise stops and says so.
- A template refers to another template (a thesis that includes the chapter template), and
  that one is missing or refers back to the first: the check reports it; a loop is refused.
- A template tries to place the content of a file from outside the workspace, or to have a
  command carried out: it is refused; a template can only arrange text and workspace
  information.
- A template brought in from someone else asks for information: the questions are shown
  before anything is created, like any other template.
- A provided template changes in a new version of the tool: documents created earlier are
  untouched, and the researcher's copies of it are untouched.
- Two templates at the same level differ only by letter case or accents in their names:
  the second is refused.
- A template's stated limit is zero or negative, or its sections are all optional and none
  exists: the check of the template reports it.
- A document is checked against a template that states no limits: only structure is
  checked.
- A template is deleted while a report definition names it: the definition is reported as
  needing a template when next used, and falls back to nothing by itself.
