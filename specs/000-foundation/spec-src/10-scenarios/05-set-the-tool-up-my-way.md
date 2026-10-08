### User Story 5 - Set the tool up my way (Priority: P5)

A researcher adjusts how TRCLI behaves: their preferences in one place, valid for
everything they do; choices that belong to a particular workspace kept with that workspace;
and, for one session or one command, an override. At any time they can see every setting,
its value, and where that value comes from.

**Why this priority**: Every feature has things worth adjusting, and without one place and
one rule for them each would invent its own. It comes after the behaviours that need no
adjusting to be useful.

**Independent Test**: Set a preference for yourself, a different value for one workspace,
and an override for one command; list settings and confirm each value and its source; set
an invalid value and confirm it is refused.

**Acceptance Scenarios**:

1. **Given** a setting, **When** the researcher sets it for themselves, **Then** it applies
   in every workspace they use.
2. **Given** a setting, **When** the researcher sets it for one workspace, **Then** it
   applies there only, and takes the place of their personal value.
3. **Given** a setting, **When** the researcher overrides it for a session or for one
   command, **Then** the override wins for that session or command and nothing stored is
   changed.
4. **Given** settings from several places, **When** the researcher lists them, **Then**
   every setting is shown with its value in effect and where that value comes from.
5. **Given** a setting, **When** the researcher asks about it, **Then** its meaning, its
   allowed values, and its default are shown.
6. **Given** an invalid value or a setting that does not exist, **When** the researcher sets
   it, **Then** the tool refuses and says what is allowed.
7. **Given** a settings file containing an invalid value or an unknown setting, **When** any
   command is run, **Then** the tool names the file, the setting, and what is expected, and
   does not run with a partly valid configuration.
8. **Given** a setting that only makes sense for a workspace, or only for a person, **When**
   it is found in the other place, **Then** it is ignored with a warning.
9. **Given** a setting, **When** the researcher removes their value, **Then** the next value
   in order applies again.
10. **Given** no settings at all, **When** the tool is used, **Then** every setting has a
    sensible default and nothing needs configuring first.
11. **Given** settings, **When** they are stored, **Then** nothing secret is ever among them.

---
