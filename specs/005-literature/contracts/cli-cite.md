# Contract: `trcli cite`

**Spec**: [Literature, References, and Bibliography](../spec.md) — User Story 3; FR-021 to FR-024

**Conventions**: [grammar, global options, shared verbs](../../000-foundation/contracts/cli-conventions.md) · [output and exit codes](../../000-foundation/contracts/output-and-exit-codes.md)

**Handle prefix**: `cit`

## Synopsis

```text
trcli cite add <ref> [--key <key>]
trcli cite key <citation-ref> <new-key>
trcli cite show <ref|key>... [--style <style>] [--in-text]
trcli cite list [--search <text>]
trcli cite rm <citation-ref>
trcli cite styles [add <file.csl>]
```

## Commands

| Command | Behavior | Fails with (exit code) |
|---------|----------|------------------------|
| `cite add` | Saves a citation with a unique key, proposed from author, year, and title | 0 with a notice when the key had to be made unique |
| `cite key` | Changes a key; lists drafts and bibliographies that use it first | 5 `confirmation_required`; 2 when the key is taken or malformed |
| `cite show` | The reference-list entry, or with `--in-text` the in-text form, in a style | 0 with a warning naming any detail the style requires that is missing |
| `cite styles` | Lists available styles; `add` registers a style definition file | 2 when the file is not a valid style |

## Values

| Value | Rule |
|-------|------|
| `<key>` | 1–64 characters of `A-Z a-z 0-9 _ : -`; does not change when the reference changes |
| `--style` | a built-in style or one added; default from `citation.style` |

## Example

```console
$ trcli cite show vaswani2017attention --style apa
Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A. N., Kaiser, Ł., &
Polosukhin, I. (2017). Attention is all you need. Advances in Neural Information Processing
Systems, 30.
```
