# specs/000-foundation, User Story 5: Set the tool up my way.
Feature: Set the tool up my way

  Background:
    Given a workspace named "Lab"

  @US5-01
  Scenario: A personal setting applies in every workspace
    Given I ran "trcli config set --user output.page_size 20"
    And I am in the directory "other"
    And I ran "trcli init --name 'Other'"
    When I run "trcli config get output.page_size"
    Then stdout contains "Value    20"
    And stdout contains "user file"

  @US5-02
  Scenario: A workspace setting applies there only and takes the place of the personal value
    Given I ran "trcli config set --user output.page_size 20"
    And I ran "trcli config set output.page_size 5"
    When I run "trcli config get output.page_size"
    Then stdout contains "Value    5"
    And stdout contains "workspace file"
    Given I am in the directory "other"
    And I ran "trcli init --name 'Other'"
    When I run "trcli config get output.page_size"
    Then stdout contains "Value    20"

  @US5-03
  Scenario: An override for a session or a command wins and stores nothing
    Given I ran "trcli config set output.format human"
    And the environment variable TRCLI_OUTPUT_FORMAT is "json"
    When I run "trcli workspace show"
    Then stdout is JSON where "data.name" is "Lab"
    And nothing was changed
    When I run "trcli workspace show --output human"
    Then stdout contains 'Workspace "Lab"'
    And nothing was changed

  @US5-04
  Scenario: Listing settings shows every value in effect and where it comes from
    Given I ran "trcli config set --user output.symbols ascii"
    And I ran "trcli config set output.page_size 5"
    And the environment variable TRCLI_STORAGE_BUSY_TIMEOUT_MS is "250"
    And the environment variable COLUMNS is "200"
    When I run "trcli config list --color never"
    Then the exit code is 0
    And stdout contains "output.symbols"
    And stdout contains "user file {home}"
    And stdout contains "workspace file {home}/work/.trcli/config.toml"
    And stdout contains "environment"
    And stdout contains "command line"
    And stdout contains "default"
    When I run "trcli config list --output json"
    Then stdout is JSON where "data.total" is "16"

  @US5-05
  Scenario: Asking about a setting shows its meaning, allowed values, and default
    When I run "trcli config get output.color"
    Then the exit code is 0
    And stdout contains "Coloured output."
    And stdout contains "one of: auto, always, never"
    And stdout contains "Default  auto"
    And stdout contains "Scope    both"
    When I run "trcli config path"
    Then stdout contains "{home}/work/.trcli/config.toml"

  @US5-06 @invalid
  Scenario: An invalid value and a setting that does not exist are refused with what is allowed
    When I run "trcli config set output.page_size 0"
    Then the exit code is 2
    And stderr contains "a whole number from 1 to 1000"
    And nothing was changed
    When I run "trcli config set output.color sometimes"
    Then the exit code is 2
    And stderr contains "auto, always, never"
    When I run "trcli config set no.such.setting 1"
    Then the exit code is 2
    And stderr contains "is not a setting"
    And nothing was changed
    When I run "trcli config set output.colour never"
    Then the exit code is 2
    And stderr contains "output.color"
    When I run "trcli config get no.such.setting"
    Then the exit code is 2
    When I run "trcli config unset no.such.setting"
    Then the exit code is 2
    When I run "trcli config set default_workspace {home}/work"
    Then the exit code is 2
    And stderr contains "can only be set for a user"
    And stderr contains "--user"
    When I run "trcli config set --user telemetry.enabled false"
    Then the exit code is 2
    And stderr contains "can only be set for a workspace"

  @US5-07 @invalid
  Scenario: A settings file with an invalid value or an unknown setting stops every command
    Given the file ".trcli/config.toml" contains 'output.colour = "never"\n'
    When I run "trcli workspace show"
    Then the exit code is 2
    And stderr contains "output.colour"
    And stderr contains "{home}/work/.trcli/config.toml"
    And stderr contains "is not a setting"
    When I run "trcli specimen list"
    Then the exit code is 2
    Given the file ".trcli/config.toml" contains "this is = not [toml"
    When I run "trcli workspace show"
    Then the exit code is 2
    And stderr contains "config.toml is not valid"

  @US5-08
  Scenario: A setting found in a place it does not belong to is ignored with a warning
    Given my settings file contains "telemetry.enabled = false\n"
    When I run "trcli telemetry status"
    Then the exit code is 0
    And stdout contains "Telemetry is on"
    And stderr contains "warning:"
    And stderr contains "setting `telemetry.enabled`"
    And stderr contains "is ignored"

  @US5-09
  Scenario: Removing a value lets the next one in order apply again
    Given I ran "trcli config set --user output.page_size 20"
    And I ran "trcli config set output.page_size 5"
    When I run "trcli config unset output.page_size"
    Then stdout contains "Removed output.page_size from this workspace"
    When I run "trcli config get output.page_size"
    Then stdout contains "Value    20"
    When I run "trcli config unset --user output.page_size"
    And I run "trcli config get output.page_size"
    Then stdout contains "Value    50"
    And stdout contains "Source   default"
    When I run "trcli config unset output.page_size"
    Then the exit code is 0
    And stdout contains "nothing to remove"

  @US5-10
  Scenario: With no settings at all every setting has a default and nothing needs configuring
    When I run "trcli config list --output json"
    Then the exit code is 0
    And stdout is JSON where "data.items.1.key" is "storage.path"
    And stdout is JSON where "data.items.1.source" is "default"
    And stdout is JSON where "data.items.6.value" is "50"
    When I run "trcli specimen add --title 'Works out of the box'"
    Then the exit code is 0

  @US5-11
  Scenario: Nothing secret is among the settings
    When I run "trcli config list"
    Then stdout does not contain "password"
    And stdout does not contain "secret"
    And stdout does not contain "token"
    And stdout does not contain "credential"
