//! The `trcli` command-line tool: its composition root and its presentation.
//!
//! - [`cli`] and [`args`] define what can be typed.
//! - [`compose`] builds the session of one command; it is the only module that names
//!   concrete adapters.
//! - [`commands`] turn parsed arguments into calls of the application's use cases and
//!   hand back view models; [`shared_verbs`] does so for the verbs every kind of record
//!   has.
//! - [`render`], [`output`], [`prompt`], [`progress`], and [`diagnostics`] are everything
//!   the researcher sees and answers.
//! - [`extension`] is how a feature's kinds of record and commands are added.
//! - [`run`] is the path of one command, from the arguments to the exit code.
//!
//! The library exists so that the black-box tests in this crate's tests/ can walk the
//! command tree, and so that a feature can be added from outside (see
//! `examples/sample_kinds`); the `trcli` binary is a few lines around [`run::main`].

// Ports are used on one thread; see the same note in the application crate.
#![allow(async_fn_in_trait)]
// Tests are straight lists of steps and assertions; see the application crate's note.
#![cfg_attr(test, allow(clippy::cognitive_complexity, clippy::too_many_lines))]

pub mod args;
pub mod cli;
pub mod commands;
pub mod compose;
pub mod diagnostics;
pub mod extension;
pub mod output;
pub mod progress;
pub mod prompt;
pub mod render;
pub mod run;
pub mod shared_verbs;
