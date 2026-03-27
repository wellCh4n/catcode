use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_declaration (simple_identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class_declaration (type_identifier) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (call_expression calleeExpression: (simple_identifier) @call)
"#;

pub struct KotlinParser(pub CoreParser);

impl KotlinParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        // tree-sitter-kotlin is built against an older tree-sitter; transmute is safe
        // because both Language types are a newtype wrapper around *const TSLanguage
        let language: Language = unsafe {
            std::mem::transmute(tree_sitter_kotlin::language())
        };
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for KotlinParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration"]
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
        let mut sig = format!("fun {}{}", name, params);
        if let Some(ret) = node.child_by_field_name("type") {
            sig.push_str(&format!(": {}", core.text(ret)));
        }
        sig
    }
}
