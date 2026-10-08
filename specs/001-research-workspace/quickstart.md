# Quickstart: Validating the TRCLI Research Workspace

**Plan**: [plan.md](./plan.md) | **Commands**: [contracts/cli-conventions.md](../000-foundation/contracts/cli-conventions.md) | **Output**: [contracts/output-and-exit-codes.md](../000-foundation/contracts/output-and-exit-codes.md)

How to build `trcli`, run its test suites, and walk through each delivery slice by hand to
confirm it works end to end. Handles shown (`pap-…`, `run-…`) are examples; use the ones
your commands print.

## Prerequisites

- Rust 1.96 or newer (`rustup show` picks it up from `rust-toolchain.toml`)
- Git
- Nothing else: SQLite is compiled into the binary

## Build and test

```sh
cargo build --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo doc --workspace --no-deps                      # fails on any undocumented public item

cargo test --workspace                               # unit + adapter integration tests
cargo test --test bdd                                # Gherkin scenarios against the binary
cargo test --test usage                              # examples in docs/usage/*.md
```

Run one story's scenarios:

```sh
cargo test --test bdd -- --input tests/features/01_papers_citations.feature
```

**Expected**: every command exits 0. CI runs the same commands on Linux, macOS, and Windows.

## Manual walk-through

Use a scratch directory so nothing touches real work:

```sh
mkdir /tmp/trcli-demo && cd /tmp/trcli-demo
alias trcli="$OLDPWD/target/debug/trcli"
```

### Slice 0 — workspace, settings, output

```sh
trcli paper list                         # exit 4: no workspace, says to run `trcli init`
trcli init --name "Demo"                 # creates .trcli/
trcli init --name "Again"                # exit 4: workspace already exists
trcli workspace show
trcli config list
trcli config set output.color never
trcli config set output.page_size 0      # exit 2: must be between 1 and 1000
trcli workspace show --output json       # one JSON document on stdout
trcli audit list                         # the init and config changes are there
```

### Slice 1 — papers and citations (Story 1, SC-001)

```sh
trcli paper add --title "Attention Is All You Need" --author "Vaswani, Ashish" --year 2017
trcli paper add --title "" --year 20244              # exit 2: both problems reported together
trcli paper add --title "Attention is all you need" --author "Vaswani, A." --year 2017
                                                     # reports a likely duplicate
trcli paper status pap-… reading
trcli paper tag pap-… transformers nlp
trcli paper list --status reading --tag nlp
trcli cite add pap-…                                 # prints key vaswani2017attention
trcli cite format vaswani2017attention --style apa
trcli cite export --format bibtex --to refs.bib
trcli paper import refs.bib --on-duplicate skip      # 0 imported, 1 duplicate skipped
trcli paper add --doi 10.48550/arXiv.1706.03762      # online lookup; shows details first
trcli config set lookup.enabled false
trcli paper add --doi 10.1000/xyz                    # exit 7: lookup disabled, offers manual entry
trcli paper rm pap-…                                 # lists the citation, asks to confirm
trcli paper rm pap-… --no-input                      # exit 5: confirmation required
```

**Check**: first paper and citation in under 3 minutes using only `--help`.

### Slice 2 — questions and hypotheses (Story 2)

```sh
trcli question add --statement "Does X improve Y?"
trcli hypothesis add rq-… --statement "X improves Y by at least 5%"
trcli question link rq-… pap-…
trcli hypothesis status hyp-… supported              # exit 2: needs linked evidence
trcli question overview rq-…
trcli question unlinked
```

### Slice 3 — bibliographic research (Story 3)

```sh
trcli review add --title "X for Y: a review" --question "What is known about X for Y?"
trcli review search add rev-… --query '"X" AND "Y"' --source "Scopus" --date 2026-10-01 --results 42
trcli review candidate add rev-… pap-…
trcli review screen rev-… pap-… --exclude            # exit 2: reason required
trcli review screen rev-… pap-… --exclude --reason "out of scope"
trcli review summary rev-…
```

### Slice 4 — drafts (Story 4)

```sh
trcli draft add --title "Our Paper" --deadline 2027-01-15
trcli draft stage ms-… drafting
trcli draft version add ms-… --summary "First full draft"
trcli draft cite ms-… vaswani2017attention
trcli draft bib ms-… --to paper.bib
trcli draft list --sort deadline
```

### Slice 5 — experiments and runs (Story 5, SC-005, SC-006)

