# Contract: `trcli doc`

**Spec**: [Templates](../spec.md) — User Stories 1, 2, and 6; FR-006 to FR-023, FR-045 to FR-048

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `doc` — a document may also be named by the path of any of its files

## Synopsis

```text
trcli doc new <template> --to <dir|file> [--draft <ref> | --unattached] [--set <question>=<value>]...
                         [--replace] [--dry-run]
trcli doc list [--draft <ref>] [--template <name>] [--kind <kind>]
trcli doc show <doc>
trcli doc status <doc>
trcli doc refresh <doc> [--dry-run] [--part <name>]... [--on-edited <ask|keep|overwrite>]
trcli doc release <doc> --part <name>
trcli doc restore <doc> --part <name>
trcli doc clean <doc> [--guidance] [--dry-run]
trcli doc check <doc> [--against <template>]
trcli doc changes <doc>
trcli doc forget <doc>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `doc new` | Applies a template: creates the files, fills placeholders, asks the template's questions (answers given with `--set` are not asked). For a paper or thesis template, records the document on the draft | 2 on invalid or missing required answers, all listed, nothing created; 5 when files exist and `--replace` is not given, listing them; 5 when the draft already has a manuscript and it cannot ask; 6 when the template fails its check; 3 when the template or draft is not found |
| `doc new --dry-run` | Lists the files and sections that would be created; writes nothing | as above |
| `doc list` | Documents created from templates, with template, version, draft, and location | — |
| `doc show` | The document's files, template and version, managed parts, and answers given | 3 when not found |
| `doc status` | Says whether the document is up to date, or lists managed parts that are out of date, edited by hand, missing, or unresolved. Writes nothing | 6 `check_failed` when anything is listed |
| `doc refresh` | Brings every managed part up to date; nothing outside managed parts is touched; files with no change are not rewritten | 5 when a part was edited by hand and it cannot ask; 1 when a file changed on disk since it was read, writing nothing; 2 when the file has no managed parts |
| `doc refresh --dry-run` | Shows each part that would change, old and new | 6 when anything would change |
| `doc release` | Turns a managed part into ordinary text, never refreshed again | 3 when the part is not found |
| `doc restore` | Puts back a managed part whose marks were removed | — |
| `doc clean --guidance` | Removes the template's guidance from the document | 5 `confirmation_required` |
| `doc check` | Against its own template, or with `--against` another of the same kind: missing required sections, order, sections with only guidance, remaining guidance, unresolved placeholders, parts out of date, each stated limit with the document's count, places that reveal the authors when the template forbids it. Additional sections are listed and are not problems. Writes nothing | 6 `check_failed` when there are problems; 2 when a file cannot be read as text or the kinds differ |
| `doc changes` | How the template's structure has changed since the version the document was created from | 0 with a message when nothing changed |
| `doc forget` | Stops tracking a document; its files are left as they are | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `<doc>` | a `doc-…` handle, or the path of any file of the document; a moved or renamed file is still recognized |
| `--to` | a directory for templates of several files, a file otherwise; must be inside the workspace or the directory the command is run from |
| `--set` | `<question>=<value>`; the question must be declared by the template and the value must fit its kind |
| `--on-edited` | what to do with a managed part edited by hand: `ask` (default on a terminal), `keep` (default otherwise), `overwrite` |
| a result that a newer run replaced | refreshed parts keep the reported value and flag it; resolve with `trcli draft refresh` |
| unresolved placeholder | left visibly marked in the document and listed by `doc status` and `doc check` |

## Example

```console
$ trcli doc new article --draft ms-5t1q --to paper/ --set keywords="transformers, attention"
Running title [Our Paper]:
Created doc-2v8n from article (workspace, version 3) for ms-5t1q "Our Paper"
  paper/main.txt         title, authors, abstract, 6 sections
  paper/references.bib   12 references
  4 managed parts: authors, results, figures, references

$ trcli doc status paper/main.txt
doc-2v8n is out of date: 2 managed parts
  authors   1 author added (Silva, Ana)
  results   accuracy 0.89 → 0.91 (res-8u2m, run-6p0z)
$ echo $?
6

$ trcli doc refresh paper/main.txt
Updated 2 managed parts in paper/main.txt. Nothing else was changed.

$ trcli doc check paper/main.txt
doc-2v8n against article (version 3): 2 problems
  section "Limitations"   required, contains only guidance
  abstract                 284 words, limit 250
  additional sections: "Ethical considerations"
$ echo $?
6
```
