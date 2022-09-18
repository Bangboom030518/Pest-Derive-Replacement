extern crate proc_macro;

use pest::iterators::{Pair, Pairs};
use pest_meta::parser::{parse, Rule};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use std::{fs, str::FromStr};
use syn::{Attribute, DeriveInput, Generics, Ident, Lit, Meta};

mod log;

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
    let (name, generics, content) = parse_derive(ast);
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

enum GrammarSource {
    File(String),
    Inline(String),
}

fn parse_rule(pair: Pair<Rule>) {
    log::log(&format_tree(pair.into_inner(), 0))
}

fn format_tree(tree: Pairs<Rule>, indent: usize) -> String {
    let mut lines = Vec::<String>::new();
    for node in tree {
        lines.push(format!(
            "{}{:?} ({})",
            "\t".repeat(indent),
            node.as_rule(),
            node.as_span().as_str()
        ));
        lines.push(format_tree(node.into_inner(), indent + 1))
    }
    lines.join("\n")
}

