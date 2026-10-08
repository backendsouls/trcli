# specs/000-foundation, User Story 4: Get answers fit for people and for programs.
Feature: Get answers fit for people and for programs

  Background:
    Given a workspace named "Lab"
    And I ran "trcli specimen add --title 'Attention is all you need, and other long titles of things'" and remember the record as "first"
    And I ran "trcli specimen add --title 'Second'" and remember the record as "second"

  @US4-01
  Scenario: At a terminal the answer is laid out, sized to the terminal, and coloured
    When I run "trcli specimen list --color always"
    Then the exit code is 0
    And stdout has colour codes
    And stdout contains "HANDLE"
    And stdout contains "<first>"
    Given the environment variable COLUMNS is "50"
    When I run "trcli specimen list"
    Then no line of stdout is wider than 50 columns
    And stdout contains "Attention is"
    And stdout contains "…"
    And stdout contains "<first>"

  @US4-02
  Scenario: Output that goes to a file or another program has no colour and no decoration
    When I run "trcli specimen list"
    Then the output has no colour codes
    When I run "trcli specimen show spc"
    Then the exit code is 3
    And the output has no colour codes

  @US4-03
  Scenario: The structured form holds the whole result with the same content as the form for people
    When I run "trcli specimen show <second> --output json"
    Then the exit code is 0
    And stdout is one JSON document
    And stdout is JSON where "ok" is "true"
    And stdout is JSON where "data.handle" is "<second>"
    And stdout is JSON where "data.kind" is "specimen"
    And stdout is JSON where "data.title" is "Second"
    And stdout is JSON where "data.created_at" is "2026-10-08T14:00:00Z"
    And stdout is JSON where "data.tags" is "[]"
    And the output has no colour codes
    When I run "trcli specimen show <second>"
    Then stdout contains "<second>"
    And stdout contains "specimen"
    And stdout contains "Second"
    And stdout contains "2026-10-08 14:00"
    When I run "trcli specimen list --output json --color always"
    Then stdout is JSON where "data.total" is "2"
    And the output has no colour codes

  @US4-04
  Scenario: Results go to one stream and messages to another
    When I run "trcli specimen list"
    Then stdout contains "Second"
    And stderr is empty
    When I run "trcli specimen show spc-zzzzzzzz"
    Then stdout is empty
    And stderr contains "no record matches"
    When I run "trcli specimen list --limit 1"
    Then stdout does not contain "Showing"
    And stderr contains "Showing 1 of 2"
    When I run "trcli specimen list --limit 1 --quiet"
    Then stderr is empty

  @US4-05 @invalid
  Scenario: Each kind of failure ends differently
    When I run "trcli specimen add --title ''"
    Then the exit code is 2
    When I run "trcli specimen show spc-zzzzzzzz"
    Then the exit code is 3
    When I run "trcli --workspace {home}/nowhere specimen list"
    Then the exit code is 4
    When I run "trcli specimen rm <second>"
    Then the exit code is 5
    When I run "trcli audit export --to {home}/no-such-directory/audit.md"
    Then the exit code is 7
    And stderr contains "could not be written"
    When audit entry 2 is altered outside the tool
    And I run "trcli audit verify"
    Then the exit code is 6

  @US4-06 @invalid
  Scenario: A failure is described in the structured form too, with a stable name
    When I run "trcli specimen rm <second> --output json"
    Then the exit code is 5
    And stdout is one JSON document
    And stdout is JSON where "ok" is "false"
    And stdout is JSON where "error.code" is "confirmation_required"
    And stdout is JSON where "error.next_step" contains "--yes"
    And stderr contains "error:"
    When I run "trcli nosuchcommand --output json"
    Then the exit code is 2
    And stdout is JSON where "error.code" is "usage"

  # At a terminal the indication is drawn after a moment and erased when done; that half
  # is covered by the unit tests of `progress.rs`, since a test has no terminal.
  @US4-07
  Scenario: Long work shows nothing when nobody is watching, and still completes
    When I run "trcli specimen slow --seconds 1"
    Then the exit code is 0
    And stdout contains "Held the workspace for 1 s"
    And stderr is empty

  @US4-08 @unix
  Scenario: Interrupted work stops promptly, leaves the workspace valid, and says so
    When I run "trcli specimen slow --seconds 30 --title 'Never added'" and stop it with INT once it holds the workspace
    Then the exit code is 130
    And stderr contains "interrupted before the command finished"
    And stderr contains "Nothing was changed."
    And nothing was changed
    When I run "trcli specimen list"
    Then stdout does not contain "Never added"
    When I run "trcli workspace check"
    Then the exit code is 0

  @US4-09
  Scenario: Preferences for colour and plain symbols are followed, and so are the usual conventions
    Given I ran "trcli config set output.color always"
    When I run "trcli specimen list"
    Then stdout has colour codes
    Given the environment variable NO_COLOR is "1"
    When I run "trcli specimen list"
    Then the output has no colour codes
    When I run "trcli specimen list --color always"
    Then stdout has colour codes
    When I run "trcli link add <first> <second> --color never"
    Then stdout contains "⟷"
    Given I ran "trcli config set output.symbols ascii"
    When I run "trcli link list <first> --color never"
    Then stdout contains "<->"
    And stdout does not contain "⟷"

  @US4-09
  Scenario: A theme style can be changed
    Given I ran "trcli config set theme.handle 'red bold'"
    When I run "trcli specimen list --color always"
    Then the exit code is 0
    And stdout has colour codes
    When I run "trcli config set theme.handle crimson"
    Then the exit code is 2
    And stderr contains "a colour (black, red, green"

  @US4-10 @invalid
  Scenario: A message about a problem says what happened, that nothing changed, and what to do next
    When I run "trcli specimen show spc"
    Then the exit code is 3
    And stderr contains "error: `spc` matches 2 records"
    And stderr contains "Nothing was changed."
    And stderr contains "Next: type more of the short name"

  @US4-11
  Scenario: A long listing is limited to a stated number with a count of the rest
    Given I ran "trcli specimen add --title 'Third'"
    And I ran "trcli config set output.page_size 2"
    When I run "trcli specimen list"
    Then stderr contains "Showing 2 of 3. Use --limit <n> to see more."
    When I run "trcli specimen list --limit 3"
    Then stdout contains "Third"
    And stderr is empty
    When I run "trcli specimen list --output json"
    Then stdout is JSON where "data.total" is "3"

  @US4-12
  Scenario: Dates and times are shown in one unambiguous form
    When I run "trcli specimen show <second>"
    Then stdout contains "Created  2026-10-08 14:00 UTC"
    When I run "trcli specimen list"
    Then stdout contains "2026-10-08 14:00"
    When I run "trcli audit list"
    Then stdout contains "2026-10-08 14:00"
    When I run "trcli audit list --output json"
    Then stdout is JSON where "data.items.0.at" is "2026-10-08T14:00:00Z"

  # Spec, edge cases: two commands at the same moment in the same workspace.
  Scenario: A second command waits for the first and then completes
    When I run "trcli specimen add --title 'Meanwhile'" while "trcli specimen slow --seconds 2" holds the workspace
    Then the exit code is 0
    When I run "trcli audit verify"
    Then the exit code is 0

  @invalid
  Scenario: A second command that cannot wait long enough says the workspace is busy
    Given the environment variable TRCLI_STORAGE_BUSY_TIMEOUT_MS is "100"
    When I run "trcli specimen add --title 'Meanwhile'" while "trcli specimen slow --seconds 3" holds the workspace
    Then the exit code is 4
    And stderr contains "the workspace is busy"
    And stderr contains "run this one again"
    When I run "trcli specimen list"
    Then stdout does not contain "Meanwhile"
