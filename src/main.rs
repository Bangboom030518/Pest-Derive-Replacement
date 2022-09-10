#![warn(clippy::pedantic, clippy::nursery)]

use pest_test::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
struct GrammarParser;

fn main() {
    println!("Hello!");
}
