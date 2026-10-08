//! Ports: the small interfaces through which use cases reach the outside (FR-071).
//!
//! Each trait is named for what its caller needs, not for what implements it, and is kept
//! small so that a use case's type parameters list exactly what it uses. Every port has an
//! in-memory fake in the `trcli-testing` crate and passes the same contract tests as its real
//! adapter.
//!
//! Only ports that reach storage, or wait for a person, are `async`.

pub mod audit;
pub mod environment;
pub mod interaction;
pub mod records;
pub mod settings;
pub mod unit_of_work;
pub mod workspace;
