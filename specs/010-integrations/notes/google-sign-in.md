# Notes for Planning: Signing in to Google and Syncing with Drive

**Spec**: [../spec.md](../spec.md) | **Checked**: 2026-10-08, against Google's developer documentation

The request asked: *is it possible to sync directly via Google OAuth?* **Yes.** These notes
record what was verified, so that planning starts from facts. They are technical on purpose
and are not part of the specification.

## What was verified

| Finding | Consequence for TRCLI | Source |
|---------|----------------------|--------|
| A desktop or command-line application signs a user in by opening the system browser and listening on a loopback address (`http://127.0.0.1:<port>`) for the answer, with PKCE. This is supported for the "Desktop app" client type. | The browser sign-in of User Story 2 is the standard, supported path. | [OAuth 2.0 for iOS & Desktop Apps](https://developers.google.com/identity/protocols/oauth2/native-app), [Loopback migration guide](https://developers.google.com/identity/protocols/oauth2/resources/loopback-migration) |
| The older copy-and-paste ("out-of-band") flow has been removed. | Do not plan a "paste this code back into the terminal" sign-in. | [OOB migration guide](https://developers.google.com/identity/protocols/oauth2/resources/oob-migration) |
| The device flow (show a web address and a short code, approve on another device) exists for limited-input devices and supports only a short list of scopes, which **includes** `drive.file` and `drive.appdata`. | The no-browser sign-in of User Story 2 (a lab server over a remote shell) is possible precisely because the spec asks only for per-file access. | [OAuth 2.0 for TV and Limited-Input Devices](https://developers.google.com/identity/protocols/oauth2/limited-input-device) |
| `drive.file` grants access only to files the application created or the user opened with it. Google classes it as non-sensitive; the full `drive` scope is restricted and needs a security assessment. | FR-018 (only what TRCLI creates) matches the scope that avoids the heaviest review. It also means TRCLI cannot browse existing Drive files — stated in the spec's assumptions. | [Choose Google Drive API scopes](https://developers.google.com/workspace/drive/api/guides/api-specific-auth), [Restricted scope verification](https://developers.google.com/identity/protocols/oauth2/production-readiness/restricted-scope-verification) |
| While an application's consent screen is in "Testing" status with external users, refresh tokens expire after 7 days. | FR-020 (keeps working for months) requires the application to be published "In production", not left in testing. | [Using OAuth 2.0 to Access Google APIs](https://developers.google.com/identity/protocols/oauth2), [Manage App Audience](https://support.google.com/cloud/answer/15549945?hl=en) |

## Decisions this leaves for planning

1. **Whose registration with Google?** Someone must register TRCLI as an application.
   Options: the project registers once and ships the client identifier (simplest for
   users; the project then owns Google's publishing and any verification); or each
   researcher or institution registers their own and gives it to TRCLI (no dependency on
   the project, more setup). Supporting both is common. For a desktop client the "client
   secret" is not confidential, which is why PKCE is used.
2. **Publishing status.** To be usable beyond a small list of test accounts and to avoid
   7-day sign-ins, the registration must be "In production". What review Google asks for
   with only `drive.file` should be confirmed at that time; these notes did not verify it.
3. **Visible folder or hidden application data.** `drive.file` allows a folder the
   researcher can see (what FR-019 requires); `drive.appdata` is a hidden per-application
   area. The spec chose visible. A hidden area could still hold small bookkeeping.
4. **What is stored remotely.** The plan of `specs/001-research-workspace` already rules out
   placing the working database where another program rewrites it. Sync should exchange
   changes or complete, verified snapshots through the Drive API, never the live database
   file. The hash-chained audit trail and globally unique record identifiers in that plan
   were chosen with this in mind.
5. **Concurrent senders.** FR-032 needs "one wins, the other receives first". Planning must
   pick how to detect a remote copy that changed since it was read; this was not verified
   against Drive's features.
6. **Where credentials live.** FR-012 needs the operating system's secret store on Linux,
   macOS, and Windows, with a stated fallback. Headless Linux machines often have none.
7. **Limits.** Drive applies request quotas and upload size rules; FR-058 covers behaving
   well under them. Actual numbers were not checked here.

## Not verified

- The exact review Google requires for a published application that uses only
  `drive.file`.
- Drive's quota figures and resumable-upload details.
- Behavior under institution-managed (Workspace) accounts whose administrators restrict
  third-party applications; FR-022 assumes such blocking can be told apart from other
  failures.
