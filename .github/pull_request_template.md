<!-- What this change does, in a sentence or two, and which specification it belongs to. -->

## Specification

<!-- Link the spec, plan, and tasks this implements, and the task numbers. -->

## Acceptance gates

These are the gates of `specs/000-foundation/contracts/feature-contract.md`. CI checks
most of them; the first is yours to show.

- [ ] **Tests came first.** Link or paste the failing run from before the change:
- [ ] **Behaviour.** Every acceptance scenario of the specification is a Gherkin scenario,
      tagged with the scenario it automates, and passes against the binary.
- [ ] **Invalid input.** Every argument and option of every new command appears in a
      scenario tagged `@invalid` (`tests/invalid_input_gate.rs`).
- [ ] **Documentation.** Every new item, public and private, has a doc comment that says
      what it is for; each new module says what it contains and what it does not do.
- [ ] **Usage guide.** `docs/usage/<noun>.md` exists for each new group of commands and its
      examples pass (`tests/usage.rs`).
- [ ] **Help.** Every new command has help text with an example (`tests/help_examples.rs`).
- [ ] **Layers.** `tests/layering.rs` passes; no adapter is named outside `compose.rs`.
- [ ] **Contracts.** Every new port has a contract suite passed by its fake and its adapter.
- [ ] **Three systems.** CI is green on Linux, macOS, and Windows.
- [ ] **Upgrade.** If storage changed: a migration, and the upgrade tests pass.

## Notes for the reviewer

<!-- Anything unusual: a new dependency (and its reason in research.md §3), a deviation
     from the plan, something left for a later pull request. -->
