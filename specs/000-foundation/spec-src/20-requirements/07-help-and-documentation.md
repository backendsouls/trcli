#### Help and documentation

- **FR-055**: The tool run with no arguments, and asked for help with no further word, MUST
  list its groups of commands with one line each and how to learn more, and MUST NOT fail.
- **FR-056**: Every command MUST have built-in help stating its purpose, its arguments and
  options with allowed values and defaults, and at least one example.
- **FR-057**: Every feature MUST have a usage guide covering its purpose, its commands and
  options, worked examples with what they print, the ways it can fail, and how each failure
  ends.
- **FR-058**: The examples in usage guides MUST be checked against the tool automatically,
  so that a guide that no longer matches the tool is detected.
- **FR-059**: The tool MUST report its version and the workspace format it reads and writes,
  and MUST provide completion support for the commonly used command shells.
- **FR-060**: Help, guides, messages, and commands MUST use the same word for the same thing.
- **FR-061**: In a workspace with nothing in it, the tool MUST be able to suggest first
  steps.
