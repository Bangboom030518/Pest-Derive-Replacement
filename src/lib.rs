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

impl<'a> Tokens<'a> {
    fn next(&mut self) -> Option<&Pair<'a>> {
        self.current_index += 1;
        self.content.get(self.current_index)
    }

    fn peek(&self) -> Option<&Pair<'a>> {
        self.content.get(self.current_index + 1)
    }

    fn peek_back(&self) -> Option<&Pair<'a>> {
        self.content.get(self.current_index + 1)
    }

    fn new(content: Vec<Pair<'a>>) -> Self {
        Self {
            content,
            current_index: 0,
        }
    }
}

enum BinaryOperator {
    Sequence,
    Choice,
}

fn expect_structure<'a>(
    mut pairs: Pairs<'a>,
    rules: &'a [Rule],
    name: &'a str,
    index: usize,
) -> Pair {
    let all_pairs = pairs.clone().collect::<Vec<Pair>>();
    let length = all_pairs.len();
    for (index, &rule) in rules.iter().enumerate() {
        let pair = pairs
            .next()
            .unwrap_or_else(|| panic!("Rule '{}' should have a child at index {}", name, index));
        assert_eq!(
            pair.as_rule(),
            rule,
            "Child {} should be the rule '{:?}'. Found '{:?}'",
            index,
            rule,
            pair.as_rule()
        );
    }
    all_pairs
        .get(index)
        .unwrap_or_else(|| {
            panic!(
                "Index should be in range. Trying to access index '{}' in list of length '{}'",
                index, length
            )
        })
        .clone()
}
