# specs/000-foundation, User Story 6: Know everything that happened.
Feature: Know everything that happened

  Background:
    Given a workspace named "Lab"

  @US6-01
  Scenario: Every creation, change, and deletion is recorded with who, when, and what changed
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli specimen edit <first> --title 'Renamed'"
    And I ran "trcli specimen rm <first> --yes"
    And the environment variable COLUMNS is "200"
    When I run "trcli audit list"
    Then the exit code is 0
    And stdout contains "create"
    And stdout contains "update"
    And stdout contains "delete"
    And stdout contains "ana"
    And stdout contains "2026-10-08 14:00"
    And stdout contains "title: First → Renamed"
    And stdout contains "<first>"
    When I run "trcli audit list --output json"
    Then stdout is JSON where "data.total" is "4"
    And stdout is JSON where "data.items.1.action" is "update"
    And stdout is JSON where "data.items.1.changes.0.before" is "First"
    And stdout is JSON where "data.items.1.changes.0.after" is "Renamed"
    And stdout is JSON where "data.items.1.actor" is "ana"

  @US6-02 @invalid
  Scenario: A change that fails or is refused leaves neither the change nor an entry
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    Then the workspace has 2 audit entries
    When I run "trcli specimen add --title ''"
    Then the exit code is 2
    And the workspace has 2 audit entries
    When I run "trcli specimen rm <first> --no-input"
    Then the exit code is 5
    And the workspace has 2 audit entries
    And nothing was changed

  @US6-03
  Scenario: The trail is filtered by item, kind, actor, action, and dates, newest first
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    And I ran "trcli workspace edit --researcher 'Ana Souza'"
    And I ran "trcli specimen tag <first> field-work"
    And the environment variable COLUMNS is "200"
    When I run "trcli audit list --record <first>"
    Then stdout lists "tag" before "create"
    And stdout does not contain "<note>"
    When I run "trcli audit list --kind sample-note"
    Then stdout contains "<note>"
    And stdout does not contain "<first>"
    When I run "trcli audit list --actor 'Ana Souza'"
    Then stdout contains "field-work"
    And stdout does not contain "A note"
    When I run "trcli audit list --action create --output json"
    Then stdout is JSON where "data.total" is "3"
    When I run "trcli audit list --from 2026-10-08 --to 2026-10-08 --output json"
    Then stdout is JSON where "data.total" is "5"
    When I run "trcli audit list --from 2026-10-09 --output json"
    Then stdout is JSON where "data.total" is "0"
    When I run "trcli audit list --until 2026-10-07"
    Then stdout is empty
    And stderr contains "No audit entries match."
    When I run "trcli audit list --limit 2"
    Then stderr contains "Showing 2 of 5"

  @US6-04
  Scenario: An entry about a deleted record still says what the record was called
    Given I ran "trcli specimen add --title 'Short-lived'" and remember the record as "first"
    And I ran "trcli specimen rm <first> --yes"
    When I run "trcli audit list --action delete"
    Then stdout contains '<first> "Short-lived"'
    When I run "trcli specimen show <first>"
    Then the exit code is 3

  @US6-05
  Scenario: An entry altered outside the tool is detected, with where
    Given I ran "trcli specimen add --title 'First'"
    And I ran "trcli specimen add --title 'Second'"
    When I run "trcli audit verify"
    Then the exit code is 0
    And stdout contains "The audit trail is intact: 3 entries verified."
    When audit entry 2 is altered outside the tool
    And I run "trcli audit verify"
    Then the exit code is 6
    And stderr contains "the audit trail has been tampered with"
    And stderr contains "entry 2 was altered"
    When I run "trcli workspace check"
    Then the exit code is 6
    When I run "trcli audit verify --output json"
    Then stdout is JSON where "error.code" is "check_failed"

  @US6-05
  Scenario: An entry removed from the end outside the tool is detected
    Given I ran "trcli specimen add --title 'First'"
    And I ran "trcli specimen add --title 'Second'"
    When the last audit entry is removed outside the tool
    And I run "trcli audit verify"
    Then the exit code is 6
    And stderr contains "the trail was shortened: it should reach entry 3"

  @US6-06 @invalid
  Scenario: There is no way to change or remove an entry
    When I run "trcli audit rm 1"
    Then the exit code is 2
    When I run "trcli audit edit 1"
    Then the exit code is 2
    When I run "trcli audit add"
    Then the exit code is 2
    When I run "trcli audit --help"
    Then stdout does not contain "  rm "
    And stdout does not contain "  edit "
    And stdout does not contain "  add "

  @US6-07
  Scenario: A range of the trail is exported as a report that can be handed over
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    When I run "trcli audit export --to audit.md"
    Then the exit code is 0
    And stdout contains "Exported 2 audit entries to audit.md (markdown)"
    And the file "audit.md" contains '# Audit trail of "Lab"'
    And the file "audit.md" contains "<first>"
    When I run "trcli audit export --action create --to created.csv --format csv"
    Then the file "created.csv" contains "sequence,at,actor,action"
    And the file "created.csv" contains "specimen,<first>,First"
    When I run "trcli audit export --kind specimen --actor ana --from 2026-10-01 --until 2026-10-31 --to october.json --format json"
    Then the exit code is 0
    And the file "october.json" contains '"action": "create"'
    When I run "trcli audit list --action export --output json"
    Then stdout is JSON where "data.total" is "3"

  @US6-07 @invalid
  Scenario: An export does not write over a file without being told to
    Given the file "audit.md" contains "keep me"
    When I run "trcli audit export --to audit.md"
    Then the exit code is 5
    And the file "audit.md" contains "keep me"
    And nothing was changed
    When I run "trcli audit export --to audit.md --yes"
    Then the exit code is 0
    And the file "audit.md" contains "# Audit trail"
    When I run "trcli audit export"
    Then the exit code is 2
    And stderr contains "--to"
    And stderr contains "is required"

  @US6-08
  Scenario: While telemetry is on, the commands used are recorded locally and can be viewed
    Given I ran "trcli workspace show"
    And I ran "trcli workspace show"
    And I ran "trcli specimen show spc-zzzzzzzz"
    When I run "trcli telemetry show"
    Then the exit code is 0
    And stdout contains "Telemetry is on"
    And stdout contains "workspace show  2     2"
    And stdout contains "specimen show   1     0"
    When I run "trcli telemetry show --output json"
    Then stdout is JSON where "data.enabled" is "true"
    And stdout is JSON where "data.items.0.command" is "workspace show"

  @US6-09
  Scenario: Turning telemetry off stops it while the trail continues unchanged
    When I run "trcli telemetry off"
    Then the exit code is 0
    Given I ran "trcli workspace show"
    And I ran "trcli specimen add --title 'Still audited'"
    When I run "trcli telemetry show --output json"
    Then stdout is JSON where "data.enabled" is "false"
    And stdout is JSON where "data.total" is "1"
    And stdout is JSON where "data.items.0.command" is "telemetry off"
    When I run "trcli audit list"
    Then stdout contains "Still audited"
    When I run "trcli telemetry on"
    And I run "trcli telemetry status"
    Then stdout contains "Telemetry is on"

  @US6-10
  Scenario: Telemetry stays in the workspace and is never sent anywhere
    When I run "trcli telemetry status"
    Then stdout contains "kept in this workspace only and is never sent anywhere"
    When I run "trcli config get telemetry.enabled"
    Then stdout contains "Nothing is ever transmitted."
    And stdout contains "Scope    workspace"

  @US6-11
  Scenario: Nothing typed as a value is ever recorded as telemetry
    Given I ran "trcli specimen add --title 'A very private title'"
    And I ran "trcli workspace edit --description 'A very private description'"
    When I run "trcli telemetry show --output json"
    Then stdout does not contain "private"
    And stdout does not contain "--title"
    And stdout is JSON where "data.items.0.command" is "specimen add"

  @US6-12
  Scenario: Who acted is the name the researcher set, or otherwise their name on the machine
    Given I ran "trcli specimen add --title 'By the system name'"
    And I ran "trcli workspace edit --researcher 'Ana Souza'"
    And I ran "trcli specimen add --title 'By the name that was set'"
    When I run "trcli audit list --output json"
    Then stdout is JSON where "data.items.0.actor" is "Ana Souza"
    And stdout is JSON where "data.items.2.actor" is "ana"

  # SC-008: a change is wholly present or wholly absent, even when the process dies.
  @unix
  Scenario: A command killed in the middle of a change leaves the workspace as it was
    When I run "trcli specimen slow --seconds 30 --title 'Never added'" and stop it with KILL once it holds the workspace
    Then nothing was changed
    When I run "trcli audit verify"
    Then the exit code is 0
    When I run "trcli workspace check"
    Then the exit code is 0
    When I run "trcli specimen add --title 'Life goes on'"
    Then the exit code is 0
