use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    [
      (function_declaration name: (identifier) @name) @method
      (method_definition name: (property_identifier) @name) @method
      (method_definition name: (private_property_identifier) @name) @method
    ]
"#;

const CLASSES_QUERY: &str = r#"
    (class_declaration name: (type_identifier) @name) @class
"#;

const CALLS_QUERY: &str = r#"
    (call_expression function: [
      (identifier) @call
      (member_expression property: (property_identifier) @call)
    ])
"#;

pub struct TypeScriptParser(pub CoreParser);
pub struct TsxParser(pub CoreParser);

impl TypeScriptParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_typescript::LANGUAGE_TYPESCRIPT);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl TsxParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_typescript::LANGUAGE_TSX);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

fn get_ts_annotations<'a>(core: &'a CoreParser, node: Node<'a>) -> Vec<String> {
    // Decorators are preceding siblings of type "decorator"
    let parent = match node.parent() {
        Some(p) => p,
        None => return vec![],
    };
    let mut attrs = vec![];
    for child in parent.children(&mut parent.walk()) {
        if child == node {
            break;
        }
        if child.kind() == "decorator" {
            attrs.push(core.text(child).to_string());
        } else {
            attrs.clear();
        }
    }
    attrs
}

fn build_ts_signature(core: &CoreParser, node: Node) -> String {
    let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
    let params = node
        .child_by_field_name("parameters")
        .map(|n| core.text(n))
        .unwrap_or("()");
    let mut sig = format!("{}{}", name, params);
    if let Some(ret) = node.child_by_field_name("return_type") {
        sig.push_str(&format!(": {}", core.text(ret)));
    }
    sig
}

impl LanguageParser for TypeScriptParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "class"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_declaration", "method_definition"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        get_ts_annotations(&self.0, node)
    }

    fn build_signature(&self, node: Node) -> String {
        build_ts_signature(&self.0, node)
    }
}

impl LanguageParser for TsxParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "class"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_declaration", "method_definition"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        get_ts_annotations(&self.0, node)
    }

    fn build_signature(&self, node: Node) -> String {
        build_ts_signature(&self.0, node)
    }
}
