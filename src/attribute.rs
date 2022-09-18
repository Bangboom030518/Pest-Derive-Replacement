use syn::{Attribute, DeriveInput, Lit, Meta};
pub fn get(ast: DeriveInput) -> String {
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
