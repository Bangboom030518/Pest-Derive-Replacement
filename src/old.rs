fn parse_derive(ast: DeriveInput) -> (Ident, Generics, GrammarSource) {
    let DeriveInput {
        generics,
        ident: name,
        ..
    } = ast;

    let grammar: Vec<&Attribute> = ast
        .attrs
        .iter()
        .filter(|attr| match attr.parse_meta() {
            Ok(Meta::NameValue(name_value)) => {
                name_value.path.is_ident("grammar") || name_value.path.is_ident("grammar_inline")
            }
            _ => false,
        })
        .collect();

    let argument = match grammar.len() {
        0 => panic!("a grammar file needs to be provided with the #[grammar = \"PATH\"] or #[grammar_inline = \"GRAMMAR CONTENTS\"] attribute"),
        1 => get_attribute(grammar[0]),
        _ => panic!("only 1 grammar file can be provided"),
    };

    (name, generics, argument)
}

fn get_attribute(attr: &Attribute) -> GrammarSource {
    match attr.parse_meta() {
        Ok(Meta::NameValue(name_value)) => match name_value.lit {
            Lit::Str(string) => {
                if name_value.path.is_ident("grammar") {
                    GrammarSource::File(string.value())
                } else {
                    GrammarSource::Inline(string.value())
                }
            }
            _ => panic!("grammar attribute must be a string"),
        },
        _ => panic!("grammar attribute must be of the form `grammar = \"...\"`"),
    }
}
