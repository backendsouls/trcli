# Records: what every kind shares; `trcli link`, `trcli tag`

## Purpose

Whatever you keep in TRCLI — a reference, an experiment, a task — the same handful of
actions works the same way: add one, list them, look at one, change it, remove it; give it
tags and notes; link it to any other record. Learn one kind of record and you know them
all.

The foundation itself has no kind of record; kinds arrive with the features that own them
(references with the literature feature, tasks with projects, and so on). The examples
below use `specimen` and `sample-note`, two sample kinds that are not part of `trcli`:
they live in an example program that is the same tool with those two kinds added
(`cargo run -p trcli-cli --example sample_kinds -- <command>`), to show and to check
exactly this shared behaviour.

## Commands

Every kind of record has these verbs, with the same words, order, and options:

| Verb | Form | What it does |
|------|------|--------------|
| `add` | `trcli <noun> add [fields]` | Creates a record and prints its short name |
| `list` | `trcli <noun> list [--search <text>] [--tag <tag>]... [--sort <field>] [--desc] [--limit <n>]` | Lists matching records |
| `show` | `trcli <noun> show <ref>` | One record with its tags, notes, and links |
| `edit` | `trcli <noun> edit <ref> [fields]` | Changes only the fields you name |
| `rm` | `trcli <noun> rm <ref>` | Lists what refers to the record, asks, deletes |
| `tag` | `trcli <noun> tag <ref> <tag>... [--remove]` | Adds or removes tags |
| `note` | `trcli <noun> note <ref> <text>` | Adds a dated note |

And across kinds:

| Command | What it does |
|---------|--------------|
| `trcli link add <ref> <ref> [--relation <text>]` | Links two records of any kind; the link is shown from both |
| `trcli link rm <ref> <ref> [--relation <text>]` | Removes a link |
| `trcli link list <ref>` | The links of a record |
| `trcli tag list [--kind <kind>]` | Every tag with the number of records carrying it |

### Short names

A record's **short name** is its kind's prefix, a hyphen, and a short code: `spc-7k3f`.
Wherever a command takes `<ref>`, type the short name or any beginning of it that matches
exactly one record; letter case does not matter. Short names never change and are never
given to another record, even after the record is deleted.

| Value | Rule |
|-------|------|
| `<tag>` | 1 to 50 characters of `a-z 0-9 - _`; upper case is lowered; spaces are not allowed |
| `<text>` of a note | 1 to 20,000 characters |
| `--relation` | 1 to 50 characters; default `related` |
| `--sort` | the kind's name for its main text (`title`), `created`, `updated`, or `handle` |
| `--limit` | 1 to 1000; default: the `output.page_size` setting |
| `--search` | words in the record's main text; case and accents are ignored |

## Examples

Add records of two kinds; each gets a short name that says what it is:

```console
$ trcli init --name "Field work"
Created workspace "Field work" in [..]/work/.trcli
$ trcli specimen add --title "Soil sample 14"
Saved specimen spc-z7kj "Soil sample 14"
$ trcli specimen add --title "Água da chuva"
Saved specimen spc-gmff "Água da chuva"
$ trcli sample-note add --body "Collected in the rain"
Saved sample-note smp-21bc "Collected in the rain"
```

List, search without regard to accents, sort, and limit — the same for every kind:

```console
$ trcli specimen list
HANDLE    TITLE           TAGS  UPDATED
spc-gmff  Água da chuva         2026-10-08 14:00
spc-z7kj  Soil sample 14        2026-10-08 14:00
$ trcli specimen list --search agua
HANDLE    TITLE          TAGS  UPDATED
spc-gmff  Água da chuva        2026-10-08 14:00
$ trcli specimen list --sort title --desc --limit 1
HANDLE    TITLE           TAGS  UPDATED
spc-z7kj  Soil sample 14        2026-10-08 14:00
Showing 1 of 2. Use --limit <n> to see more.
```

Tag, note, and link; everything is shown with the record:

```console
$ trcli specimen tag spc-z Field-Work
Tagged spc-z7kj "Soil sample 14": field-work (now: field-work)
$ trcli specimen note spc-z "Dried overnight"
Added a note to spc-z7kj "Soil sample 14" (2026-10-08 14:00)
$ trcli link add spc-z smp --relation "same site"
Linked spc-z7kj "Soil sample 14" ⟷ smp-21bc "Collected in the rain" (same site)
$ trcli specimen show spc-z
spc-z7kj "Soil sample 14"
  Kind     specimen
  Id       01920000-0000-7000-8f9e-72ebf533e025
  Title    Soil sample 14
  Created  2026-10-08 14:00 UTC
  Updated  2026-10-08 14:00 UTC
  Tags     field-work
Notes
  2026-10-08 14:00  Dried overnight
Links
  ⟷ smp-21bc "Collected in the rain" (same site, sample-note)
$ trcli tag list
TAG         RECORDS
field-work  1
$ trcli link list smp
1 link of sample-note smp-21bc "Collected in the rain":
  ⟷ spc-z7kj "Soil sample 14" (same site, specimen)
```

Delete: what refers to the record is listed first, and nothing is left pointing at it:

```console
$ trcli specimen rm spc-z --yes
Deleted specimen spc-z7kj "Soil sample 14"
  removed: link to smp-21bc "Collected in the rain" (same site)
  removed: 1 tag: field-work
  removed: 1 note
$ trcli link list smp
smp-21bc "Collected in the rain" has no links.
```

## When it fails

A beginning that matches several records lists them and changes nothing (exit code 3):

```console
$ trcli specimen add --title "Soil sample 15"
Saved specimen spc-jvt3 "Soil sample 15"
$ trcli specimen show spc
error: `spc` matches 2 records
  spc-gmff "Água da chuva"
  spc-jvt3 "Soil sample 15"
Nothing was changed.
Next: type more of the short name
[exit 3]
```

A short name that matches nothing (exit code 3), an invalid tag (exit code 2), and a
deletion nobody confirmed (exit code 5):

```console
$ trcli specimen show spc-zzzz
error: no record matches `spc-zzzz`
Nothing was changed.
[exit 3]
$ trcli specimen tag spc-g "two words"
error: 1 value is invalid
  <tag> "two words"  contains characters a tag name may not have; expected 1 to 50 characters of a-z, 0-9, '-' and '_' (no spaces) (for example: field-work)
Nothing was changed.
[exit 2]
$ trcli specimen rm spc-g
error: Deleting specimen spc-gmff "Água da chuva" needs confirmation, and nobody can be asked
Nothing was changed.
Next: run the command again with `--yes` to confirm beforehand
[exit 5]
```

| What happened | Exit code | What the tool says and what to do |
|---------------|-----------|-----------------------------------|
| What you typed matches no record | 3 | Close short names, if there are any |
| What you typed matches several records | 3 | The matches; type more of the short name |
| An invalid tag, an empty note, a relation that is too long | 2 | The rule; nothing is changed |
| A record linked to itself, or the same link made twice | 2 | Nothing is changed |
| A link that does not exist is removed | 3 | How to list the record's links |
| Deletion needs confirmation and nobody can be asked | 5 | What would be removed; run again with `--yes` |
| A feature forbids the deletion | 5 | What stands in the way and the alternative the feature offers |
