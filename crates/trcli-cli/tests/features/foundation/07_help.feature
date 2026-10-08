# specs/000-foundation, User Story 7: Find out how to do anything.
Feature: Find out how to do anything

  @US7-01
  Scenario: Help with no further word lists the groups of commands and how to learn more
    When I run "trcli --help"
    Then the exit code is 0
    And stdout contains "init"
    And stdout contains "View, change, upgrade, and check the workspace"
    And stdout contains "View and change settings"
    And stdout contains "Look through, verify, and export the record of every change"
    And stdout contains "Run `trcli <command> --help`"
    And stdout contains "docs/usage/"
    When I run "trcli help"
    Then the exit code is 0
    And stdout contains "Usage: trcli"

  @US7-02
  Scenario: Help on any command shows its purpose, its options, and an example
    When I run "trcli workspace edit --help"
    Then the exit code is 0
    And stdout contains "Change the workspace's details; only what you name is changed"
    And stdout contains "--name <NAME>"
    And stdout contains "1 to 200 characters"
    And stdout contains "--researcher <NAME>"
    And stdout contains "Example:"
    And stdout contains "trcli workspace edit --name"
    And stdout contains "--output <FORMAT>"
    And stdout contains "[possible values: human, json]"
    When I run "trcli help config set"
    Then the exit code is 0
    And stdout contains "Example:"
    When I run "trcli specimen list --help"
    Then stdout contains "Example:"
    And stdout contains "[default: title]"

  @US7-03 @invalid
  Scenario: A mistyped command or option is answered with the nearest ones that exist
    When I run "trcli confg list"
    Then the exit code is 2
    And stderr contains "config"
    When I run "trcli config lst"
    Then the exit code is 2
    And stderr contains "list"
    When I run "trcli workspace show --outptu json"
    Then the exit code is 2
    And stderr contains "--output"

  @US7-04
  Scenario: Every group of commands names its usage guide
    When I run "trcli workspace --help"
    Then stdout contains "Guide: docs/usage/workspace.md"
    When I run "trcli config --help"
    Then stdout contains "Guide: docs/usage/config.md"
    When I run "trcli audit --help"
    Then stdout contains "Guide: docs/usage/audit.md"
    When I run "trcli link --help"
    Then stdout contains "Guide: docs/usage/records.md"

  # Every example of every guide is run by tests/usage.rs; this is one of them, verbatim
  # from docs/usage/workspace.md.
  @US7-05
  Scenario: An example of a usage guide behaves as the guide says
    When I run "trcli init --name 'Doctorate'"
    Then stdout contains 'Created workspace "Doctorate" in'
    When I run "trcli init --name 'Again'"
    Then the exit code is 4
    And stderr contains "error: a workspace already exists in"
    And stderr contains "Nothing was changed."

  @US7-06
  Scenario: The version is shown with the workspace format it reads and writes
    When I run "trcli --version"
    Then the exit code is 0
    And stdout contains "trcli 0.1.0 (workspace format 1)"

  @US7-07
  Scenario: Completion support is obtained for the researcher's shell
    When I run "trcli completions bash"
    Then the exit code is 0
    And stdout contains "_trcli"
    And stdout contains "workspace"
    And stdout contains "--output"
    When I run "trcli completions zsh"
    Then stdout contains "#compdef trcli"
    When I run "trcli completions fish"
    Then stdout contains "complete -c trcli"
    When I run "trcli completions powershell"
    Then stdout contains "Register-ArgumentCompleter"

  @US7-08
  Scenario: A message about a failure names the obvious next step
    When I run "trcli workspace show"
    Then stderr contains "Next: create one here with `trcli init --name <name>`"
    Given a workspace named "Lab"
    And the workspace is stored in format 0
    When I run "trcli workspace edit --name 'New'"
    Then stderr contains "Next: run `trcli workspace upgrade` first"

  @US7-09
  Scenario: In a new workspace the tool suggests the first steps
    Given a workspace named "Lab"
    When I run "trcli"
    Then the exit code is 0
    And stdout contains "First steps in this workspace:"
    And stdout contains "trcli workspace show"
    And stdout contains "trcli workspace edit --researcher"
    Given I ran "trcli specimen add --title 'First'"
    When I run "trcli"
    Then stdout does not contain "First steps in this workspace:"

  @US7-09
  Scenario: Outside a workspace the tool says how to create one
    When I run "trcli"
    Then the exit code is 0
    And stdout contains "There is no workspace here yet."
    And stdout contains "trcli init --name"

  @US7-10
  Scenario: Help and messages use the same words for the same things
    Given a workspace named "Lab"
    When I run "trcli specimen show --help"
    Then stdout contains "Short name of the record, or a unique beginning of it"
    When I run "trcli specimen show spc-zzzzzzzz"
    Then stderr contains "no record matches"
    When I run "trcli specimen show 'two words'"
    Then stderr contains "is not a short name or the beginning of one"
    When I run "trcli workspace --help"
    Then stdout contains "workspace"
    When I run "trcli workspace show"
    Then stdout contains "Workspace"
    When I run "trcli audit --help"
    Then stdout contains "trail"
    When I run "trcli audit verify"
    Then stdout contains "audit trail"
