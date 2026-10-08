# specs/000-foundation, User Story 3: Have every input checked before anything changes.
Feature: Have every input checked before anything changes

  Background:
    Given a workspace named "Lab"

  @US3-01 @invalid
  Scenario: An invalid value stores nothing and is named with what is wrong and what is expected
    When I run "trcli specimen add --title ''"
    Then the exit code is 2
    And stderr contains "--title"
    And stderr contains "must not be empty"
    And stderr contains "1 to 500 characters"
    And stderr contains "Nothing was changed."
    And stdout is empty
    And nothing was changed

  @US3-02 @invalid
  Scenario: Several invalid values are all reported together
    When I run "trcli specimen list --tag 'two words' --sort colour --limit 0"
    Then the exit code is 2
    And stderr contains "3 values are invalid"
    And stderr contains "--tag"
    And stderr contains "--sort"
    And stderr contains "--limit"
    And nothing was changed

  @US3-03 @invalid
  Scenario: A value of the wrong form is answered with an example of a valid one
    When I run "trcli audit list --from 08/10/2026"
    Then the exit code is 2
    And stderr contains "--from"
    And stderr contains "is not a date"
    And stderr contains "for example: 2026-10-08"
    When I run "trcli audit list --limit ten"
    Then the exit code is 2
    And stderr contains "is not a whole number"
    And stderr contains "for example: 1"
    When I run "trcli specimen slow --seconds soon"
    Then the exit code is 2
    And stderr contains "--seconds"

  @US3-04 @invalid
  Scenario: A value outside its set is answered with the valid choices
    When I run "trcli specimen list --sort colour"
    Then the exit code is 2
    And stderr contains "title, name, created, updated, handle"
    When I run "trcli audit list --action explode"
    Then the exit code is 2
    And stderr contains "create, update, delete"
    When I run "trcli audit list --kind planet"
    Then the exit code is 2
    And stderr contains "specimen, sample-note"
    When I run "trcli tag list --kind planet"
    Then the exit code is 2
    And stderr contains "specimen, sample-note"
    When I run "trcli audit export --to audit.pdf --format pdf"
    Then the exit code is 2
    And stderr contains "markdown, json, csv"
    When I run "trcli workspace show --output xml"
    Then the exit code is 2
    And stderr contains "human, json"
    When I run "trcli workspace show --color sometimes"
    Then the exit code is 2
    And stderr contains "auto, always, never"
    When I run "trcli completions tcsh"
    Then the exit code is 2
    And stderr contains "bash"

  @US3-05 @invalid
  Scenario: A value that names a record that does not exist is reported like any other invalid value
    When I run "trcli audit list --record spc-zzzzzzzz --limit 0"
    Then the exit code is 2
    And stderr contains "2 values are invalid"
    And stderr contains "--record"
    And stderr contains "no record matches"
    And stderr contains "--limit"
    When I run "trcli audit export --record spc-zzzzzzzz --to audit.md"
    Then the exit code is 2
    And the file "audit.md" does not exist

  @US3-06 @invalid
  Scenario: Text that is empty, too long, or cannot be stored is rejected with the field and the limit
    When I run "trcli specimen add --title '{long title}'"
    Then the exit code is 2
    And stderr contains "is too long (501 characters)"
    And stderr contains "1 to 500 characters"
    When I run "trcli specimen add --title 'ring{bell}ring'"
    Then the exit code is 2
    And stderr contains "control characters"
    When I run "trcli sample-note add --body ' '"
    Then the exit code is 2
    And stderr contains "--body"
    When I run "trcli workspace edit --name '' --researcher ''"
    Then the exit code is 2
    And stderr contains "2 values are invalid"
    And stderr contains "--name"
    And stderr contains "--researcher"
    And stderr contains "1 to 200 characters"
    And nothing was changed
    When I run "trcli workspace edit --description '{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}'"
    Then the exit code is 2
    And stderr contains "at most 20000 characters"
    When I run "trcli workspace edit"
    Then the exit code is 2
    And stderr contains "nothing to change was given"

  @US3-06 @invalid
  Scenario: Tags, notes, relations, and edits are checked like everything else
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    When I run "trcli specimen tag <first> 'Two Words' ok ação"
    Then the exit code is 2
    And stderr contains "2 values are invalid"
    And stderr contains "1 to 50 characters of a-z, 0-9"
    And nothing was changed
    When I run "trcli specimen note <first> ' '"
    Then the exit code is 2
    And stderr contains "<text>"
    And nothing was changed
    When I run "trcli link add <first> <note> --relation '{long title}'"
    Then the exit code is 2
    And stderr contains "--relation"
    And stderr contains "1 to 50 characters"
    And nothing was changed
    When I run "trcli specimen edit <first> --title ''"
    Then the exit code is 2
    And nothing was changed
    When I run "trcli sample-note edit <note> --body ''"
    Then the exit code is 2
    And nothing was changed
    When I run "trcli specimen show 'not a name'"
    Then the exit code is 2
    And stderr contains "<ref>"
    When I run "trcli specimen rm 'not a name' --yes"
    Then the exit code is 2
    When I run "trcli specimen list --search rain --tag 'no good'"
    Then the exit code is 2

  @US3-07 @invalid
  Scenario: Two values that are each valid and together are not are reported with the rule
    When I run "trcli audit list --from 2026-10-08 --to 2026-10-01"
    Then the exit code is 2
    And stderr contains "--to"
    And stderr contains "is before the start (2026-10-08)"
    And stderr contains "an end on or after the start"
    When I run "trcli audit export --from 2026-10-08 --until 2026-10-01 --to audit.md"
    Then the exit code is 2
    And stderr contains "--until"
    When I run "trcli sample-note edit smp-0000 --lock --unlock"
    Then the exit code is 2
    And stderr contains "cannot be used with"

  @US3-08 @invalid
  Scenario: Input read from a file is checked by the same rules as typed input
    When I run "trcli config set output.page_size 0"
    Then the exit code is 2
    And stderr contains "a whole number from 1 to 1000"
    And nothing was changed
    Given the file ".trcli/config.toml" contains "output.page_size = 0\n"
    When I run "trcli specimen list"
    Then the exit code is 2
    And stderr contains "output.page_size"
    And stderr contains "config.toml"
    And stderr contains "a whole number from 1 to 1000"

  @US3-09 @invalid
  Scenario: An invalid command says what is wrong and shows how the command is used
    When I run "trcli workspace explode"
    Then the exit code is 2
    And stderr contains "explode"
    And stderr contains "Usage: trcli workspace"
    When I run "trcli config set output.color"
    Then the exit code is 2
    And stderr contains "<VALUE>"
    And stderr contains "Usage: trcli config set"
    When I run "trcli init --nmae 'Doctorate'"
    Then the exit code is 2
    And stderr contains "--nmae"
    And stderr contains "--name"
    When I run "trcli config get"
    Then the exit code is 2
    And stderr contains "<KEY>"
    When I run "trcli config unset"
    Then the exit code is 2
    When I run "trcli link list"
    Then the exit code is 2
    And stderr contains "<REF>"
    And nothing was changed

  @US3-10
  Scenario: Something unusual but allowed is warned about and never silently changed
    Given I ran "trcli specimen add --title 'Soil sample'" and remember the record as "first"
    When I run "trcli specimen add --title 'soil  SAMPLE'" and remember the record as "second"
    Then the exit code is 0
    And stderr contains "warning:"
    And stderr contains "another specimen, <first>, has the same title"
    When I run "trcli specimen show <second>"
    Then stdout contains "soil  SAMPLE"
    When I run "trcli specimen add --title 'Soil sample' --output json"
    Then stdout is JSON where "warnings.0.code" is "duplicate_suspected"
    And stdout is JSON where "data.name" is "Soil sample"

  @US3-11
  Scenario: Free text in any language and script is kept exactly as written
    When I run "trcli specimen add --title 'Ação — 東京 — مرحبا — Ünïcödé'" and remember the record as "first"
    And I run "trcli specimen show <first>"
    Then stdout contains "Ação — 東京 — مرحبا — Ünïcödé"
    When I run "trcli specimen note <first> 'Собрано под дождём'"
    And I run "trcli specimen show <first> --output json"
    Then stdout is JSON where "data.title" is "Ação — 東京 — مرحبا — Ünïcödé"
    And stdout is JSON where "data.notes.0.body" is "Собрано под дождём"

  @US3-12 @invalid
  Scenario: Invalid input ends in a way a program can tell from success and from other failures
    When I run "trcli specimen add --title '' --output json"
    Then the exit code is 2
    And stdout is JSON where "ok" is "false"
    And stdout is JSON where "error.code" is "validation_failed"
    And stdout is JSON where "error.details.0.field" is "--title"
    And stdout is JSON where "error.changed" is "false"
    When I run "trcli specimen show spc-zzzzzzzz --output json"
    Then the exit code is 3
    And stdout is JSON where "error.code" is "not_found"
    When I run "trcli specimen add --title 'Fine'"
    Then the exit code is 0
