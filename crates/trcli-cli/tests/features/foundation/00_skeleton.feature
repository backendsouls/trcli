# The first thing that must work: the tool starts, answers in both forms, and ends with
# an exit code that means something. (tasks.md, T023)
Feature: The skeleton of the tool

  Scenario: Running the tool with no arguments shows the short help and does not fail
    When I run "trcli"
    Then the exit code is 0
    And stdout contains "Usage: trcli"
    And stdout contains "workspace"
    And stdout contains "trcli init"

  Scenario: The version says which workspace format the tool reads and writes
    When I run "trcli --version"
    Then the exit code is 0
    And stdout contains "trcli 0.1.0"
    And stdout contains "workspace format 1"

  Scenario: An unknown command is rejected with the nearest command that exists
    When I run "trcli worksapce show"
    Then the exit code is 2
    And stderr contains "worksapce"
    And stderr contains "workspace"
    And stdout is empty

  Scenario: A command that needs a workspace explains itself outside one
    When I run "trcli workspace show"
    Then the exit code is 4
    And stderr contains "there is no workspace"
    And stderr contains "trcli init"
    And stdout is empty

  Scenario: The same failure in the structured form is one JSON document
    When I run "trcli workspace show --output json"
    Then the exit code is 4
    And stdout is JSON where "ok" is "false"
    And stdout is JSON where "error.code" is "no_workspace"
