#![warn(clippy::pedantic, clippy::nursery)]

extern crate proc_macro;

use pest::iterators::{Pair, Pairs};
use pest_meta::parser::{parse, Rule};
use proc_macro::TokenStream;
use std::{env, fs, path::Path, str::FromStr};
use syn::DeriveInput;

mod attribute;
mod log;

/// Generates parser struct from grammar file
/// 
/// # Panics
/// 
/// 1. Panics if unable to open the file specified through the grammar attribute
/// 2. Panics if the grammar attribute doesn't point to a file
/// 3. Panics if unable to parse grammar file, due to a syntax error
#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
    let grammar_path = attribute::get(ast);
    let (data, path) = {
        let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());

        // Check whether we can find a file at the path relative to the CARGO_MANIFEST_DIR
        // first.
        //
        // If we cannot find the expected file over there, fallback to the
        // `CARGO_MANIFEST_DIR/src`, which is the old default and kept for convenience
        // reasons.
        // https://doc.rust-lang.org/std/path/fn.absolute.html
        let path = if Path::new(&root).join(&grammar_path).exists() {
            Path::new(&root).join(&grammar_path)
        } else {
            Path::new(&root).join("src/").join(&grammar_path)
        };

        let file_name = match path.file_name() {
            Some(file_name) => file_name,
            None => panic!("grammar attribute should point to a file"),
        };

        let data = match fs::read_to_string(&path) {
            Ok(data) => data,
            Err(error) => panic!("error opening {:?}: {}", file_name, error),
        };
        (data, Some(path.clone()))
    };
    let mut result = String::new();
    log::clear();
    let grammar_string =
        fs::read_to_string("src/grammar.pest").expect("Couldn't read 'grammar.pest'");
    let grammar = parse(Rule::grammar_rules, &grammar_string)
        .unwrap_or_else(|err| panic!("Error parsing grammar: {}", err));
    // fs::write("lolz.txt", format_tree(grammar, 0)).expect("Couldn't write to 'lolz.txt'");
    for pair in grammar {
        let rule = pair.as_rule();
        match rule {
            Rule::EOI => break,
            Rule::grammar_rule => parse_rule(pair),
            rule => unreachable!(
                "Top level rule should always be grammar rule. Found '{:?}'.",
                rule
            ),
        };
    }
    TokenStream::from_str(&result).expect("Couldn't parse input as tokens")
}

fn parse_rule(pair: Pair<Rule>) {
    log::log(&log::format_tree(pair.into_inner(), 0))
}
