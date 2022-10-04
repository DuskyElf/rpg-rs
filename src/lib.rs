use pancurses::*;
use std::{thread, time, fmt::Write};

mod models;
mod runtime;
mod lang;
pub use lang::Lexer;
pub use models::*;
pub use runtime::start_game;

