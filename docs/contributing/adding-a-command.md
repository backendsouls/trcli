# Adding a command

A worked example, end to end, using a command that exists: how `trcli tag list
[--kind <kind>]` is built. Follow the same steps for your command. Read
[`CONTRIBUTING.md`](../../CONTRIBUTING.md) first for the layers and the checks.

The order is the order of work: scenario, use case with its tests, command line, rendering,
help, guide.

## 1. The scenario, failing

In `tests/features/<spec>/…​.feature`, tagged with the acceptance scenario it automates:

```gherkin
@US2-07
Scenario: Records of any kind are found by tag
  Given a workspace named "Lab"
  And I ran "trcli specimen add --title 'First'" and remember the record as "first"
  And I ran "trcli specimen tag <first> field-work"
  When I run "trcli tag list"
  Then the exit code is 0
  And stdout contains "field-work  1"
```

and the rejection, tagged `@invalid` — `tests/invalid_input_gate.rs` fails without one for
every option that takes a value:

```gherkin
@US3-04 @invalid
Scenario: An unknown kind is answered with the kinds that exist
  When I run "trcli tag list --kind planet"
  Then the exit code is 2
  And stderr contains "specimen, sample-note"
```

Run `cargo test -p trcli-cli --test bdd --example sample_kinds --features test-clock` and see them fail.

## 2. The use case, with its test first

A use case is a function in `trcli-application` that takes the ports it needs as type
parameters and returns a **view model** or a `Problem`
(`crates/trcli-application/src/records/tag.rs`):

```rust
/// Every tag in use.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct TagList {
    /// The tags, in alphabetical order.
    pub items: Vec<TagCount>,
    /// How many tags there are.
    pub total: u64,
}

/// Lists every tag with the number of records carrying it, of one kind when given.
pub async fn list_tags<U: TagStore>(unit: &U, kinds: &KindRegistry, kind: Option<&str>) -> Result<TagList, Problem> {
    // An unknown kind is invalid input, reported with the kinds that exist.
    …
    let counts = unit.tag_counts(kind).await?;
    …
}
```

- The type parameter says exactly which port it uses: `TagStore`, nothing more.
- Input is checked **before** anything else, with a `Checker`
  (`crates/trcli-application/src/validation.rs`), so that every problem is reported at
  once and under the name the researcher typed (`--kind`).
- The view model derives `Serialize`: that is the structured form. Lists are
  `{ items, total }`.
- Its test is written first, in `crates/trcli-application/tests/use_cases/<module>.rs`,
  with the in-memory doubles of the `trcli-testing` crate and its `block_on`. A use case
  is tested through what it makes public, as its callers see it. (Pure logic that needs no
  double — a parser, a rule — is tested inline, beside the code.)

If your use case **changes** something, it takes `&mut U` with `AuditLog` among its ports
and records an audit entry for the change; the handler commits.

## 3. The command line

In `crates/trcli-cli/src/args/<group>.rs`, with a description, help for every option, and
an example — `tests/help_examples.rs` fails without them:

```rust
/// The verbs of `trcli tag`.
#[derive(Clone, Debug, Subcommand)]
pub enum TagCommand {
    /// List every tag with the number of records carrying it
    #[command(after_long_help = "Example:\n  trcli tag list\n  trcli tag list --kind reference")]
    List(TagListArgs),
}

/// The options of `trcli tag list`.
#[derive(Clone, Debug, Args)]
pub struct TagListArgs {
    /// Only count records of this kind
    #[arg(long, value_name = "KIND")]
    pub kind: Option<String>,
}
```

A new **group** of commands is added to `Commands` in `args/mod.rs`, and its long help ends
with `Guide: docs/usage/<file>.md`.

Take values as `Option<String>` and let the application check them: clap checks the
syntax, the command constructor checks the values, all together.

## 4. The handler

In `crates/trcli-cli/src/commands/<group>.rs`. Four things: validated command, unit of
work, use case, commit (a reading command has nothing to commit):

```rust
/// `trcli tag list [--kind <kind>]`.
pub async fn tags(session: &mut Session, command: &TagCommand) -> Result<Reply, Problem> {
    let TagCommand::List(arguments) = command;
    let storage = session.storage(Access::Read).await?;
    let unit = storage.read().await?;
    Ok(Reply::new(list_tags(&unit, &session.registries.kinds, arguments.kind.as_deref()).await?))
}
```

`session.storage(Access::Read | Access::Write)` finds the workspace, opens it, and refuses
one that is too new, damaged, or — for writing — in need of an upgrade. A changing command
uses `storage.begin()` and ends with `finish(session, unit).await?`.

Add the arm to the `match` in `commands/mod.rs`.

## 5. The form for people

Implement `Render` for the view model in `crates/trcli-cli/src/render/views/`:

```rust
impl Render for TagList {
    fn render(&self, out: &mut Human) {
        if self.items.is_empty() {
            out.notice("No tags are in use.");
            return;
        }
        let rows: Vec<Vec<Cell>> =
            self.items.iter().map(|item| vec![Cell::plain(&item.tag), Cell::plain(item.records.to_string())]).collect();
        out.table(&["TAG", "RECORDS"], &rows, 0);
    }
}
```

A view only says *what* to show. Colour, symbols, the terminal's width, and where remarks
go (standard error) are `Human`'s business. Show everything the structured form holds:
`tests/forms_agree.rs` compares the two.

## 6. The usage guide

Add the command to the table of its guide in `docs/usage/`, and an example:

````markdown
```console
$ trcli tag list
```
````

then fill in what it prints and read the result:

```sh
TRCLI_BLESS_USAGE=1 cargo test -p trcli-cli --test usage --example sample_kinds --features test-clock
git diff docs/usage
```

## 7. Everything green

```sh
cargo fmt --all
cargo clippy --workspace --all-targets --features trcli-cli/test-clock -- -D warnings
cargo test --workspace --features trcli-cli/test-clock
```
