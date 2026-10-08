# Feature Specification: TRCLI Integrations and Google Drive

**Feature Branch**: `010-integrations`

**Created**: 2026-10-08

**Status**: Draft

**Input**: User description: "add integrations spec, this feature has the core for integrations and one concrete integration google drive (it is possible to sync directly via google oauth?)"

## Overview

Until now TRCLI has kept everything on one machine and has sent nothing anywhere. That is
safe, and it is also a limit: a researcher works on a laptop and a lab workstation, wants a
copy of years of work somewhere other than one disk, and keeps datasets too large to carry
around. This specification lets a workspace reach **outside services**, deliberately and
under the researcher's control.

It has two parts:

1. **The integration core** — what every connection to an outside service has in common:
   how the researcher sees what is available, connects, sees exactly what access was
   granted, checks that it works, sees everything that was sent and received, and
   disconnects. The core defines a small set of **capabilities** a service may offer, so
   that the rest of TRCLI uses any service the same way.
2. **One integration built on it: Google Drive** — signing in with a Google account and
   using the researcher's Drive to synchronize a workspace between their machines, to hold
   the files a workspace refers to, and to keep backups.

### Capabilities an integration may offer

| Capability | What it gives the researcher | Google Drive |
|------------|------------------------------|--------------|
| **Workspace sync** | The same workspace on several of their machines, kept in step | yes |
| **File storage** | A place for the files records refer to (datasets, figures, manuscripts), fetched where needed | yes |
| **Backup destination** | Somewhere off the machine to keep backups | yes |

Later integrations — another storage service, an institutional repository, a reference
manager — declare which of these they offer, or add new capabilities, without changing how
the researcher works.

### The answer to the question in the request

Yes: a workspace can synchronize directly with Google Drive after the researcher signs in
with their Google account. The researcher approves access in their browser; TRCLI never
sees their password; and TRCLI asks only for access to the files it creates itself, not to
the rest of the Drive. What this specification requires of that sign-in and sync is below;
the technical findings behind the answer are recorded in
[notes/google-sign-in.md](./notes/google-sign-in.md) for planning.

### The concepts, in one picture

```text
 Researcher ── connects ──▶ Connection ── to ──▶ Integration (Google Drive, …)
                               │  access granted, account, health        │ offers
                               │                                         ▼
                               │                                   Capabilities
        ┌──────────────────────┼───────────────────────────┐
        ▼                      ▼                           ▼
  Workspace sync          File storage              Backup destination
  machine A ⇄ remote ⇄ B  record's file ⇄ remote    backup ──▶ remote
        │                      │                           │
        └───────── every transfer ──▶ Transfer log ◀───────┘
```
