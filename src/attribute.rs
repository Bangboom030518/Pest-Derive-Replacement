use std::path::{Path, PathBuf};
use std::{env, fs};
use syn::{Attribute, DeriveInput, Lit, Meta};

fn get_path(ast: &DeriveInput) -> String {
    let grammar: Vec<&Attribute> = ast
        .attrs
        .iter()
        .filter(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(name_value)) => name_value.path.is_ident("grammar"),
            _ => false,
        })
        .collect();

    match grammar.len() {
        0 => panic!("a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute"),
        1 => from_attribute(grammar[0]),
        _ => panic!("only 1 grammar file can be provided"),
    }
}

fn from_attribute(attr: &Attribute) -> String {
    match attr.parse_meta() {
        Ok(Meta::NameValue(name_value)) => match name_value.lit {
            Lit::Str(string) => string.value(),
            _ => panic!("grammar attribute must be a string"),
        },
        _ => panic!("grammar attribute must be of the form `grammar = \"...\"`"),
    }
}

pub struct GrammarFile {
    pub content: String,
    pub path: PathBuf,
}

fn read_file(path: &str) -> GrammarFile {
    let root = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".into());

    // Check whether we can find a file at the path relative to the CARGO_MANIFEST_DIR
    // first.
    //
    // If we cannot find the expected file over there, fallback to the
    // `CARGO_MANIFEST_DIR/src`, which is the old default and kept for convenience
    // reasons.
    // https://doc.rust-lang.org/std/path/fn.absolute.html
    let path = if Path::new(&root).join(&path).exists() {
        Path::new(&root).join(&path)
    } else {
        Path::new(&root).join("src/").join(&path)
    };

    let file_name = match path.file_name() {
        Some(file_name) => file_name,
        None => panic!("grammar attribute should point to a file"),
    };

    let content = match fs::read_to_string(&path) {
        Ok(data) => data,
        Err(error) => panic!("error opening {:?}: {}", file_name, error),
    };
    GrammarFile { content, path }
}

pub fn get_file(ast: &DeriveInput) -> GrammarFile {
    read_file(&get_path(ast))
}
