# Quickstart: Validating the TRCLI Foundation

**Plan**: [plan.md](./plan.md) | **Contracts**: [contracts/README.md](./contracts/README.md) | **Data model**: [data-model.md](./data-model.md)

How to build `trcli`, run every check, and walk through each part by hand to
confirm the foundation works end to end. Handles shown (`spc-…`) are examples; use the ones
your commands print.

## Prerequisites

- Rust 1.96 or newer (`rustup show` reads `rust-toolchain.toml`)
- Git
- Nothing else: SQLite is compiled into the binary

## Build and check

```sh
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings       # includes documentation and readability lints
cargo doc --workspace --no-deps --document-private-items   # fails on any undocumented item

cargo test --workspace                                      # unit, contract, upgrade, and structural tests
cargo test --workspace --features trcli-cli/test-clock
                                                            # the same, plus the acceptance scenarios against
                                                            # the binary, the usage guides' examples, and the
                                                            # gates that need the sample kinds

# One suite at a time:
cargo test -p trcli-cli --test layering                     # the dependency graph obeys the layers
cargo test -p trcli-cli --test scenario_coverage            # every acceptance scenario has an automated check
cargo test -p trcli-cli --test help_examples                # every command has help with an example
cargo test -p trcli-cli --test bdd --example sample_kinds --features test-clock
cargo test -p trcli-cli --test usage --example sample_kinds --features test-clock
cargo test --release -p trcli-cli --test performance --example sample_kinds --features test-clock -- --ignored
```

**Expected**: every command exits 0. CI runs the same on Linux, macOS, and Windows.

The scenarios of stories 2 and 8 need records. They use two sample kinds (`specimen`,
`sample-note`) that live in an example: the same tool with those kinds added from outside.

```sh
cargo build -p trcli-cli --example sample_kinds      # target/debug/examples/sample_kinds
```

## Manual walk-through

```sh
mkdir -p /tmp/trcli-demo/project/deep/er && cd /tmp/trcli-demo
alias trcli="$OLDPWD/target/debug/trcli"
```

### Part: skeleton, output, exit codes

```sh
trcli                         # short help, exit 0
trcli --version               # version and the workspace format it reads and writes
trcli help
trcli nosuchcommand           # exit 2, suggests the nearest command
trcli workspace show          # exit 4: no workspace here; says how to create one
trcli workspace show --output json   # same failure as one JSON document; still exit 4
```

### Part: validation

Validation is exercised through every later command; the pattern to look for:

```sh
trcli init --name ""          # exit 2: names the value, says what is expected
```

**Expected**: the message names the option as typed, says what is wrong and what is
expected, and ends with "Nothing was changed."

### Part: workspace

```sh
cd project
trcli init --name "Demo" --description "Trying the foundation"
trcli init --name "Again"                 # exit 4: a workspace already exists here
cd deep/er && trcli workspace show        # found from two levels down
cd /tmp && trcli workspace show           # exit 4
trcli --workspace /tmp/trcli-demo/project workspace show
TRCLI_WORKSPACE=/tmp/trcli-demo/project trcli workspace show
cd /tmp/trcli-demo/project
trcli workspace edit --name "Demo renamed"
mv /tmp/trcli-demo/project /tmp/trcli-demo/moved && cd /tmp/trcli-demo/moved
trcli workspace show                      # still works after the move
```

### Part: settings

```sh
trcli config list                         # every setting, its value, where it comes from
trcli config get output.color             # meaning, allowed values, default
trcli config set output.color never
trcli config set --user output.page_size 20
trcli config set output.page_size 0       # exit 2: must be between 1 and 1000
trcli config set no.such.setting 1        # exit 2: unknown setting
TRCLI_OUTPUT_COLOR=always trcli config list   # the session wins over the files
trcli config unset output.color
trcli config path
echo 'output.colour = "never"' >> .trcli/config.toml
trcli workspace show                      # exit 2: names the file and the unknown key
```

Remove the bad line before continuing.

### Part: audit trail and telemetry

```sh
trcli audit list                          # the init, the rename, the settings changes
trcli audit list --action setting --from 2026-10-01
trcli audit verify                        # exit 0: intact
trcli audit export --to audit.md
trcli telemetry show
trcli telemetry off && trcli workspace show && trcli telemetry show    # nothing new recorded
trcli audit list --limit 1                # the trail continued while telemetry was off
```

Tamper tests, with any SQLite tool:

1. Change one field of one row of `audit_entry`; run `trcli audit verify`.
2. Restore it; delete the last row; run `trcli audit verify`.

