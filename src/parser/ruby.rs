use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (method name: (identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class name: (constant) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (call method: (identifier) @call)
"#;

pub struct RubyParser(pub CoreParser);

impl RubyParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_ruby::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for RubyParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["method"]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("");
        format!("def {}{}", name, params)
    }
}
