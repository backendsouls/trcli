# Contract: Output, Errors, and Exit Codes

**Plan**: [../plan.md](../plan.md) | **Commands**: [cli-conventions.md](./cli-conventions.md)

## Streams (FR-031)

| Stream | Carries |
|--------|---------|
| stdout | The result of the command, and nothing else |
| stderr | Errors, warnings, progress, prompts, and `--verbose` diagnostics |

A script can always pipe stdout and trust that it holds only the result.

## Exit codes

| Code | Meaning | Examples |
|------|---------|----------|
| 0 | Success | Includes a run that paused at a manual step |
| 1 | Unexpected failure | Internal error, I/O error, damaged database |
| 2 | Invalid input or usage | Unknown option, missing required value, value fails validation |
| 3 | Not found or ambiguous | Unknown handle; prefix matches several records |
| 4 | Workspace problem | No workspace found; workspace already exists; schema newer than the tool |
| 5 | Confirmation required or refused | Destructive action without `--yes` and no terminal; user answered no; blocked by dependents |
| 6 | Check failed | `repro check` found differences; `audit verify` found a break; `figure verify` / `dataset verify` found a change |
| 7 | Operation failed | An automated step failed; an online lookup could not be completed |
| 130 | Interrupted | Ctrl-C; a run in progress is recorded as `interrupted` first |

Codes are stable. New codes may be added; existing ones do not change meaning.

## Human output (`--output human`, the default)

- Lists are tables sized to the terminal; long text is truncated with `…` in lists and
  shown in full by `show`.
- Handles, statuses, and dates use the theme (see [configuration.md](./configuration.md)).
  Color is never the only carrier of meaning: a status is always also written as a word.
- Color is used only when the stream is a terminal and is off when `NO_COLOR` is set, when
  `--color never` is given, or when output is piped. `--color always` forces it.
- An empty list prints a one-line message on stderr and nothing on stdout; exit code 0.
- Progress indicators appear only on a terminal, only on stderr, and are removed when done.

## Structured output (`--output json`)

One JSON document on stdout, UTF-8, ending with a newline, never colored.

### Success

```json
{
  "ok": true,
  "data": { },
  "warnings": [
    { "code": "duplicate_suspected", "message": "…", "field": null }
  ]
}
```

- `data` is an object for single-record commands and for summaries; for list commands it is
  `{ "items": [ … ], "total": 123 }`.
- Field names are `snake_case` and match the data model. Absent optional values are `null`,
  not omitted. Dates are `YYYY-MM-DD`; timestamps are RFC 3339 in UTC.
- Every record includes `handle`, `id`, `kind`, `created_at`, `updated_at`.

### Failure

Also on **stdout** when `--output json` is set, so a script reads one place; the exit code
tells success from failure. A plain-language line is still written to stderr.

```json
{
  "ok": false,
  "error": {
    "code": "validation_failed",
    "message": "2 values are invalid",
    "details": [
      { "field": "--year", "value": "20244", "problem": "is out of range", "expected": "a whole number from 1000 to 2027", "example": "1000", "choices": [] },
      { "field": "--title", "value": "", "problem": "must not be empty", "expected": "1 to 500 characters", "example": null, "choices": [] }
    ],
    "changed": false,
    "next_step": null
  }
}
```

- `field` is the value's name as the researcher typed it (`--year`, `<tag>`); `value` is
  `null` when the value is secret.
- `details` is `null`, a list of invalid values (for `validation_failed` and
  `settings_invalid`), or a list of text items (matching short names, dependents,
  differences found).
- `changed` says whether anything had been changed when the problem was met; `next_step`
  names the obvious next step, or is `null` (FR-034).

### Error codes

| `error.code` | Exit code | When |
|--------------|-----------|------|
| `validation_failed` | 2 | One or more values invalid; `details` lists every one (FR-023) |
| `usage` | 2 | Malformed command line |
| `settings_invalid` | 2 | A settings file or session variable holds an unknown key or an invalid value; `details` names the source and the key (FR-043) |
| `not_found` | 3 | No record matches; `details` may suggest close handles |
| `ambiguous_reference` | 3 | Prefix matches several; `details` lists them |
| `no_workspace` | 4 | No workspace found |
| `workspace_exists` | 4 | `init` where one exists |
| `workspace_too_new` | 4 | Database schema newer than this version |
| `workspace_needs_upgrade` | 4 | Older format: reading works, a command that would change something asks for `trcli workspace upgrade` first |
| `workspace_damaged` | 4 | The stored data cannot be opened or fails its check; `details` says what and where |
| `workspace_busy` | 4 | Another command is changing the workspace and did not finish within the wait |
| `confirmation_required` | 5 | Needs `--yes`; `details` lists what would be affected |
| `declined` | 5 | The researcher was asked and did not answer yes |
| `blocked_by_dependents` | 5 | Deletion not allowed; `details` lists dependents |
| `check_failed` | 6 | Verification or reproducibility differences; `details` lists them |
| `operation_failed` | 7 | Something outside the tool could not be done: a file that cannot be written, a read-only disk |
| `step_failed` | 7 | Automated step failed; includes step key and exit code (registered by `specs/004-experiments`) |
| `lookup_unavailable` | 7 | `details.cause`: `no_connection`, `catalogue_unavailable`, `unknown_identifier`, `disabled` (registered by `specs/005-literature`) |
| `interrupted` | 130 | Cancelled by the user |
| `internal` | 1 | Anything else |

## Validation messages (FR-022, constitution principle V)

Each problem states **which value**, **what is wrong**, and **what is expected**, with an
example when one helps:

```text
error: 2 values are invalid
  --year "20244"  is out of range; expected a whole number from 1000 to 2027 (for example: 1000)
  --title ""      must not be empty; expected 1 to 500 characters
Nothing was changed.
```

A problem with an obvious next step ends with a line `Next: …`.

## Confirmations (FR-017, FR-035)

A destructive command first prints what will be affected, then asks:

```text
Deleting paper pap-7k3f "Attention Is All You Need" will also remove:
  1 citation    cit-9d2a (vaswani2017attention)
  2 candidates  in reviews rev-41bc, rev-77e0
  3 links
Delete? [y/N]
```

- `--yes` skips the question. Without a terminal, or with `--no-input`, and without
  `--yes`, the command fails with `confirmation_required` and changes nothing.
- The default answer is always No.

## Import reports (FR-020; for the features that import)

```text
Imported 987 of 1000 entries (11 invalid, 2 duplicates skipped)
  entry 14  (key: smith2020)   year "20x0" is not a year
  entry 203 (no key)           title is missing
  …
```

In JSON: `data.imported`, `data.invalid` (list with `index`, `key`, `problems`),
`data.duplicates`. Exit code 0 when at least one entry was imported or the file was empty;
2 when every entry was invalid.
