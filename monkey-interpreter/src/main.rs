use std::io;

use crate::repl::start;

pub mod lexer;
pub mod token;
pub mod repl;

fn main() {
    println!("Hello, This is the Monkey Programming language");
    println!("fell free to type in the code");
    start(io::stdin(), io::stdout());
}
