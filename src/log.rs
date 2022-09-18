use std::fs;
use crate::Pairs;

const LOG_FILE: &str = "log.log";

const LOG_ERROR: &str = "Couldn't write to log file";

pub fn log(content: &str) {
    let file_content = fs::read_to_string(LOG_FILE).unwrap();
    fs::write(LOG_FILE, format!("{}\n{}", content, file_content)).expect(LOG_ERROR);
}

pub fn clear() {
    fs::write(LOG_FILE, "").expect(LOG_ERROR);
}

pub fn format_tree(tree: Pairs, indent: usize) -> String {
    let mut lines = Vec::<String>::new();
    for node in tree {
        lines.push(format!(
            "{}{:?} ({})",
            "\t".repeat(indent),
            node.as_rule(),
            node.as_span().as_str()
        ));
        lines.push(format_tree(node.into_inner(), indent + 1));
    }
    lines.join("\n")
}
