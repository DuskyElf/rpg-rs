use pancurses::*;
use std::{thread, time, fmt::Write};

mod models;     // Constants and data structures
mod runtime;    // The ast interpreter
mod lang;       // The rpg compiler
pub use lang::compile;
pub use models::*;
pub use runtime::start_game;

