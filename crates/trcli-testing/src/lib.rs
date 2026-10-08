//! Test support for TRCLI: stand-ins for everything outside the application, and the
//! tests every adapter must pass.
//!
//! This crate is a development dependency of the others and nothing more: the doubles
//! and the suites are kept out of the crates that are compiled into the tool.
//!
//! The doubles are *fakes*: small working implementations (storage that commits and
//! rolls back, a clock that stands still), not mocks programmed call by call. A test
//! then asserts what is true afterwards rather than which calls were made, and the
//! contract suites below hold each fake to the behaviour of the real adapter.
//!
//! - In-memory fakes of every port: [`mod@unit`] (storage and every store), [`environment`],
//!   [`interaction`], [`settings`], [`workspace`], [`audit`].
//! - Contract suites, generic over the ports: [`contract_uow`], [`contract_workspace`],
//!   [`contract_records`], [`contract_audit`]. The same functions run against the fakes
//!   here and against the SQLite adapter in its own crate, which is how a fake is known to
//!   behave like the real thing (Liskov substitution, checked).
//!
//! Nothing here is compiled into the `trcli` binary.

// Contract suites are straight lists of steps and assertions; the limits on length and
// branching that keep production functions readable would only cut them arbitrarily.
#![allow(clippy::cognitive_complexity, clippy::too_many_lines)]

pub mod audit;
pub mod contract_audit;
pub mod contract_records;
pub mod contract_uow;
pub mod contract_workspace;
pub mod environment;
pub mod interaction;
pub mod settings;
pub mod unit;
pub mod workspace;

use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};

/// Runs a future to completion on the current thread.
///
/// The fakes never wait for anything, so polling in a loop is enough; this keeps an async
/// runtime out of the tests of the use cases.
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    loop {
        if let Poll::Ready(output) = future.as_mut().poll(&mut context) {
            return output;
        }
    }
}
