use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_declaration name: (simple_identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    [
      (class_declaration name: (type_identifier) @name) @class
      (struct_declaration name: (type_identifier) @name) @class
    ]
"#;

const CALLS_QUERY: &str = r#"
    (call_expression function: (simple_identifier) @call)
"#;

pub struct SwiftParser(pub CoreParser);

impl SwiftParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_swift::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for SwiftParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "struct_declaration"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_declaration"]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("function_value_parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        let mut sig = format!("func {}{}", name, params);
        if let Some(ret) = node.child_by_field_name("return_type") {
            sig.push_str(&format!(" -> {}", core.text(ret)));
        }
        sig
    }
}
