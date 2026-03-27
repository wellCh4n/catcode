use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    [
      (function_declaration name: (identifier) @name) @method
      (method_definition name: (property_identifier) @name) @method
    ]
"#;

const CLASSES_QUERY: &str = r#"
    (class_declaration name: (identifier) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (call_expression function: [
      (identifier) @call
      (member_expression property: (property_identifier) @call)
    ])
"#;

pub struct JavaScriptParser(pub CoreParser);

impl JavaScriptParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_javascript::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for JavaScriptParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "class"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_declaration", "method_definition"]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        format!("{}{}", name, params)
    }
}
