use pancurses::*;
use std::{thread, time, fmt::Write};

mod models;
mod runtime;
mod lang;
pub use lang::compile;
pub use models::*;
pub use runtime::start_game;

