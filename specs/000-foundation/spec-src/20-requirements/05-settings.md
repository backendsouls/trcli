#### Settings

- **FR-039**: Users MUST be able to set, view, and remove settings for themselves (applying
  in every workspace) and for a workspace (applying there), and to override any setting for
  a session or a single command.
- **FR-040**: The value in effect MUST be decided in this order, later winning: the default;
  the user's value; the workspace's value; the session's override; the command's override.
- **FR-041**: Users MUST be able to list every setting with its value in effect and where
  that value comes from, and to see for any setting its meaning, allowed values, and
  default.
- **FR-042**: Every setting MUST have a default with which the tool is usable; nothing MUST
  need configuring before first use.
- **FR-043**: Every setting MUST be validated when it is set and when it is read; an invalid
  value or an unknown setting, in a command or in a stored file, MUST be an error that names
  where it is and what is expected, and the tool MUST NOT run with a partly valid
  configuration.
- **FR-044**: A setting that applies only to a workspace, or only to a person, MUST be
  ignored with a warning when found in the other place.
- **FR-045**: Settings MUST NOT hold secrets.
