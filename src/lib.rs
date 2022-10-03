use pancurses::*;
use std::{thread, time, fmt::Write};

mod models;
mod runtime;
pub use models::*;
pub use runtime::start_game;

