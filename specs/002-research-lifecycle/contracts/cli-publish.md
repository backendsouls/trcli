# Contract: `trcli publish`

**Spec**: [Research Lifecycle Extensions](../spec.md) — User Story 15; FR-057 to FR-059

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `dep`

## Synopsis

```text
trcli publish check <dataset|software|package-ref>
trcli publish bundle <ref> --to <file>
trcli publish record <ref> --identifier <text> --archive <text> --date <date>
trcli publish cite <ref> [--style <style>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `publish check` | Lists what is missing for deposit (license, description, creators…) | 6 `check_failed` when anything is missing |
| `publish bundle` | Writes a deposit bundle with the output's description in a widely accepted form | 6 when the check fails; 5 when the output is sensitive and `--yes` is not given |
| `publish record` | Records the identifier, archive, and date once deposited | 2 on a malformed identifier |
| `publish cite` | The citation others should use for the published output | 2 when not yet published |

## Values

| Value | Rule |
|-------|------|
| note | the tool prepares the deposit; uploading is done by the researcher |

## Example

```console
$ trcli publish check dat-3n8x
dat-3n8x "Train set" is not ready for deposit: 2 items missing
  license       not set
  creators      no staff linked
```
