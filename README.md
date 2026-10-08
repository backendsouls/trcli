# TRCLI — The Research CLI

A command-line tool for keeping the records of your research in one place: a
**workspace** on your own machine, with one way of naming and finding records, every input
checked before anything changes, answers for people and for programs, and a record of
everything that happened that can be verified.

This repository currently holds the **foundation** (`specs/000-foundation`): the ground
every feature stands on. The features themselves — literature, experiments, manuscripts,
projects, and the rest — are specified under `specs/` and are built on it one by one.

```console
$ trcli init --name "Doctorate"
Created workspace "Doctorate" in /home/ana/research/.trcli

$ trcli workspace show
Workspace "Doctorate"
  Description  (none)
  Location     /home/ana/research
  Format       1
  Created      2026-10-08 14:00 UTC
Records
  (this version of trcli has no kinds of record yet)

$ trcli audit list
SEQ  WHEN              ACTOR  ACTION  WHAT
1    2026-10-08 14:00  ana    create  "Doctorate" name: (none) → Doctorate
```

## What the foundation gives you

- **A workspace** found from wherever you stand inside it; several workspaces stay separate.
- **Settings in layers**: yours, the workspace's, the session's, the command's — and a
  listing that shows every value and where it comes from.
- **Checked input**: every value is checked before anything changes, and every problem is
  reported at once with what would be right.
- **Two forms of output**: readable at a terminal, plain when piped, one JSON document
  with `--output json`; exit codes that mean something.
- **An audit trail** of every change, chained so that tampering is detected, exportable as
  Markdown, JSON, or CSV.
- **Nothing leaves your machine.** Local telemetry about the tool's own use stays in the
  workspace and can be turned off.

Usage guides with worked examples are in [`docs/usage/`](./docs/usage/README.md). Every
example in them is run against the tool by the test suite.

## Install

TRCLI is one self-contained program; it needs nothing else installed.

**From a release.** Download the archive for your system from the
[releases page](https://github.com/backendsouls/trcli/releases), unpack it, and put
`trcli` (or `trcli.exe`) somewhere on your `PATH`:

| System | Archive |
|--------|---------|
| Linux, x86_64 | `trcli-<version>-x86_64-unknown-linux-gnu.tar.gz` |
| Linux, ARM 64 | `trcli-<version>-aarch64-unknown-linux-gnu.tar.gz` |
| macOS, Intel | `trcli-<version>-x86_64-apple-darwin.tar.gz` |
| macOS, Apple silicon | `trcli-<version>-aarch64-apple-darwin.tar.gz` |
| Windows, x86_64 | `trcli-<version>-x86_64-pc-windows-msvc.zip` |
| Windows, ARM 64 | `trcli-<version>-aarch64-pc-windows-msvc.zip` |

**From source.** With [Rust](https://rustup.rs) 1.96 or newer:

```sh
git clone https://github.com/backendsouls/trcli
cd trcli
cargo install --path crates/trcli-cli
```

Then:

```sh
trcli --version
trcli init --name "My research"
trcli                      # suggests the first steps in a new workspace
```

Shell completion: `trcli completions bash|zsh|fish|powershell` prints a script for your shell.

## For contributors

[`CONTRIBUTING.md`](./CONTRIBUTING.md) explains how to build, check, and add to TRCLI.
The short version:

```sh
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo test --workspace --features trcli-cli/test-clock
```

The code is a Cargo workspace of five crates, one per layer, so that the compiler enforces
which code may depend on which:

```text
trcli-cli  ─▶  trcli-infra-sqlite  ─▶  trcli-application  ─▶  trcli-domain
           ─▶  trcli-infra-system  ─▶
```

A sixth crate, `trcli-testing`, holds the test doubles and is never compiled into the
tool. `crates/trcli-cli/examples/sample_kinds` is the same tool with two sample kinds of
record added from outside, which is how a feature plugs in:

```sh
cargo run -p trcli-cli --example sample_kinds -- specimen add --title "Soil sample 14"
```

## License

MIT or Apache-2.0, at your option.
