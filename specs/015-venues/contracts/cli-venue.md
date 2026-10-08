# Contract: `trcli venue`

**Spec**: [Venues — Places to Publish](../spec.md) — User Stories 1 and 4; FR-001 to FR-011, FR-032 to FR-036

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `ven` — a venue may also be named by its acronym when that is unambiguous

## Synopsis

```text
trcli venue add --name <text> --kind <kind> [--acronym <text>] [--publisher <text>] [--url <address>]
                [--issn <id>]... [--language <code>]... [--topic <text>]...
                [--review <single-blind|double-blind|open|none>] [--reviews-published]
                [--open-access <full|hybrid|none>] [--preprints <allowed|restricted|forbidden>]
                [--interest <target|watching|neutral|avoid>] [--avoid-because <text>]
trcli venue cost <ref> --kind <publication|registration|other> --amount <n> --currency <code> [--noted <date>] [--note <text>]
trcli venue requires <ref> [--takes <manuscript-kind>]... [--max-words <n>] [--max-pages <n>]
                           [--anonymous | --not-anonymous] [--template <template-name>] [--note <text>]
trcli venue states <ref> [--acceptance-rate <percent>] [--decision-time <text>] [--noted <date>]
trcli venue rank <ref> --scheme <text> --value <text> [--year <year>]
trcli venue rank list [<ref>] | schemes
trcli venue interest <ref> <target|watching|neutral|avoid> [--because <text>]
trcli venue renamed <ref> --to <text> [--on <date>]
trcli venue succeeded <ref> --by <ref>
trcli venue appears-in <ref> <venue-ref>
trcli venue retire <ref> [--because <text>]

trcli venue history <ref>
trcli venue impression <ref> <text> [--reviews <1-5>] [--speed <1-5>] [--handling <1-5>]
trcli venue waiting

trcli venue export [filters] --to <file> [--with-impressions]
trcli venue import <file> [--on-duplicate <ask|skip|update>] [--dry-run]
```

The shared verbs `list`, `show`, `edit`, `rm`, `tag`, and `note` apply and are not repeated.

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `venue add` | Records a venue. Reports a likely duplicate (same name, acronym, or identifier) | 2 on a missing name, unknown kind, or malformed address or identifier — all reported together; 2 for `--interest avoid` without `--avoid-because`; 5 on a duplicate when it cannot ask |
| `venue cost` | Records a cost with its currency and the date it was noted. Never converted or added across currencies; marked when noted over a year ago | 2 on a negative amount or unknown currency |
| `venue requires` | The venue's general requirements for submissions, used when a manuscript is aimed at it | 2 on limits that are not positive |
| `venue states` | What the venue itself says about its acceptance rate and time to decision; always shown apart from the researcher's own figures | 2 on a rate outside 0–100 |
| `venue rank` | Records the venue's standing in a ranking scheme for a year; earlier entries are kept | 2 without `--scheme` or `--value` |
| `venue interest` | Sets the researcher's view of the venue | 2 for `avoid` without `--because` |
| `venue renamed` / `succeeded` / `appears-in` | Records a change of name, a successor, or where the venue's papers are published; earlier names still find it | 3 when the other venue is not found |
| `venue retire` | Marks a venue as no longer active; it is kept and left out of suggestions | — |
| `venue show` | Shared verb. Profile, next deadline, manuscripts aimed at it, events, and a summary of the researcher's experience | 3 when not found; lists matches when an acronym is ambiguous |
| `venue history` | The researcher's own submissions there: manuscript, dates, decision, review rounds, and time to first decision; then the summary — submissions, acceptances, rejections, shortest, typical, and longest time — saying on how many submissions it rests | 0 with a message when there is none |
| `venue impression` | Adds a dated, private impression with optional ratings | 2 on a rating outside 1–5 |
| `venue waiting` | Submissions awaiting a decision, with the time waited beside the researcher's typical time at that venue; marked when longer than their longest | — |
| `venue export` | Writes venues' profiles to a file; private notes, impressions, and the researcher's history are left out unless `--with-impressions` | 5 when the file exists and `--yes` is not given |
| `venue import` | Brings venues in; reports duplicates and shows differences for the researcher to choose | 2 when the file is not a venue list; nothing is added |
| `venue rm` | Shared verb | 5 `blocked_by_dependents` when manuscripts, events, calls, submissions, or presentations refer to it — retire it instead |

## Values

| Value | Rule |
|-------|------|
| `<kind>` | `journal`, `conference`, `workshop`, `symposium`, `book-series`, `preprint-server`, `other` |
| `--scheme`, `--value` | free text: the tool knows no ranking scheme and does not know which values are better |
| `--currency` | three-letter currency code |
| `--noted` | the date a figure was noted; defaults to today |
| list filters | `--kind`, `--topic`, `--scheme <text> [--value <text>]`, `--interest`, `--language`, `--open-access`, `--active` / `--retired`, `--search`; `--sort name\|deadline\|my-time\|my-record\|scheme:<text>` |
| nothing is looked up | the tool fetches no calls, rankings, fees, or rates and ships no list of venues |

## Example

```console
$ trcli venue show jspeech
ven-2m7t  Journal of Speech Research (JSpeech) · journal · target
  Topics        speech recognition, annotation, low-resource languages
  Review        double-blind · reviews not published
  Open access   hybrid · publication charge 1,800 EUR (noted 2026-03-04)
  Standing      Qualis A2 (2024) · quartile Q1 (2025)
  Next          open call — accepts submissions at any time
  Aimed here    ms-3e8k "Study B" (drafting)
  My record     2 submissions · 1 accepted · 1 rejected
  My time       first decision in 74 and 131 days (typical 103, on 2 submissions)
  Venue states  acceptance 22% · decision in "about 8 weeks" (noted 2026-03-04)
```
