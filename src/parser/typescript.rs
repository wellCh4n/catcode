use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, QueryCursor};

use crate::types::ImportInfo;

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

const IMPORTS_QUERY: &str = r#"
    (import_statement) @import
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
            Some(IMPORTS_QUERY),
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
            Some(IMPORTS_QUERY),
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

    fn get_annotations(&self, node: Node) -> Vec<String> {
        get_ts_annotations(&self.0, node)
    }

    fn build_signature(&self, node: Node) -> String {
        build_ts_signature(&self.0, node)
    }

    fn get_method_params(&self, node: Node) -> Vec<String> {
        let core = &self.0;
        let mut params = vec![];
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for child in params_node.children(&mut params_node.walk()) {
                if child.kind() == "required_parameter" || child.kind() == "optional_parameter" || child.kind() == "rest_parameter" {
                    params.push(core.text(child).to_string());
                }
            }
        }
        params
    }

    fn get_method_return_type(&self, node: Node) -> Option<String> {
        node.child_by_field_name("return_type").map(|n| self.0.text(n).to_string())
    }

    fn get_extends(&self, class_node: Node) -> Vec<String> {
        let core = &self.0;
        let mut result = vec![];
        // TypeScript: class Foo extends Bar
        if let Some(extends_clause) = class_node.child_by_field_name("superclass") {
            result.push(core.text(extends_clause).to_string());
        }
        result
    }

    fn list_imports(&self) -> Vec<ImportInfo> {
        list_ts_imports(&self.0)
    }
}

impl LanguageParser for TsxParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "class"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        get_ts_annotations(&self.0, node)
    }

    fn build_signature(&self, node: Node) -> String {
        build_ts_signature(&self.0, node)
    }

    fn get_method_params(&self, node: Node) -> Vec<String> {
        let core = &self.0;
        let mut params = vec![];
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for child in params_node.children(&mut params_node.walk()) {
                if child.kind() == "required_parameter" || child.kind() == "optional_parameter" || child.kind() == "rest_parameter" {
                    params.push(core.text(child).to_string());
                }
            }
        }
        params
    }

    fn get_method_return_type(&self, node: Node) -> Option<String> {
        node.child_by_field_name("return_type").map(|n| self.0.text(n).to_string())
    }

    fn get_extends(&self, class_node: Node) -> Vec<String> {
        let core = &self.0;
        let mut result = vec![];
        if let Some(extends_clause) = class_node.child_by_field_name("superclass") {
            result.push(core.text(extends_clause).to_string());
        }
        result
    }

    fn list_imports(&self) -> Vec<ImportInfo> {
        list_ts_imports(&self.0)
    }
}

fn list_ts_imports(core: &CoreParser) -> Vec<ImportInfo> {
    let imports_q = match &core.imports_query {
        Some(q) => q,
        None => return vec![],
    };
    let import_idx = match imports_q.capture_index_for_name("import") {
        Some(i) => i,
        None => return vec![],
    };

    let mut cursor = QueryCursor::new();
    let mut matches =
        cursor.matches(imports_q, core.tree.root_node(), core.source.as_slice());

    let mut imports = vec![];
    while let Some(m) = matches.next() {
        for cap in m.captures {
            if cap.index == import_idx {
                let line = cap.node.start_position().row + 1;
                let text = core.text(cap.node);
                imports.push(ImportInfo {
                    _name: text.to_string(),
                    path: text.to_string(),
                    _is_wildcard: false,
                    line,
                });
            }
        }
    }
    imports.sort_by_key(|i| i.line);
    imports
}
