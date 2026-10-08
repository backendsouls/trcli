## Assumptions

- **Builds on `specs/001-research-workspace`**: The workspace, papers, citations, drafts,
  experiments, pipelines, runs, results, datasets, methodologies, environment snapshots,
  staff, research questions, hypotheses, audit trail, and telemetry are assumed to exist as
  specified there. Each story names, by its content, the base records it needs.
- **Venues are specified separately**: venues, events, calls, and talks (formerly User Story
  12 here) are in `specs/015-venues`. Submissions and peer review (User Story 8) stay here
  and refer to those venues.
- **Sync and outside services are specified separately**: connecting to a service, syncing
  a workspace between one researcher's machines, remote file storage, and remote backups
  are in `specs/010-integrations`. Collaboration (User Story 17 here) builds on that sync
  and adds members and roles.
- **Literature is specified separately**: reading annotations and structured summaries
  (formerly User Story 1 here) are in `specs/005-literature`.
- **Experiments are specified separately**: run parameters, metrics, sweeps, and code and
  software versions (formerly User Stories 5 and 6 here) are in `specs/004-experiments`.
- **Three tiers, delivered in order**: Stories 1–3 are expected soon after the base
  workspace; stories 4–10 serve real projects; stories 11–17 come later. Each story is
  planned and delivered separately.
- **Collaboration replaces the single-researcher assumption only when it arrives**: Until
  story 17 is delivered, the base workspace's assumption holds — one researcher, one
  machine, no accounts. Story 17 assumes a small team (up to about 20 members), each
  working on their own copy and exchanging changes deliberately; it is not live, simultaneous
  editing. How members are identified and how a shared copy is hosted are decided at
  planning time.
- **Files are referenced, not stored**: As in the base workspace, slides, figures, code,
  notebooks, and data are known by location, version, and fingerprint. Backups cover the
  workspace's records; whether referenced files are included is a choice the researcher
  makes per backup, off by default.
- **Publishing prepares, the researcher deposits**: The tool checks readiness, produces the
  bundle, and records the identifier. Uploading to an archive directly, and obtaining an
  identifier automatically, are out of scope.
- **Funding is tracking, not accounting**: Budget lines and spending are recorded for the
  researcher's overview. Invoices, payroll, and institutional finance systems are out of
  scope.
- **Ethics features support, they do not certify**: Warnings and acknowledgements help the
  researcher meet obligations; the tool does not judge legal compliance, and it does not
  encrypt or anonymize data.
- **Peer review is recorded by the author**: The tool holds the comments the researcher
  received and their responses. It is not a system through which reviewers submit reviews.
- **Pre-registration is sealed locally**: Freezing proves the content has not changed since
  the recorded time within the workspace. Depositing it with a public registry is covered
  by publishing outputs or done by the researcher.
- **Sweeps run on the researcher's own machine**, like other runs; distributing runs across
  remote machines is out of scope.
- **Integrations are with the file formats and conventions researchers already use**; which
  specific tools are supported first is decided at planning time.
- **Extensions are trusted by the researcher who adds them**; the tool informs and asks, but
  it does not vet third-party extensions.
- **Money is recorded in the currency entered**, with no conversion.
- **Interface language is English**, with dates in an unambiguous international form.