```sh
trcli experiment add --name "Baseline" --hypothesis hyp-…
trcli step add exp-… --key prepare --name "Prepare" --run echo --arg prepared
trcli step add exp-… --key label   --name "Label by hand" --manual \
      --instructions "Label 50 samples" --after prepare
trcli step add exp-… --key train   --name "Train" --run echo --arg trained --after label
trcli step add exp-… --key bad     --name "Loop" --run echo --after bad    # exit 2: loop named

trcli run start exp-…          # runs `prepare`, prints the manual instructions, RETURNS
trcli run list --status paused
trcli run confirm run-… --notes "50 labelled"       # continues with `train`
trcli run show run-…
```

Interruption: add a step that runs `sleep 30` (`timeout /T 30` on Windows), start a run,
press Ctrl-C.

**Expected**: exit code 130; `trcli run show` reports `interrupted`; `trcli run resume`
continues from the interrupted step without repeating finished ones.

### Slice 6 — results, figures, tables (Story 6, SC-016)

```sh
trcli result add run-… --name accuracy --value 0.91 --unit "" --uncertainty 0.01
echo "x" > fig1.png && trcli figure add run-… --title "Accuracy" --file fig1.png
trcli result evidence res-… hyp-… --supports
trcli hypothesis status hyp-… supported             # now accepted
trcli draft report ms-… res-… fig-…
trcli result origin res-…                           # run, pipeline, data, method, environment
echo "y" > fig1.png && trcli figure verify fig-…    # exit 6: file changed
```

Start and finish a second run, record `accuracy` again, then `trcli draft evidence ms-…`.

**Expected**: the draft is flagged as reporting a result that a newer run has replaced.

### Slice 7 — methodologies and datasets (Story 7)

```sh
trcli method add --name "5-fold cross-validation"
mkdir data && echo "a,b" > data/train.csv
trcli dataset add --name "Train set" --location data
trcli dataset verify dat-…                           # unchanged
echo "c,d" >> data/train.csv
trcli dataset verify dat-…                           # exit 6: changed, offers a new version
trcli dataset version add dat-… --note "added rows"
trcli dataset add --name "Missing" --location nowhere    # exit 2: location does not exist
```

### Slice 8 — environment and reproducibility (Story 8, SC-008)

```sh
trcli config set environment.tools '["git"]'
trcli config set environment.variables '["API_TOKEN"]'   # exit 2: looks like a secret
trcli env capture --name "laptop"
trcli repro check run-…                # exit 0, "reproducible"
echo "e,f" >> data/train.csv
trcli repro check run-…                # exit 6: lists the dataset difference
trcli repro package run-… --to run.trcli-repro
```

### Slice 9 — staff (Story 9)

```sh
trcli staff add --name "Silva, Ana" --role "PhD student" --email "not-an-email"   # exit 2
trcli staff add --name "Silva, Ana" --role "PhD student" --email ana@example.org
trcli draft author set ms-… stf-…
trcli staff assignments stf-…
```

### Slice 10 — audit and telemetry (Story 10, SC-010, SC-011)

```sh
trcli audit list --record pap-…
trcli audit list --action delete --from 2026-10-01
trcli audit verify                     # exit 0: intact
trcli audit export --to audit.md
trcli telemetry show --experiment exp-…
trcli telemetry off && trcli telemetry status
```

Tamper test: with any SQLite tool, change one row of the audit table, then run
`trcli audit verify`.

**Expected**: exit code 6, naming the first entry that no longer matches.

### Slice 11 — courses (Story 11)

```sh
trcli course add --title "Research Methods" --taking --from 2026-08-01 --to 2026-07-01   # exit 2
trcli course add --title "Research Methods" --taking --from 2026-08-01 --to 2026-12-15
trcli course link crs-… pap-… mth-…
trcli course list --status planned
```

## Cross-cutting checks

| Check | How | Expected |
|-------|-----|----------|
| Output is script-safe | `trcli paper list --output json \| python3 -m json.tool` | Valid JSON, no color codes |
| Color follows the rules | `trcli paper list \| cat`; `NO_COLOR=1 trcli paper list` | No escape codes in either |
| Nothing hangs without a terminal | `trcli paper rm pap-… < /dev/null` | Exit 5, immediately |
| Offline (SC-015) | Disconnect the network; run slices 1–11 except lookups; run one lookup | All pass; the lookup fails in under 10 s with exit 7 |
| Scale (SC-002, SC-003) | Import a generated 10,000-entry `.bib`; time `paper list --search` | Import of 1,000 under 1 min; search under 2 s |
| Platforms | CI matrix | All suites green on Linux, macOS, Windows |

## Clean up

```sh
cd / && rm -rf /tmp/trcli-demo
```
