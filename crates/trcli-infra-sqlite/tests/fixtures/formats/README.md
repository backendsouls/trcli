# Workspaces of earlier formats

One directory per **released** workspace format, each holding a small workspace exactly as
that release wrote it:

```text
tests/fixtures/formats/
└── <format number>/
    └── .trcli/
        ├── trcli.db
        ├── audit.head
        └── config.toml
```

`tests/integration/trcli-infra-sqlite/upgrade.rs` upgrades a copy of every directory here and
checks that no record is lost and that the audit trail still verifies (FR-075).

## When a fixture is added

The format number in `trcli-domain` (`FormatVersion::CURRENT`) increases with each
**released** change to how a workspace is stored. Migrations added between two releases
together make the next format.

At each release that raises the format number, **before** raising it:

1. build the outgoing release;
2. create a workspace with it, holding at least one record of every kind, a tag, a note,
   a link, and a deleted record;
3. copy its `.trcli/` directory here under the outgoing format number.

Before the first release there is nothing to upgrade from, so there is no fixture and the
upgrade test covers only a failing migration and a workspace that is too new.
