# Contract: CLI Conventions

**Plan**: [../plan.md](../plan.md) | **Index**: [README.md](./README.md) | **Output**: [output-and-exit-codes.md](./output-and-exit-codes.md) | **Settings**: [configuration.md](./configuration.md)

The rules every `trcli` command follows. Each specification's `contracts/` directory holds
one file per group of commands; those files state only what is particular to them and rely
on this one for the rest. Changing anything here after release is a breaking change.

## Grammar

```text
trcli [GLOBAL OPTIONS] <noun> <verb> [ARGUMENTS] [OPTIONS]
```

- A **noun** names a kind of record or an area (`ref`, `run`, `report`); a **verb** names
  what to do (`add`, `list`, `start`). A few commands have no verb (`trcli init`,
  `trcli status`, `trcli due`).
- Options that take a list are repeatable (`--tag a --tag b`).
- Every noun and verb has `--help`; `trcli help <noun>` is the same.
- Dates are written `YYYY-MM-DD`. Spans are written `<n>d`, `<n>w`, `<n>m`.

## Referring to a record

`<ref>` in any synopsis is a record **handle** or any unique prefix of one.

- A handle is a type prefix, a hyphen, and a short code: `ref-7k3f`, `run-6p0z`.
- A prefix that matches several records fails with exit code 3 and lists the matches.
- A handle that matches nothing fails with exit code 3 and suggests close ones.
- Where a contract says so, another name is also accepted (a citation key, a step key, a
  definition name).
- Handles never change and are never reused.

## Global options

| Option | Values | Default | Meaning |
|--------|--------|---------|---------|
| `--workspace <dir>` | path | discovered | Workspace to use (also `TRCLI_WORKSPACE`) |
| `--project <ref>` | project | current project | Project to act in (added by `specs/003-research-projects`; not part of the foundation) |
| `--all-projects` | flag | off | Act across the whole workspace (added by `specs/003-research-projects`) |
| `--output <format>` | `human`, `json` | `human` | Output form |
| `--color <when>` | `auto`, `always`, `never` | `auto` | Colored output |
| `-y`, `--yes` | flag | off | Answer yes to confirmations |
| `--no-input` | flag | off on a terminal | Never prompt; fail instead |
| `-q`, `--quiet` | flag | off | Only results and errors |
| `-v`, `--verbose` | flag, repeatable | off | Diagnostic detail on stderr |

## Verbs every record noun has

A contract that says "the shared verbs apply" means these, with the noun's own fields and
filters:

| Verb | Form | Behavior |
|------|------|----------|
| `add` | `<noun> add [fields]` | Creates a record and prints its handle |
| `list` | `<noun> list [filters] [--search <text>] [--tag <tag>]... [--sort <field>] [--desc] [--limit <n>]` | Lists matching records |
| `show` | `<noun> show <ref>` | One record with its notes, tags, and links |
| `edit` | `<noun> edit <ref> [fields]` | Changes only the fields given |
| `rm` | `<noun> rm <ref>` | Lists what refers to the record, asks for confirmation, deletes |
| `tag` | `<noun> tag <ref> <tag>... [--remove]` | Adds or removes tags |
| `note` | `<noun> note <ref> <text>` | Adds a note |

## What every command guarantees

1. **Validation first.** Every supplied value is checked, and all problems are reported
   together, before anything is written or carried out. Exit code 2.
2. **All or nothing.** A command either completes, with its audit entry, or changes nothing.
3. **No hidden prompts.** A command that needs confirmation and cannot ask (no terminal, or
   `--no-input`) fails with exit code 5 unless `--yes` is given. The default answer is No.
4. **Clean streams.** Results go to stdout; errors, warnings, progress, and prompts go to
   stderr.
5. **Stable structured output.** `--output json` prints one document in the shape defined in
   [output-and-exit-codes.md](./output-and-exit-codes.md), for success and for failure.
6. **Stable exit codes.** 0 success · 1 unexpected failure · 2 invalid input · 3 not found
   or ambiguous · 4 workspace problem · 5 confirmation required, refused, or blocked ·
   6 a check failed · 7 an operation failed · 130 interrupted.
7. **Outside a workspace**, every command except `init`, `config --user`, `completions`,
   and `help` fails with exit code 4 and says how to create or locate one.
8. **Never blocked.** Anything that may take long shows progress on a terminal and can be
   interrupted; nothing waits indefinitely for a person.

## Where each noun is specified

