## Assumptions

- **This specification comes first.** Every other specification of TRCLI depends on it and
  none is built before it. It owns the workspace, the behaviour shared by all records,
  input validation, output and interaction, settings, the audit trail, local telemetry,
  help, and the rules every feature follows.
- **It replaces foundational parts of `specs/001-research-workspace`**: its User Stories 1
  and 10 and its requirements FR-001 to FR-015 and FR-051 to FR-055, which keep pointers.
  The contracts that state these rules precisely — the CLI conventions, output and exit
  codes, and configuration — now live with this specification.
- **"Architecture" is specified here as testable qualities and rules**, in the sense given
  in the overview. The decisions already taken for the implementation — the programming
  language, the storage, the libraries, the layout of the code, and practices such as
  writing tests first, modelling the domain explicitly, and specifying behaviour as
  scenarios — are recorded in the project's constitution and in the implementation plans,
  and will be consolidated in this specification's own plan.
- **The implementation plan written for `specs/001-research-workspace` predates this
  specification.** Its architectural content (the layering, the storage, the handling of
  identifiers, the conventions for testing) is the starting point for this specification's
  plan and is to be moved there; the later rule to keep libraries to a minimum applies to
  it.
- **Single researcher, local workspace.** One person uses a workspace at a time, on their
  own machine, with no accounts, sign-in, or permissions. Two commands may run at the same
  moment (two terminals); two people editing at once is not in scope here.
- **The actor is not authenticated.** The audit trail records the name the researcher set,
  or the name they are known by on the machine; it is evidence of what the tool did, not
  proof of who was at the keyboard.
- **Tamper-evidence, not tamper-proofing.** The audit trail makes alterations detectable;
  someone who rewrites the entire trail consistently is not detected without an outside
  reference, which is out of scope.
- **Telemetry is for the researcher**, about their own use; it is never usage
  reporting to the authors of the tool.
- **Exceptions to "nothing leaves the machine" are owned by the features that make them**:
  looking up a reference by identifier (`specs/005-literature`) and connecting an outside
  service (`specs/010-integrations`). This specification states the rule they must respect.
- **Projects scope records** as specified in `specs/003-research-projects`; full backup,
  restore, and export of a workspace in `specs/002-research-lifecycle`. This specification
  requires only that an upgrade keeps a copy first.
- **The three supported systems are Linux, macOS, and Windows**, in their versions in
  current use.
- **Interface language is English**; content in any language is accepted. Dates are shown in
  an unambiguous international form.
- **Researchers are the users of every story except the last**, whose user is a contributor
  to TRCLI itself. That story is in this specification because the consistency the
  researcher experiences depends on it.
