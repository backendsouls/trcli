### User Story 3 - Have every input checked before anything changes (Priority: P3)

Whatever a researcher gives the tool — typed on the command line, answered to a question,
read from a file, brought in from elsewhere — is checked before anything is stored or
done. If something is wrong, nothing changes, and they are told everything that is wrong
at once: which value, why, and what would be right.

**Why this priority**: A research record that contains a wrong date or a broken link is
worse than no record. Checking at the door, completely and helpfully, is what lets a
researcher trust what is inside.

**Independent Test**: Give a command three invalid values at once and confirm all three are
reported together, each named with what is expected, and that nothing was stored; bring in
a file with some invalid entries and confirm each is reported and the valid ones handled as
that feature specifies.

**Acceptance Scenarios**:

1. **Given** a command with an invalid value, **When** it is run, **Then** nothing is stored
   or done, and the tool names the value, says what is wrong with it, and says what is
   expected.
2. **Given** a command with several invalid values, **When** it is run, **Then** all of them
   are reported together, not only the first.
3. **Given** a value that must be of a certain form — a date, a number, an address, an
   identifier — **When** it is not, **Then** the message shows an example of a valid one.
4. **Given** a value that must be one of a set, **When** it is not, **Then** the valid
   choices are listed.
5. **Given** a value that names another record or a file, **When** that record or file does
   not exist, **Then** this is reported like any other invalid value.
6. **Given** text that is empty where it is required, longer than its limit, or contains
   characters that cannot be stored or shown, **When** it is given, **Then** it is rejected
   with the field and the limit.
7. **Given** values that are each valid and together are not — an end before a start —
   **When** they are given, **Then** the pair is reported with the rule it breaks.
8. **Given** input read from a file or brought in from another tool, **When** it is
   processed, **Then** every entry is checked by the same rules as typed input.
9. **Given** an invalid command — an unknown action, a missing required value, an option
   that does not exist — **When** it is run, **Then** the tool says what is wrong and shows
   how the command is used.
10. **Given** something that is unusual but allowed, **When** it is given, **Then** the tool
    warns and proceeds, or asks first when the feature says so — and never silently changes
    the value.
11. **Given** free text in any language and script, **When** it is given where free text is
    expected, **Then** it is accepted and kept exactly as written.
12. **Given** invalid input, **When** the command ends, **Then** it reports failure in a way
    that a calling program can tell apart from success and from other kinds of failure.

---
