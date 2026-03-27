use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_item name: (identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    [
      (struct_item name: (type_identifier) @name) @class
      (impl_item type: (type_identifier) @name) @class
    ]
"#;

const CALLS_QUERY: &str = r#"
    (call_expression function: [
      (identifier) @call
      (field_expression field: (field_identifier) @call)
      (scoped_identifier name: (identifier) @call)
    ])
"#;

pub struct RustLangParser(pub CoreParser);

impl RustLangParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_rust::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for RustLangParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["impl_item"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_item"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        // #[attr] nodes are preceding siblings of type attribute_item
        let parent = match node.parent() {
            Some(p) => p,
            None => return vec![],
        };
        let mut attrs = vec![];
        for child in parent.children(&mut parent.walk()) {
            if child == node {
                break;
            }
            if child.kind() == "attribute_item" {
                attrs.push(self.0.text(child).to_string());
            } else {
                // Only keep immediately preceding attributes
                attrs.clear();
            }
        }
        attrs
    }

    fn enclosing_class(&self, node: Node) -> Option<String> {
        let mut parent = node.parent();
        while let Some(p) = parent {
            if p.kind() == "impl_item" {
                if let Some(type_node) = p.child_by_field_name("type") {
                    return Some(self.0.text(type_node).to_string());
                }
            }
            parent = p.parent();
        }
        None
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let mut parts = vec![];

        // visibility modifier
        for child in node.children(&mut node.walk()) {
            if child.kind() == "visibility_modifier" {
                parts.push(core.text(child).to_string());
                break;
            }
        }

        parts.push("fn".to_string());

        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        parts.push(format!("{}{}", name, params));

        if let Some(ret) = node.child_by_field_name("return_type") {
            parts.push(format!("-> {}", core.text(ret)));
        }

        let sig = parts.join(" ");
        let attrs = self.get_annotations(node);
        if !attrs.is_empty() {
            format!("{} {}", attrs.join(" "), sig)
        } else {
            sig
        }
    }
}
