extern crate proc_macro;

use pest::iterators::Pairs;
use pest_meta::parser::{parse, Rule};
use proc_macro::TokenStream;
use std::{fs, str::FromStr};

#[proc_macro_derive(Parser, attributes(grammar))]
pub fn derive_parser(input: TokenStream) -> TokenStream {
    let mut result = String::new();
    clear_file("lolz.txt");
    let grammar_string =
        fs::read_to_string("src/grammar.pest").expect("Couldn't read 'grammar.pest'");
    let grammar = parse(Rule::grammar_rules, &grammar_string)
        .unwrap_or_else(|err| panic!("Error parsing grammar: {}", err));
    // fs::write("lolz.txt", format_tree(grammar, 0)).expect("Couldn't write to 'lolz.txt'");
    for pair in grammar {
        let rule = pair.as_rule();
        if rule != Rule::grammar_rule {
            unreachable!(
                "Top level rule should always be grammar rule. Found '{:?}'.",
                rule
            )
        };

    }
    TokenStream::from_str(&result).expect("Couldn't parse input as tokens")
}

fn parse_rule() {
    
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

fn append_file(path: &str, content: &str) {
    let file_content = fs::read_to_string(path).unwrap();
    fs::write(path, format!("{}\n{}", content, file_content))
        .unwrap_or_else(|_| panic!("Couldn't write to file '{}'", path));
}

fn clear_file(path: &str) {
    fs::write(path, "").unwrap_or_else(|_| panic!("Couldn't write to file '{}'", path));
}
