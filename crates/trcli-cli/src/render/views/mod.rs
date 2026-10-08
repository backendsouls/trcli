//! The form for people of every view model, grouped as the commands are.
//!
//! A view says *what* to show by calling the layout helpers of
//! [`crate::render::human::Human`]; it never prints, and it never decides anything.

mod audit;
mod general;
mod records;
mod settings;
mod workspace;

pub use general::{Text, Verified};
