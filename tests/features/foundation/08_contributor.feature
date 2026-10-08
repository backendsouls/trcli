# specs/000-foundation, User Story 8: Add a feature without breaking the whole.
#
# These are the contributor scenarios that can be shown through the binary. The others
# are automated by structural tests: scenario 3 by tests/scenario_coverage.rs, 4 by
# tests/invalid_input_gate.rs, 5 by the documentation lints and tests/help_examples.rs,
# 6 by the CI matrix, 7 by tests/layering.rs, and 9 by the upgrade tests.
Feature: Add a feature without breaking the whole

  Background:
    Given a workspace named "Lab"

  @US8-01
  Scenario Outline: A kind that only declares what is particular to it has everything records share
    When I run "trcli <noun> add <field> 'Declared once'" and remember the record as "one"
    And I run "trcli <noun> add <field> 'Another'" and remember the record as "other"
    Then stdout contains "<prefix>-"
    When I run "trcli <noun> tag {one} shared"
    Then the exit code is 0
    When I run "trcli <noun> note {one} 'A note for any kind'"
    Then the exit code is 0
    When I run "trcli link add {one} {other}"
    Then the exit code is 0
    When I run "trcli <noun> list --tag shared --search declared"
    Then stdout contains "Declared once"
    And stdout does not contain "Another"
    When I run "trcli <noun> show {one} --output json"
    Then stdout is JSON where "data.kind" is "<noun>"
    And stdout is JSON where "data.tags.0" is "shared"
    And stdout is JSON where "data.notes.0.body" is "A note for any kind"
    And stdout is JSON where "data.links.0.handle" is "{other}"
    When I run "trcli <noun> rm {one}"
    Then the exit code is 5
    When I run "trcli <noun> rm {one} --yes"
    Then the exit code is 0
    When I run "trcli audit list --kind <noun> --output json"
    Then stdout is JSON where "data.items.0.action" is "delete"
    And stdout is JSON where "data.items.0.display_name" is "Declared once"
    When I run "trcli workspace show --output json"
    Then stdout is JSON where "data.records.<index>.kind" is "<noun>"
    And stdout is JSON where "data.records.<index>.count" is "1"

    Examples:
      | noun        | field   | prefix | index |
      | specimen    | --title | spc    | 0     |
      | sample-note | --body  | smp    | 1     |

  @US8-02
  Scenario Outline: A new command follows the one grammar, the shared options, both forms, and the exit codes
    When I run "trcli <noun> list --help"
    Then stdout contains "Usage: trcli <noun> list [OPTIONS]"
    And stdout contains "--workspace"
    And stdout contains "--output"
    And stdout contains "--no-input"
    When I run "trcli <noun> add <field> 'Follows the rules' --output json"
    Then the exit code is 0
    And stdout is JSON where "ok" is "true"
    And stdout is JSON where "data.kind" is "<noun>"
    When I run "trcli <noun> add <field> ''"
    Then the exit code is 2
    And stderr contains "Nothing was changed."
    When I run "trcli <noun> show <prefix>-zzzzzzzz"
    Then the exit code is 3
    When I run "trcli --workspace {home}/nowhere <noun> list"
    Then the exit code is 4

    Examples:
      | noun        | field   | prefix |
      | specimen    | --title | spc    |
      | sample-note | --body  | smp    |

  @US8-08
  Scenario: The clock and the identifiers can be replaced without changing a feature
    Given the environment variable TRCLI_TEST_NOW is "2031-01-02T03:04:05Z"
    And the environment variable TRCLI_TEST_ID_SEED is "7"
    When I run "trcli specimen add --title 'At a fixed time'" and remember the record as "fixed"
    And I run "trcli specimen show <fixed>"
    Then stdout contains "2031-01-02 03:04 UTC"
    Given I am in the directory "other"
    And I ran "trcli init --name 'Other'"
    When I run "trcli specimen add --title 'With the same seed'"
    Then stdout contains "<fixed>"

  @US8-10
  Scenario: Two features refer to each other's records only by kind and name
    Given I ran "trcli specimen add --title 'A specimen'" and remember the record as "specimen"
    And I ran "trcli sample-note add --body 'A sample note'" and remember the record as "note"
    When I run "trcli link add <specimen> <note> --relation describes"
    And I run "trcli specimen show <specimen> --output json"
    Then stdout is JSON where "data.links.0.kind" is "sample-note"
    And stdout is JSON where "data.links.0.handle" is "<note>"
    And stdout is JSON where "data.links.0.name" is "A sample note"
    And stdout is JSON where "data.links.0.relation" is "describes"
    When I run "trcli sample-note rm <note> --yes"
    And I run "trcli specimen show <specimen> --output json"
    Then stdout is JSON where "data.links" is "[]"
