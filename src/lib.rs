#![warn(clippy::pedantic, clippy::nursery)]

extern crate proc_macro;

use attribute::GrammarFile;
use pest::iterators::Pair as PestPair;
use pest_meta::parser::{parse, Rule};
use proc_macro::TokenStream;
use std::{env, fs, path::Path, str::FromStr};
use syn::DeriveInput;

mod attribute;
mod log;

type Pair<'a> = PestPair<'a, Rule>;

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
    let attribute::GrammarFile {
        content: grammar_string,
        path,
    } = attribute::get_file(ast);
    let result = String::new();
    log::clear();
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

fn parse_rule(pair: Pair) {
    log::log(&log::format_tree(pair.into_inner(), 0));
}
