# specs/000-foundation, User Story 2: Work with any record the same way.
# Run against the two sample kinds, `specimen` and `sample-note` (build feature
# `sample-kind`), which get all of this by registering a descriptor.
Feature: Work with any record the same way

  Background:
    Given a workspace named "Lab"

  @US2-01
  Scenario: A record receives a short name that says its kind, is unique, and never changes
    When I run "trcli specimen add --title 'First'" and remember the record as "first"
    And I run "trcli sample-note add --body 'A note'" and remember the record as "note"
    Then stdout contains "smp-"
    When I run "trcli specimen edit <first> --title 'Renamed'"
    Then the exit code is 0
    And stdout contains "<first>"
    When I run "trcli specimen show <first>"
    Then the exit code is 0
    And stdout contains "spc-"
    And stdout contains "Renamed"

  @US2-02
  Scenario: The beginning of a short name is enough, in any letter case
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'"
    When I run "trcli specimen show <first:6>"
    Then the exit code is 0
    And stdout contains "First"
    When I run "trcli specimen show SPC"
    Then the exit code is 0
    And stdout contains "First"

  @US2-03 @invalid
  Scenario: A beginning that matches several records lists them and changes nothing
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli specimen add --title 'Second'" and remember the record as "second"
    When I run "trcli specimen tag spc field-work"
    Then the exit code is 3
    And stderr contains "`spc` matches 2 records"
    And stderr contains '<first> "First"'
    And stderr contains '<second> "Second"'
    And nothing was changed

  @US2-04 @invalid
  Scenario: A short name that matches nothing suggests close ones
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    When I run "trcli specimen show <first:7>0"
    Then the exit code is 3
    And stderr contains "no record matches"
    And stderr contains "did you mean <first>?"
    When I run "trcli specimen show spc-zzzzzzzz"
    Then the exit code is 3
    And stderr does not contain "did you mean"

  @US2-04 @invalid
  Scenario: A short name of another kind is not found by this kind's command
    Given I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    When I run "trcli specimen show <note>"
    Then the exit code is 3
    When I run "trcli specimen note <note> 'not mine'"
    Then the exit code is 3
    And nothing was changed

  @US2-05
  Scenario: Lists are filtered, sorted, searched, and limited the same way for every kind
    Given I ran "trcli specimen add --title 'Rain gauge'" and remember the record as "gauge"
    And I ran "trcli specimen add --title 'Rain water'" and remember the record as "water"
    And I ran "trcli specimen add --title 'Dry soil'"
    And I ran "trcli specimen tag <water> field-work"
    When I run "trcli specimen list --search rain --sort title"
    Then stdout lists "Rain gauge" before "Rain water"
    And stdout does not contain "Dry soil"
    When I run "trcli specimen list --sort title --desc --limit 1"
    Then stdout contains "Rain water"
    And stdout does not contain "Rain gauge"
    And stderr contains "Showing 1 of 3"
    When I run "trcli specimen list --tag field-work"
    Then stdout contains "Rain water"
    And stdout does not contain "Rain gauge"
    Given I ran "trcli sample-note add --body 'Rain all day'"
    And I ran "trcli sample-note add --body 'Clear sky'"
    When I run "trcli sample-note list --search rain --sort body --desc --limit 5"
    Then the exit code is 0
    And stdout contains "Rain all day"
    And stdout does not contain "Clear sky"

  @US2-06
  Scenario: Changing a record changes only what was named
    Given I ran "trcli sample-note add --body 'Original text' --locked" and remember the record as "note"
    And I ran "trcli sample-note tag <note> keep"
    When I run "trcli sample-note edit <note> --body 'Corrected text'"
    Then the exit code is 0
    When I run "trcli sample-note show <note>"
    Then stdout contains "Corrected text"
    And stdout contains "Locked   true"
    And stdout contains "keep"

  @US2-07
  Scenario: Tags and dated notes are shown with the record, and records of any kind are found by tag
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    When I run "trcli specimen tag <first> Field-Work to-read"
    Then the exit code is 0
    And stdout contains "field-work, to-read"
    When I run "trcli sample-note tag <note> field-work"
    And I run "trcli specimen note <first> 'Collected in the rain'"
    Then the exit code is 0
    When I run "trcli specimen show <first>"
    Then stdout contains "field-work, to-read"
    And stdout contains "Collected in the rain"
    And stdout contains "2026-10-08 14:00"
    When I run "trcli tag list"
    Then stdout contains "field-work  2"
    And stdout contains "to-read     1"
    When I run "trcli tag list --kind sample-note"
    Then stdout contains "field-work  1"
    And stdout does not contain "to-read"
    When I run "trcli specimen tag <first> to-read --remove"
    Then stdout contains "Removed"
    When I run "trcli tag list"
    Then stdout does not contain "to-read"

  @US2-08
  Scenario: Any two records are linked, saying how they relate, and the link is seen from both
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    When I run "trcli link add <first> <note> --relation 'same site'"
    Then the exit code is 0
    And stdout contains 'Linked <first> "First" ⟷ <note> "A note" (same site)'
    When I run "trcli link list <first>"
    Then stdout contains '<note> "A note" (same site, sample-note)'
    When I run "trcli link list <note>"
    Then stdout contains '<first> "First" (same site, specimen)'
    When I run "trcli sample-note show <note>"
    Then stdout contains "Links"
    And stdout contains "<first>"
    When I run "trcli link rm <note> <first> --relation 'same site'"
    Then the exit code is 0
    When I run "trcli link list <first>"
    Then stderr contains "has no links"

  @US2-08 @invalid
  Scenario: A record is not linked to itself, and the same link is not made twice
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli specimen add --title 'Second'" and remember the record as "second"
    When I run "trcli link add <first> <first>"
    Then the exit code is 2
    And stderr contains "a record cannot be linked to itself"
    And nothing was changed
    Given I ran "trcli link add <first> <second>"
    When I run "trcli link add <second> <first>"
    Then the exit code is 2
    And stderr contains "is already linked"
    And nothing was changed
    When I run "trcli link add <first> spc-zzzzzzzz"
    Then the exit code is 3
    When I run "trcli link rm <first> <second> --relation 'cites'"
    Then the exit code is 3
    And stderr contains "are not linked"
    When I run "trcli link list spc-zzzzzzzz"
    Then the exit code is 3

  @US2-09 @invalid
  Scenario: Deleting a record lists what refers to it and requires confirmation
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    And I ran "trcli link add <note> <first>"
    And I ran "trcli specimen tag <first> field-work"
    And I ran "trcli specimen note <first> 'Collected in the rain'"
    And standard input is not a terminal
    When I run "trcli specimen rm <first>"
    Then the exit code is 5
    And stderr contains "needs confirmation"
    And stderr contains 'link to <note> "A note" (related)'
    And stderr contains "1 tag: field-work"
    And stderr contains "1 note"
    And stderr contains "--yes"
    And nothing was changed

  @US2-10
  Scenario: After a confirmed deletion nothing points to the deleted record
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    And I ran "trcli sample-note add --body 'A note'" and remember the record as "note"
    And I ran "trcli link add <note> <first>"
    And I ran "trcli specimen tag <first> field-work"
    And I ran "trcli specimen note <first> 'Collected in the rain'"
    When I run "trcli specimen rm <first> --yes"
    Then the exit code is 0
    And stdout contains 'Deleted specimen <first> "First"'
    When I run "trcli link list <note>"
    Then stderr contains "has no links"
    When I run "trcli tag list"
    Then stdout does not contain "field-work"
    When I run "trcli specimen show <first>"
    Then the exit code is 3
    When I run "trcli workspace check"
    Then the exit code is 0

  @US2-11 @invalid
  Scenario: A deletion a feature forbids is refused with what stands in the way and the alternative
    Given I ran "trcli sample-note add --body 'Keep me' --locked" and remember the record as "note"
    When I run "trcli sample-note rm <note> --yes"
    Then the exit code is 5
    And stderr contains "cannot be deleted: it is locked"
    And stderr contains "trcli sample-note edit <ref> --unlock"
    And nothing was changed
    When I run "trcli sample-note edit <note> --unlock"
    And I run "trcli sample-note rm <note> --yes"
    Then the exit code is 0

  @US2-12 @invalid
  Scenario: A confirmation nobody can give fails at once unless yes was said beforehand
    Given I ran "trcli specimen add --title 'First'" and remember the record as "first"
    When I run "trcli specimen rm <first> --no-input"
    Then the exit code is 5
    And stderr contains "nobody can be asked"
    And nothing was changed
    When I run "trcli specimen rm <first> --no-input --yes"
    Then the exit code is 0

  @US2-13
  Scenario: The same action is invoked the same way on two kinds of record
    Then the options of "trcli specimen list" and "trcli sample-note list" are the same
    And the options of "trcli specimen show" and "trcli sample-note show" are the same
    And the options of "trcli specimen rm" and "trcli sample-note rm" are the same
    And the options of "trcli specimen tag" and "trcli sample-note tag" are the same
    And the options of "trcli specimen note" and "trcli sample-note note" are the same
    When I run "trcli sample-note list --help"
    Then stdout contains "--search"
    And stdout contains "--tag"
    And stdout contains "--sort"
    And stdout contains "--desc"
    And stdout contains "--limit"

  @US2-14
  Scenario: A list with nothing in it says so in one line and is not a failure
    When I run "trcli specimen list"
    Then the exit code is 0
    And stdout is empty
    And stderr contains "No specimen records match."
    When I run "trcli specimen list --output json"
    Then the exit code is 0
    And stdout is JSON where "data.total" is "0"
    And stdout is JSON where "data.items" is "[]"
