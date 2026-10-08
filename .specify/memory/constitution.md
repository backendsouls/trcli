# TRCLI (The Research CLI) Constitution

TRCLI is a command-line tool that helps users manage and conduct research. This constitution
defines the non-negotiable engineering rules every feature, plan, and change MUST follow.

## Core Principles

### I. Clean Architecture
The codebase MUST be organized in concentric layers: domain (entities, value objects, domain
services), application (use cases, ports), and infrastructure/interface (CLI commands, file
storage, network, external adapters). Dependencies MUST point inward only; the domain and
application layers MUST NOT import from CLI frameworks, I/O libraries, or persistence code.
External concerns MUST be reached through ports (interfaces) implemented by adapters.
Rationale: isolating business rules keeps them testable and lets the CLI, storage, or
integrations change without rewriting research logic.

### II. Domain-Driven Design
The research domain MUST be modeled with a ubiquitous language shared by specs, code, tests,
and documentation. Code MUST use entities, value objects, aggregates, and repositories where
they express domain concepts, and each bounded context MUST have explicit boundaries.
Domain invariants MUST be enforced inside the domain model, never only in the CLI layer.
Rationale: a shared, explicit model prevents drift between what users mean and what the
code does.

### III. Test-First Development (TDD) (NON-NEGOTIABLE)
Tests MUST be written before implementation and MUST be observed failing before production
code is written (Red-Green-Refactor). No production code is merged without tests that cover
it. Unit tests cover the domain and application layers; integration tests cover adapters and
CLI commands. A failing test MUST NOT be disabled or deleted to make a change pass.
Rationale: tests written first define the behavior and guard against regressions.

### IV. Behavior-Driven Development (BDD)
Every user-facing feature MUST have acceptance scenarios in Given/When/Then form, derived
from the feature spec, expressed in domain language, and automated as executable tests.
Scenarios MUST cover the primary flow, error flows, and invalid-input cases. A feature is
not complete until its scenarios pass.
Rationale: executable scenarios keep implementation tied to the behavior users were promised.

### V. Input Validation (NON-NEGOTIABLE)
All user input (arguments, options, flags, prompts, stdin, files, environment variables, and
data from external sources) MUST be validated at the system boundary before reaching the
application or domain layers. Validation MUST check type, format, range, length, and
allowed values, and MUST reject invalid input with a clear, actionable message on stderr and
a non-zero exit code. Invalid-input tests MUST exist for every input a command accepts.
Domain invariants (Principle II) are enforced in addition to, not instead of, boundary
validation. Rationale: a CLI is driven entirely by untrusted input; rejecting bad input
early protects data integrity and security.

### VI. Full Code Documentation
All code MUST be fully documented: every module, public class, function, method, and port
MUST have a doc comment describing purpose, parameters, return values, raised errors, and
side effects, using the idiomatic documentation format of the chosen language. Non-obvious
logic and design decisions MUST carry explanatory comments stating why. Documentation MUST
be updated in the same change that alters the code. Rationale: documented code lowers the
cost of contribution and review for an open-source project.

### VII. Feature Documentation and Usage Guides
Every feature MUST be documented, and MUST have a dedicated usage Markdown file (e.g.,
`docs/usage/<feature>.md`) covering purpose, commands and options, examples with expected
output, error cases, and exit codes. The usage file MUST be created or updated in the same
change as the feature, and a feature is not complete without it. Rationale: users adopt
only what they can learn to use, and docs that ship with the code stay accurate.

## Quality Gates

- Every change MUST pass the full automated test suite (unit, integration, BDD scenarios).
- Every change MUST include tests that failed before the implementation (Principle III).
- Linting, formatting, and documentation-coverage checks MUST pass once tooling is chosen.
- Dependency direction MUST be verified: no inward layer imports outward layers.
- Each new or changed command MUST have boundary-validation tests (Principle V).
- Each feature MUST ship its usage Markdown file (Principle VII).
- Technology stack: TODO(TECH_STACK): not yet chosen; to be recorded here when decided.

## Development Workflow

- Work follows the Spec Kit flow: specify → clarify → plan → tasks → implement.
- Plans MUST include a Constitution Check against all seven principles; violations MUST be
  listed with justification in the plan's complexity tracking before work proceeds.
- Tasks MUST order tests before the implementation they cover.
- Code review MUST verify compliance with this constitution; non-compliant changes MUST NOT
  be merged.
- Commits SHOULD be small and focused, and MUST NOT mix unrelated concerns.

## Governance

This constitution supersedes all other practices and conventions in the project. Amendments
MUST be proposed in writing with rationale, reviewed and approved by the maintainers, and
include a migration plan for any existing code or documents they invalidate. Versioning
follows semantic versioning: MAJOR for backward-incompatible principle removals or
redefinitions, MINOR for added principles/sections or materially expanded guidance, PATCH
for clarifications and wording fixes. Compliance MUST be reviewed in every pull request and
at each plan's Constitution Check. Any justified deviation MUST be documented in the plan
and approved by a maintainer.

**Version**: 1.0.0 | **Ratified**: 2026-10-08 | **Last Amended**: 2026-10-08