**Expected**: exit 6 both times — first naming the altered entry, then reporting that the
trail was shortened.

### Part: records (with the example: `alias trcli="$OLDPWD/target/debug/examples/sample_kinds"`)

```sh
trcli specimen add --title "First"        # prints spc-…
trcli specimen add --title "Second"
trcli specimen show spc-7                 # a unique beginning is enough
trcli specimen show SPC-7                 # letter case does not matter
trcli specimen show spc                   # exit 3: ambiguous, lists both
trcli specimen show spc-zzzz              # exit 3: not found, suggests close ones
trcli specimen tag spc-… Field-Work       # stored as "field-work"
trcli specimen tag spc-… "two words"      # exit 2: states the rule for tag names
trcli specimen note spc-… "Collected in the rain"
trcli link add spc-…1 spc-…2 --relation "same site"
trcli link add spc-…1 spc-…1              # exit 2: a record cannot be linked to itself
trcli specimen list --tag field-work --search first --sort title --limit 10
trcli specimen list --output json
trcli specimen rm spc-…1                  # lists the link, the tag, the note; asks
trcli specimen rm spc-…1 --no-input       # exit 5: confirmation required; nothing changed
trcli specimen rm spc-…1 --yes
trcli link list spc-…2                    # the link is gone
trcli workspace show                      # counts: specimen 1
trcli audit list --kind specimen          # create, tag, note, link, delete — with the name it had
```

### Part: interaction

```sh
trcli specimen rm spc-…2 < /dev/null      # exit 5 at once; never waits
trcli specimen list | cat                 # no colour codes
NO_COLOR=1 trcli specimen list            # no colour
trcli specimen list --color always | cat  # colour forced
COLUMNS=30 trcli specimen list            # still readable; long values shortened
```

Interruption and concurrency (the sample kind has a deliberately slow command):

```sh
trcli specimen slow --seconds 20          # shows progress; press Ctrl-C
echo $?                                   # 130
trcli audit verify                        # intact: nothing half-done
trcli specimen slow --seconds 10 & trcli specimen add --title "Meanwhile"
```

**Expected**: the second command completes after the first or reports the workspace as
busy; either way `trcli audit verify` passes.

### Part: upgrade and safety

```sh
# Needs a fixture of an earlier released format; there is none before the second release.
cp -r "$OLDPWD/tests/fixtures/formats/1" /tmp/trcli-demo/old && cd /tmp/trcli-demo/old
trcli workspace show                      # works; says an upgrade is needed
trcli specimen add --title "x"            # exit 4: upgrade first
trcli workspace upgrade --check
trcli workspace upgrade                   # copies to .trcli/backups/, upgrades
ls .trcli/backups/
trcli audit verify
```

Too-new and damaged workspaces:

```sh
# raise format_version in the workspace row by hand, then:
trcli workspace show                      # exit 4: made by a newer version; nothing changed
# truncate .trcli/trcli.db to a few bytes in a scratch copy, then:
trcli workspace show                      # exit 4: reports damage, suggests .trcli/backups/
```

### Part: help and contributor experience

```sh
trcli completions bash | head
trcli workspace --help                    # purpose, options, an example
trcli                                     # in an empty workspace: suggests first steps
```

Follow `CONTRIBUTING.md` from a fresh clone and `docs/contributing/adding-a-command.md`.

**Expected**: built, all checks green, and a trivial command added in under an hour
(SC-015).

## Cross-cutting checks

| Check | How | Expected |
|-------|-----|----------|
| Start-up (SC-012) | `time trcli --version`; `time trcli workspace show` | Under 100 ms each (measured on Linux, release build: 3 ms and 16 ms) |
| Scale (SC-012) | Load 10,000 specimens from a script; `time trcli specimen list --search x` | Under 2 s |
| Nothing leaves the machine (SC-010) | Run the whole walk-through with the network disconnected | Everything works |
| Power loss (SC-008) | Kill the process (`kill -9`) during `specimen slow`; then `trcli audit verify` and `workspace show` | Valid; the change is wholly absent |
| Both forms agree (SC-005) | Compare `specimen show` with `specimen show --output json` | Same content |
| Any script (FR-067) | Add specimens titled "Ação", "acao", "東京"; `--search acao`; `--sort title` | Both Portuguese spellings found; all three listed and sorted without error |
| Three systems (SC-014) | CI | Every suite green on Linux, macOS, Windows |
| Layers (FR-070) | Add `sea-orm` to `trcli-domain/Cargo.toml`; run `cargo test --test layering` | Fails, naming the forbidden dependency |

## Clean up

```sh
cd / && rm -rf /tmp/trcli-demo
```
