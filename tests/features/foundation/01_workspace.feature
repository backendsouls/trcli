# specs/000-foundation, User Story 1: Create and find a workspace.
# One scenario per acceptance scenario; the tag names the scenario it automates.
Feature: Create and find a workspace

  @US1-01
  Scenario: A workspace is created with a name and reported with where it is
    When I run "trcli init --name 'Doctorate' --description 'Thesis on soil microbes'"
    Then the exit code is 0
    And stdout contains 'Created workspace "Doctorate" in {home}/work/.trcli'
    And the file ".trcli/trcli.db" exists
    And the file ".trcli/config.toml" exists
    And the file ".trcli/audit.head" exists

  @US1-02
  Scenario: A workspace is found from any directory beneath it
    Given a workspace named "Doctorate"
    And I am in the directory "work/chapters/three/figures"
    When I run "trcli workspace show"
    Then the exit code is 0
    And stdout contains "Doctorate"
    And stdout contains "{home}/work"

  @US1-03 @invalid
  Scenario: Outside any workspace the tool explains and creates nothing
    Given I am in the directory "elsewhere"
    When I run "trcli workspace show"
    Then the exit code is 4
    And stderr contains "there is no workspace"
    And stderr contains "trcli init --name"
    And stderr contains "--workspace"
    And stderr contains "Nothing was changed."
    And the file ".trcli" does not exist
    And nothing was changed

  @US1-04 @invalid
  Scenario: A second workspace is not created where one exists
    Given a workspace named "Doctorate"
    When I run "trcli init --name 'Again'"
    Then the exit code is 4
    And stderr contains "a workspace already exists in {home}/work/.trcli"
    And nothing was changed
    When I run "trcli workspace show"
    Then stdout contains "Doctorate"

  @US1-05
  Scenario: Viewing a workspace shows its details and its records by kind
    Given I ran "trcli init --name 'Doctorate' --description 'Thesis on soil microbes'"
    And I ran "trcli specimen add --title 'First'"
    When I run "trcli workspace show"
    Then the exit code is 0
    And stdout contains 'Workspace "Doctorate"'
    And stdout contains "Thesis on soil microbes"
    And stdout contains "{home}/work"
    And stdout contains "Format       1"
    And stdout contains "specimen     1"
    And stdout contains "sample-note  0"

  @US1-06
  Scenario: The name, the description, and the researcher's name can be changed
    Given a workspace named "Doctorate"
    When I run "trcli workspace edit --name 'Doctorate 2026' --description 'Second year' --researcher 'Ana Souza'"
    Then the exit code is 0
    And stdout contains 'Updated workspace "Doctorate 2026"'
    When I run "trcli workspace show"
    Then stdout contains "Doctorate 2026"
    And stdout contains "Second year"
    When I run "trcli config get researcher.name"
    Then stdout contains "Ana Souza"

  @US1-07
  Scenario: Two workspaces are entirely separate
    Given a workspace named "First workspace"
    And I am in the directory "other"
    And I ran "trcli init --name 'Second workspace'"
    And I ran "trcli specimen add --title 'Only in the second'"
    When I go to the directory "work"
    And I run "trcli specimen list"
    Then the exit code is 0
    And stdout does not contain "Only in the second"
    When I run "trcli workspace show"
    Then stdout contains "First workspace"
    And stdout contains "specimen     0"

  @US1-08
  Scenario: A workspace elsewhere is named for one command or for a session
    Given a workspace named "Doctorate"
    And I am in the directory "elsewhere"
    When I run "trcli --workspace {home}/work workspace show"
    Then the exit code is 0
    And stdout contains "Doctorate"
    When I run "trcli workspace show"
    Then the exit code is 4
    Given the environment variable TRCLI_WORKSPACE is "{home}/work"
    When I run "trcli workspace show"
    Then the exit code is 0
    And stdout contains "Doctorate"

  @US1-08 @invalid
  Scenario: A workspace named for a command must be there
    Given a workspace named "Doctorate"
    When I run "trcli --workspace {home}/nowhere workspace show"
    Then the exit code is 4
    And stderr contains "there is no workspace in {home}/nowhere"
    And nothing was changed

  @US1-09
  Scenario: A default workspace is used when none is found, and the tool says so
    Given a workspace named "Doctorate"
    And I ran "trcli config set --user default_workspace {home}/work"
    And I am in the directory "elsewhere"
    When I run "trcli workspace show"
    Then the exit code is 0
    And stdout contains "Doctorate"
    And stderr contains "Using your default workspace in {home}/work"

  @US1-10
  Scenario: Of nested workspaces the nearest is used
    Given a workspace named "Outer"
    And I am in the directory "work/inner"
    And I ran "trcli init --name 'Inner'"
    And I am in the directory "work/inner/deep"
    When I run "trcli workspace show"
    Then stdout contains 'Workspace "Inner"'
    When I go to the directory "work/beside"
    And I run "trcli workspace show"
    Then stdout contains 'Workspace "Outer"'

  @US1-11 @invalid
  Scenario: A workspace made by a newer version is not changed
    Given a workspace named "Doctorate"
    And the workspace is stored in format 99
    When I run "trcli workspace edit --name 'Changed'"
    Then the exit code is 4
    And stderr contains "newer version of trcli"
    And stderr contains "format 99"
    And nothing was changed

  @US1-12
  Scenario: An older workspace is upgraded only when asked, and a copy is kept
    Given a workspace named "Doctorate"
    And the workspace is stored in format 0
    When I run "trcli workspace show"
    Then the exit code is 0
    And stderr contains "needs an upgrade"
    When I run "trcli specimen add --title 'Too early'"
    Then the exit code is 4
    And stderr contains "trcli workspace upgrade"
    And nothing was changed
    When I run "trcli workspace upgrade --check"
    Then stdout contains "An upgrade is needed"
    And nothing was changed
    When I run "trcli workspace upgrade"
    Then the exit code is 0
    And stdout contains "Upgraded the workspace from format 0 to format 1"
    And the directory ".trcli/backups" holds 1 file
    When I run "trcli specimen add --title 'Now it works'"
    Then the exit code is 0
    When I run "trcli audit verify"
    Then the exit code is 0

  @US1-13 @invalid
  Scenario: A damaged workspace is reported with what and where, and is not overwritten
    Given a workspace named "Doctorate"
    And the workspace's database has been damaged
    When I run "trcli workspace show"
    Then the exit code is 4
    And stderr contains "the workspace is damaged"
    And stderr contains "trcli.db"
    And stderr contains "backups"
    And the file ".trcli/trcli.db" contains "this is no longer a database"

  @US1-14 @invalid
  Scenario: A workspace is not created without a name
    When I run "trcli init"
    Then the exit code is 2
    And stderr contains "--name"
    And stderr contains "is required"
    And the file ".trcli" does not exist
    When I run "trcli init --name ''"
    Then the exit code is 2
    And stderr contains "must not be empty"
    And stderr contains "1 to 200 characters"
    And the file ".trcli" does not exist

  @US1-14 @invalid
  Scenario: A workspace is not created with a description that is too long or a directory that is not there
    When I run "trcli init {home}/missing --name 'Doctorate' --description '{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}{long title}'"
    Then the exit code is 2
    And stderr contains "2 values are invalid"
    And stderr contains "--description"
    And stderr contains "<dir>"
    And stderr contains "an existing, writable directory"

  @US1-14 @invalid @unix
  Scenario: A workspace is not created in a directory that cannot be written to
    Given the directory "locked" cannot be written to
    And I am in the directory "locked"
    When I run "trcli init --name 'Doctorate'"
    Then the exit code is 2
    And stderr contains "<dir>"
    And stderr contains "is not a directory that can be written to"
    And the file ".trcli" does not exist
    And the directory "locked" can be written to again

  # FR-005: nothing inside a workspace depends on where it is.
  Scenario: A workspace keeps working after its directory is renamed
    Given a workspace named "Doctorate"
    And I ran "trcli specimen add --title 'Before the move'"
    When the directory "work" is renamed to "moved"
    And I run "trcli specimen list"
    Then the exit code is 0
    And stdout contains "Before the move"
    When I run "trcli audit verify"
    Then the exit code is 0
