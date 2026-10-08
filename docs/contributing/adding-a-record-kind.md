# Adding a kind of record

A kind of record — a reference, a task, an experiment — declares what is particular to it
and receives everything records have in common: a short name, `list`, `show`, `rm`, `tag`,
`note`, links, guarded deletion, input checking, both output forms, its count in
`workspace show`, and entries in the audit trail. The contract is
[`specs/000-foundation/contracts/feature-contract.md`](../../specs/000-foundation/contracts/feature-contract.md).

The reference is the sample kind `specimen`. It lives in an example,
`crates/trcli-cli/examples/sample_kinds/`, which is the whole tool with two sample kinds
added from outside the crates — the proof that the contract is enough, since no file of
the foundation names them. Try it:

```sh
cargo run -p trcli-cli --example sample_kinds -- specimen add --title "Soil sample 14"
```

The example is laid out as a feature is, one part per layer:

| Part of the example (`sample/…`) | What it holds | Where yours goes |
|----------------------------------|---------------|------------------|
| `application/specimen.rs` | descriptor, port for its own rows, behaviour, `add` and `edit` use cases | `crates/trcli-application/src/<feature>/` |
| `storage/mod.rs`, `storage/specimen.rs` | its table, and its port implemented on the unit of work | `crates/trcli-infra-sqlite/src/<feature>/`, with a migration |
| `commands/specimen.rs` | its noun, its own `add` and `edit`, and one call for the shared verbs | `crates/trcli-cli/src/args/` and `commands/` |
| `mod.rs` | the `Extension` that registers the kinds and adds the commands | the same trait, implemented for your feature and composed in `main.rs` |

A feature that ships with TRCLI is built **into** the crates, as a module of the same name
in each layer; the example stays outside only because no release may contain it.

Work in the order of [`CONTRIBUTING.md`](../../CONTRIBUTING.md): scenarios first.

## 1. The descriptor

What the foundation needs to know. The name is also the command's noun; the prefix starts
the kind's short names and may never change after a release.

```rust
/// What the foundation needs to know about specimens.
pub fn descriptor() -> RecordKindDescriptor {
    RecordKindDescriptor::new("specimen", "spc", "Sample records with a title, used to try the tool out.", "title")
}
```

The last argument is what the kind calls the text its records are named and found by; it
becomes a value of `--sort` and the heading of that column. Names and prefixes must be
unique among kinds; the table in `contracts/cli-conventions.md` lists those already taken.

## 2. A port for the kind's own rows

Small, and named for what the use cases need:

```rust
/// The rows specimens keep for themselves.
pub trait SpecimenStore {
    /// Stores a new specimen's title.
    async fn insert_specimen(&mut self, id: RecordId, title: &Title) -> Result<(), StoreError>;
    …
}
```

Implement it twice: for the in-memory `FakeUnit` of the `trcli-testing` crate, for your
tests, and for the SQLite unit of work in the storage crate. If the port has rules of its
own, write a contract suite both must pass, as the foundation's ports have in
`crates/trcli-testing/src/contract_*.rs`.

## 3. The behaviour only the kind knows

```rust
impl<U: SpecimenStore> KindBehaviour<U> for Specimens {
    fn descriptor(&self) -> &RecordKindDescriptor { … }

    /// What, if anything, forbids deleting this record — with the alternative to offer.
    async fn deletion_block(&self, unit: &U, id: RecordId) -> Result<Option<DeletionBlock>, StoreError> { … }

    /// What else refers to it and would be affected, beyond links, tags, and notes.
    async fn dependents(&self, unit: &U, id: RecordId) -> Result<Vec<String>, StoreError> { … }

    /// Removes the kind's own rows; called inside the foundation's unit of work.
    async fn remove_rows(&self, unit: &mut U, id: RecordId) -> Result<(), StoreError> { … }

    /// The kind's own fields, in the order `show` displays them.
    async fn fields(&self, unit: &U, id: RecordId) -> Result<Vec<KindField>, StoreError> { … }
}
```

The second sample kind, `sample-note`, shows a deletion that is forbidden: a locked note
answers `deletion_block` with the reason and the alternative ("unlock it first").

## 4. `add` and `edit`

Each takes a **validated command** built by a constructor that reports every problem at
once, and calls two helpers of the foundation:

```rust
pub async fn add<U>(unit: &mut U, stamp: &Stamp, ids: &impl IdGenerator, command: AddSpecimen) -> Result<…, Problem>
where
    U: SpecimenStore + RecordIndex + TagStore + AuditLog,
{
    let id = ids.next_id();
    let changes = vec![Change::set("title", command.title.as_str())];
    // The index row comes first: the kind's own row refers to it.
    let record = register(unit, stamp, &descriptor(), id, (command.title.as_str(), changes)).await?;
    unit.insert_specimen(id, &command.title).await?;
    …
}
```

- `records::create::register` gives the record its short name, puts it in the index, and
  records the `create` audit entry.
- `records::create::update` records an `update` entry with the fields that changed, and
  renames the record in the index when its name changed. Build the list of changes with
  `ChangeSet`: a field whose value did not change is left out, and a secret field is never
  written to the trail.

## 5. The table

One table per kind, whose `id` refers to the record index. That foreign key is what makes
"nothing is left pointing at a deleted record" a rule of the database:

```sql
CREATE TABLE specimen (
    id    TEXT PRIMARY KEY NOT NULL REFERENCES record (id),
    title TEXT NOT NULL
);
```

A feature adds its migration to `Migrator::migrations()` in
`crates/trcli-infra-sqlite/src/migrations/mod.rs`, after the existing ones, and its tables
to `EXPECTED_TABLES`. (The example cannot: it lives outside the crate, so it creates its
tables itself where they are missing, and they are not part of the workspace format.)
Identifiers are stored as hyphenated text and moments as whole milliseconds; see
`convert.rs`.

## 6. The commands

The kind's noun with its own verbs, plus the shared ones in one call:

```rust
Command::new("specimen")
    .about(descriptor.summary)
    .subcommands(own)                                      // add, edit
    .subcommands(shared_verbs::commands(&descriptor))      // list, show, rm, tag, note
```

and in the dispatch, the shared verbs are tried first:

```rust
if let Some(shared) = shared_verbs::run(session, &Specimens::new(), verb, matches).await {
    return shared;
}
```

## 7. Registration

A feature tells the tool about itself through the `Extension` trait
(`crates/trcli-cli/src/extension.rs`): the kinds it owns, the nouns it adds, and how its
commands are run.

```rust
impl Extension for SampleKinds {
    fn register_kinds(&self, kinds: &mut KindRegistry) -> Result<(), KindError> { … }
    fn commands(&self) -> Vec<Command> { … }
    async fn run(&self, session: &mut Session, noun: &str, matches: &ArgMatches) -> Result<Reply, Problem> { … }
}
```

The example's `main.rs` is one line, `trcli_cli::run::main_with(&sample::SampleKinds)`.
Two kinds with the same name or prefix stop the tool at start-up, where you see it at once.

## 8. What you did not write

Run the scenarios of your spec. Without code of yours, records of the new kind can be
listed with `--search`, `--tag`, `--sort`, `--desc`, `--limit`; shown with their tags,
notes, and links; tagged and noted; linked to records of any kind; deleted with the list of
what refers to them and a confirmation; shown in both output forms; counted in `workspace
show`; and followed in `audit list --kind <your kind>`.

If something of that needed a change to foundation code, the contract has a gap: say so in
your pull request rather than working around it.
