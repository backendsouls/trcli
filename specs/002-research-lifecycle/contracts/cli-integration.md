# Contract: `trcli sync, type, extension`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 16; FR-060 to FR-063

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

## Synopsis

```text
trcli sync bib <draft-ref|bibliography-ref> --file <path> [--stop]
trcli sync library <file> [--once]
trcli type add <name> --field <name>:<kind>[:required]...
trcli type list | show <name> | edit <name> ... | rm <name>
trcli extension add <path> | list | on <name> | off <name> | rm <name>
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `sync bib` | Keeps a bibliography file in step with a draft's or bibliography's citations | 2 when the file is outside the workspace and not writable |
| `sync library` | Brings in new and changed entries from a reference manager's export without creating duplicates | — |
| `type add` | Defines a custom record type; its records get the shared verbs | 2 when the name is used by a built-in or existing type |
| `type edit` | Changes fields; shows the effect on existing records first | 5 `confirmation_required` |
| `extension add` | Shows what the extension adds and can access; it is off until turned on | 5 `confirmation_required` |

## Values

| Value | Rule |
|-------|------|
| `<kind>` | `text`, `number`, `date`, `yes-no`, `choice(a\|b\|c)`, `ref(<record-kind>)` |

## Example

```console
$ trcli type add interview --field participant:text:required --field held:date --field consent:yes-no:required
Defined record type "interview" (3 fields). Use it with: trcli interview add ...
```
