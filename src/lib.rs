#![feature(const_discriminant)]
#![feature(string_remove_matches)]
#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod state;

pub mod types;
pub mod pane;

pub use app::App;
pub use state::State;