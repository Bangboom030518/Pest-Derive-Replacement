#![warn(clippy::pedantic, clippy::nursery)]

extern crate proc_macro;

use parser::Expression;
use pest::iterators::{Pair as PestPair, Pairs as PestPairs};
use pest_meta::parser::{parse, Rule};
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

mod attribute;
mod log;
mod parser;

type Pair<'a> = PestPair<'a, Rule>;
type Pairs<'a> = PestPairs<'a, Rule>;

/// Generates parser struct from grammar file
///
/// # Panics
///
/// 1. Panics if unable to open the file specified through the grammar attribute
/// 2. Panics if the grammar attribute doesn't point to a file
/// 3. Panics if unable to parse grammar file, due to a syntax error
#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    log::clear();
    let ast: DeriveInput = syn::parse2(input.into()).unwrap();
    let attribute::GrammarFile {
        content: grammar_string,
        ..
    } = attribute::get_file(&ast);
    let name = ast.ident;
    let tokens = {
        quote! {
            impl #name {
                fn parse<'a>(self) -> &'a str {
                    "Hello World!"
                }
            }
        }
    };

    log::log(&tokens.to_string());
    let grammar = parse(Rule::grammar_rules, &grammar_string)
        .unwrap_or_else(|err| panic!("Error parsing grammar: {}", err));
    log::log(log::format_tree(grammar.clone(), 0));
    for pair in grammar {
        let rule = pair.as_rule();
        match rule {
            Rule::EOI => continue,
            Rule::grammar_rule => parse_rule(pair),
            rule => unreachable!(
                "Top level rule should always be grammar rule. Found '{:?}'.",
                rule
            ),
        };
    }
    tokens.into()
}

/// Returns enum from rule
fn parse_rule(pair: Pair) -> TokenStream {
    let pairs = pair.into_inner();
    let expression = expect_structure(
        pairs,
        &[
            Rule::identifier,
            Rule::assignment_operator,
            Rule::opening_brace,
            Rule::expression,
            Rule::closing_brace,
        ],
        "rule",
        3,
    );
    log::log(expression.clone());
    Expression::from(expression);
    TokenStream::new()
}

struct Tokens<'a> {
    content: Vec<Pair<'a>>,
    current_index: usize,
}

enum BinaryOperator {
    Sequence,
    Choice,
}
