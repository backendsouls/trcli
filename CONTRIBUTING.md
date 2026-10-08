# Contributing to TRCLI

This guide takes you from a fresh clone to a change that passes every check. If you only
want to add a command, read this page and then
[`docs/contributing/adding-a-command.md`](./docs/contributing/adding-a-command.md); for a
new kind of record, [`docs/contributing/adding-a-record-kind.md`](./docs/contributing/adding-a-record-kind.md).

## Prerequisites

- [Rust](https://rustup.rs) — `rust-toolchain.toml` pins the version; `rustup` installs it
  the first time you build.
- Git.
- Nothing else. SQLite is compiled into the binary.

## Build and run

```sh
cargo build --workspace
cargo run -p trcli-cli -- --help

# The same tool with two sample kinds of record added, to try what every record shares:
cargo run -p trcli-cli --example sample_kinds -- specimen add --title "First"
```

## Every check, and what it protects

Run these before opening a pull request. CI runs the same on Linux, macOS, and Windows.

| Command | What it protects |
|---------|------------------|
| `cargo fmt --all --check` | One formatting for everyone |
| `cargo clippy --workspace --all-targets -- -D warnings` | Every item documented, public and private; functions at most 50 lines, 5 arguments, and a complexity of 10; no wildcard imports |
| `cargo doc --workspace --no-deps --document-private-items` (with `RUSTDOCFLAGS=-D warnings`) | The documentation builds and its links resolve |
| `cargo test --workspace` | Unit tests; the use cases against the doubles; the contract suites against the doubles **and** against SQLite; upgrade tests; and the structural gates below |
| `cargo test --workspace --features trcli-cli/test-clock` | The same, plus the acceptance scenarios against the built tool and the usage guides' examples, which need a fixed clock and seeded identifiers |

The structural gates, each a test file at the repository root:

| Test | Fails when |
|------|-----------|
| `tests/layering.rs` | a crate gains a dependency its layer may not have, or anything that can reach the network |
| `tests/scenario_coverage.rs` | an acceptance scenario of the spec has no automated check |
| `tests/invalid_input_gate.rs` | an input of a command has no scenario in which an invalid form of it is rejected |
| `tests/help_examples.rs` | a command has no description or no example, an option has no help, or a group of commands has no usage guide |
| `tests/usage.rs` | an example in `docs/usage/*.md` does not behave as written |
| `tests/sample_kind_is_external.rs` | a source file of any crate names a sample record kind |
| `tests/forms_agree.rs` | the form for people leaves out something the structured form says |

Timings (`tests/performance.rs`) are run on purpose:
`cargo test --release -p trcli-cli --test performance --example sample_kinds --features test-clock -- --ignored`.

## The order of work

Tests come first. This is not a preference; it is the first principle of the project's
constitution that a reviewer will ask you to show.

1. **Failing scenarios.** Write the acceptance scenarios of your spec as Gherkin under
   `tests/features/<spec>/`, each tagged with the scenario it automates (`@US3-07` is user
   story 3, scenario 7) and, when it shows a rejection, with `@invalid`. Run them and see
   them fail. The steps in `tests/bdd/steps.rs` are generic; you should rarely need a new one.
2. **Failing unit tests**, inside-out: a value object or a rule in the domain, then the
   use case with in-memory fakes, then the adapter with the contract suite.
3. **The code** that makes them pass.
4. **The usage guide**, `docs/usage/<noun>.md`, with examples. Write the commands, then run
   `TRCLI_BLESS_USAGE=1 cargo test -p trcli-cli --test usage --example sample_kinds --features test-clock`
   to fill in what the tool prints, and read the difference before committing it.

Open one pull request per phase of your spec's `tasks.md`, stacked on the previous one.
Never commit to `main` directly.

## The layers, and where each kind of code goes

```text
trcli-cli  ─▶  trcli-infra-sqlite  ─▶  trcli-application  ─▶  trcli-domain
           ─▶  trcli-infra-system  ─▶
```

| Crate | Holds | May not |
|-------|-------|---------|
| `trcli-domain` | The rules of the research: value objects, entities, what is valid | Do I/O, use `async`, depend on any framework |
| `trcli-application` | Use cases; ports (small traits named for what the caller needs); validated commands; view models | Name an adapter, parse a command line, render |
| `trcli-infra-sqlite` | SeaORM entities, migrations, the stores behind the storage ports | Know the command line, the file system's conventions, or the other adapter |
| `trcli-infra-system` | Paths, files, the clock, identifiers, the actor: everything that differs between systems | Know SQL |
| `trcli-testing` | In-memory doubles of every port, and the contract suites every adapter must pass. A development dependency only | Be compiled into the tool (`tests/layering.rs` checks) |
| `trcli-cli` | clap definitions (`args/`), handlers (`commands/`), rendering (`render/`), and `compose.rs` | Decide anything a use case should decide |

### Where tests go

| Kind of test | Where | Why there |
|--------------|-------|-----------|
| Pure logic: a value object, a rule, a parser | Inline, in a `#[cfg(test)] mod tests` beside the code | Rust's convention; it can see private items |
| A use case, which needs doubles of its ports | `crates/trcli-application/tests/use_cases/<module>.rs` | It uses only the public interface, and the doubles live outside the crate |
| An adapter against its port's contract | `crates/trcli-infra-*/tests/` | The same suite the double passes |
| Behaviour through the built tool | `tests/features/` (Gherkin), `tests/*.rs` | They run the binary as a researcher would |

Doubles are **fakes**, not mocks: small working implementations held to the same contract
suites as the real adapters. A test then says what must be true afterwards, not which
calls were made, and a use case can be rewritten without rewriting its tests.

A feature is a **module of the same name in each layer**, not a crate. It refers to another
feature's records by `RecordRef` only, and reaches another feature's behaviour only through
a port that feature publishes.

`compose.rs` is the only file that names concrete adapters.

### The path of one command

```text
argv ─▶ clap ─▶ session (settings, workspace) ─▶ command constructor (every value checked)
     ─▶ handler ─▶ use case ─▶ ports ─▶ unit of work: the change and its audit entry, or neither
     ◀─ view model ─▶ renderer ─▶ stdout (result) / stderr (everything else) ─▶ exit code
```

A handler does four things and nothing else: builds a validated command, opens a unit of
work, calls one use case, commits.

## Rules for code a human can read

The build enforces most of these; the rest are what a reviewer looks for.

- **Every item has a doc comment** that says what it is *for*, in the words of the spec —
  modules, types, functions, fields, constants, private ones included.
- **Every module file opens** with what it contains, which requirements it implements, and
  what it deliberately does not do.
- **Inline comments say why** a line is there, citing the requirement (`FR-047`) when a
  rule comes from the spec. They do not restate the code.
- **Names are whole words** from the spec's vocabulary. No abbreviations but `id`.
- **Functions are short**: at most 50 lines, 5 arguments, a complexity of 10. Prefer early
  returns to nesting. A function either decides or performs I/O, not both.
- **Async only where necessary**: on ports that reach storage or wait for a person, and
  the handlers that await them.
- **No new library** without a written reason in `specs/000-foundation/research.md` §3.
  The test: would our own version exceed about 60 lines, or be easy to get subtly wrong
  (Unicode, cryptography, time zones, SQL)? If neither, we write it.

## SOLID, as rules

| Principle | Rule here | How it is checked |
|-----------|-----------|-------------------|
| **S**ingle responsibility | One use case per file; one reason to change per module | The length and complexity lints; review |
| **O**pen/closed | A feature is added by registering — a record kind, settings, problem codes — never by editing a central `match` | `tests/sample_kind_is_external.rs` |
| **L**iskov substitution | Every implementation of a port passes the port's contract suite | The same suite runs against the fake and the adapter |
| **I**nterface segregation | Ports are small and named for what the caller needs | A use case's type parameters list exactly the ports it uses |
| **D**ependency inversion | Use cases depend on ports defined beside them; adapters depend on the application crate | `tests/layering.rs`; only `compose.rs` names adapters |

A trait is introduced where there is a second implementation (at least a test fake) or a
second caller — not "for flexibility".

## Release checklist

1. Every check above is green on the three systems.
2. **If this release changes how a workspace is stored:** before raising
   `FormatVersion::CURRENT`, add a fixture workspace of the outgoing format under
   `tests/fixtures/formats/<number>/` (see the README there); then raise the number, and
   check that `crates/trcli-infra-sqlite/tests/upgrade.rs` upgrades the fixture.
3. Update the version in the workspace `Cargo.toml`.
4. Tag `v<version>`; the release workflow builds a binary for each system and drafts the
   release.
5. Check that the release binaries have no `specimen` or `sample-note` command.