| Nouns | Handle prefix | Contract |
|-------|---------------|----------|
| `init, workspace, config, completions` | `—` | [000-foundation](./cli-workspace.md) |
| `link, tag, note` | `—` | [000-foundation](./cli-link.md) |
| `idea`, `inbox`, `topic`, `question`, `hypothesis`, `review-ideas` | `tht`, `ida`, `top`, `rq`, `hyp` | [013-ideas-questions](../../013-ideas-questions/contracts/README.md) |
| `manuscript` (alias: `draft`), `part`, `publication` | `ms`, `ver`, `pub` | [008-manuscripts](../../008-manuscripts/contracts/cli-manuscript.md) |
| `convention`, `method`, `assumption`, `decision`, `lesson`, `checklist`, `handbook`, `applies`, `departure` | `cnv`, `mth`, `asm`, `dec`, `lsn`, `chk`, `dpt` | [014-conventions-methods](../../014-conventions-methods/contracts/README.md) |
| `dataset` | `dat, dv` | [001-research-workspace](../../001-research-workspace/contracts/cli-dataset.md) |
| `env, repro` | `env` | [001-research-workspace](../../001-research-workspace/contracts/cli-env.md) |
| `staff` (alias: `person`), `meeting`, `handover` | `stf`, `mtg`, `hnd` | [012-people](../../012-people/contracts/README.md) |
| `audit, telemetry` | `—` | [000-foundation](./cli-audit.md) |
| `course`, `term`, `programme`, `curriculum`, `plan`, `requirement`, `roadmap` | `crs`, `prg`, `cur`, `pln`, `req`, `rmp` | [011-courses-roadmaps](../../011-courses-roadmaps/contracts/README.md) |
| `backup, restore, export, upgrade` | `—` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-backup.md) |
| `notebook` | `nb` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-notebook.md) |
| `prereg` | `pre` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-prereg.md) |
| `submission, review-comment` | `sub, cmt` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-submission.md) |
| `ethics, dmp` | `eth, dmp` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-ethics.md) |
| `funder, grant` | `fnd, grt` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-grant.md) |
| `status` | `—` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-status.md) |
| `venue`, `event`, `call`, `talk` | `ven`, `evt`, `cfp`, `tlk` | [015-venues](../../015-venues/contracts/README.md) |
| `concept` | `cpt` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-concept.md) |
| `instrument, sample, material` | `ins, smp, mat` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-lab.md) |
| `publish` | `dep` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-publish.md) |
| `sync, type, extension` | `—` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-integration.md) |
| `member, sync` | `mbr` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-collab.md) |
| `due (extended)` | `—` | [002-research-lifecycle](../../002-research-lifecycle/contracts/cli-due-extended.md) |
| `project` | `prj` | [003-research-projects](../../003-research-projects/contracts/cli-project.md) |
| `milestone` | `mil` | [003-research-projects](../../003-research-projects/contracts/cli-milestone.md) |
| `task` | `tsk` | [003-research-projects](../../003-research-projects/contracts/cli-task.md) |
| `due, progress` | `—` | [003-research-projects](../../003-research-projects/contracts/cli-due.md) |
| `todo` | `—` | [003-research-projects](../../003-research-projects/contracts/cli-todo.md) |
| `project-type` | `—` | [003-research-projects](../../003-research-projects/contracts/cli-project-type.md) |
| `experiment` | `exp` | [004-experiments](../../004-experiments/contracts/cli-experiment.md) |
| `step` | `—` | [004-experiments](../../004-experiments/contracts/cli-step.md) |
| `run, sweep` | `run, swp` | [004-experiments](../../004-experiments/contracts/cli-run.md) |
| `param, metric` | `—` | [004-experiments](../../004-experiments/contracts/cli-param.md) |
| `result, figure, table` | `res, fig, tbl` | [004-experiments](../../004-experiments/contracts/cli-result.md) |
| `software` | `sw` | [004-experiments](../../004-experiments/contracts/cli-software.md) |
| `ref (alias: paper)` | `ref` | [005-literature](../../005-literature/contracts/cli-ref.md) |
| `cite` | `cit` | [005-literature](../../005-literature/contracts/cli-cite.md) |
| `bib` | `bib` | [005-literature](../../005-literature/contracts/cli-bib.md) |
| `read, annotate, summary` | `ann` | [005-literature](../../005-literature/contracts/cli-read.md) |
| `relation` | `—` | [005-literature](../../005-literature/contracts/cli-relation.md) |
| `review` | `rev` | [005-literature](../../005-literature/contracts/cli-review.md) |
| `extract, theme` | `—` | [005-literature](../../005-literature/contracts/cli-synthesis.md) |
| `report` | `rpt` | [006-reports](../../006-reports/contracts/cli-report.md) |
| `model`, `scenario`, `sim` | `mdl`, `scn`, `bat` | [009-simulations](../../009-simulations/contracts/README.md) |
| `integration`, `sync`, `file`, `backup remote` | `con`, `cnf` | [010-integrations](../../010-integrations/contracts/README.md) |
| `template` | `—` | [007-templates](../../007-templates/contracts/cli-template.md) |
| `doc` | `doc` | [007-templates](../../007-templates/contracts/cli-doc.md) |

## How contracts are verified

Each example in a contract file is the expected behavior of the built tool. The acceptance
scenarios of the owning specification are run against the compiled `trcli`, and the usage
guide of each noun repeats these examples as executable checks, so a contract and the tool
cannot drift apart unnoticed.
