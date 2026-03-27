use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (method_declaration name: (identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class_declaration name: (identifier) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (invocation_expression function: (member_access_expression name: (identifier) @call))
"#;

pub struct CSharpParser(pub CoreParser);

impl CSharpParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_c_sharp::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for CSharpParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["method_declaration"]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let mut parts = vec![];

        // modifiers
        for child in node.children(&mut node.walk()) {
            if child.kind() == "modifier" {
                parts.push(core.text(child).to_string());
            }
        }

        if let Some(ret) = node.child_by_field_name("type") {
            parts.push(core.text(ret).to_string());
        }

        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        parts.push(format!("{}{}", name, params));

        parts.join(" ")
    }
}
