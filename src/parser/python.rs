use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_definition name: (identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class_definition name: (identifier) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (call function: [
      (identifier) @call
      (attribute attribute: (identifier) @call)
    ])
"#;

pub struct PythonParser(pub CoreParser);

impl PythonParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_python::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for PythonParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_definition"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_definition"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        // Decorators live in decorated_definition parent
        if let Some(parent) = node.parent() {
            if parent.kind() == "decorated_definition" {
                return parent
                    .children(&mut parent.walk())
                    .filter(|c| c.kind() == "decorator")
                    .map(|c| self.0.text(c).to_string())
                    .collect();
            }
        }
        vec![]
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        let mut sig = format!("def {}{}", name, params);
        if let Some(ret) = node.child_by_field_name("return_type") {
            sig.push_str(&format!(" -> {}", core.text(ret)));
        }
        let decorators = self.get_annotations(node);
        if !decorators.is_empty() {
            format!("{} {}", decorators.join(" "), sig)
        } else {
            sig
        }
    }
}
