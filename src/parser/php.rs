use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_definition name: (name) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class_declaration name: (name) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (function_call_expression function: (name) @call)
"#;

pub struct PhpParser(pub CoreParser);

impl PhpParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_php::LANGUAGE_PHP);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for PhpParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_definition", "method_declaration"]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        format!("function {}{}", name, params)
    }
}
