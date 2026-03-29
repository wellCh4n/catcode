use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, QueryCursor};

use crate::types::ImportInfo;

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    (function_definition name: (identifier) @name) @method
"#;

const CLASSES_QUERY: &str = r#"
    (class_definition name: (identifier) @name) @class
"#;

const IMPORTS_QUERY: &str = r#"
    (import_statement (dotted_name) @name) @import
    (import_from_statement (dotted_name) @name) @import
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
            Some(IMPORTS_QUERY),
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

    fn get_method_params(&self, node: Node) -> Vec<String> {
        let core = &self.0;
        let mut params = vec![];
        if let Some(params_node) = node.child_by_field_name("parameters") {
            for child in params_node.children(&mut params_node.walk()) {
                if child.kind() == "identifier" || child.kind() == "typed_parameter" || child.kind() == "default_parameter" {
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
        // Python: class Foo(Bar, Baz) - base classes in argument_list
        // Try superclass field first
        if let Some(bases) = class_node.child_by_field_name("superclass") {
            for child in bases.children(&mut bases.walk()) {
                if child.kind() == "identifier" {
                    result.push(core.text(child).to_string());
                }
            }
        }
        // If not found, search for argument_list directly (some tree-sitter versions)
        if result.is_empty() {
            for child in class_node.children(&mut class_node.walk()) {
                if child.kind() == "argument_list" {
                    for subchild in child.children(&mut child.walk()) {
                        if subchild.kind() == "identifier" {
                            result.push(core.text(subchild).to_string());
                        }
                    }
                }
            }
        }
        result
    }

    fn list_imports(&self) -> Vec<ImportInfo> {
        let core = &self.0;
        let imports_q = match &core.imports_query {
            Some(q) => q,
            None => return vec![],
        };
        let import_idx = match imports_q.capture_index_for_name("import") {
            Some(i) => i,
            None => return vec![],
        };
        let name_idx = imports_q.capture_index_for_name("name");

        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(imports_q, core.tree.root_node(), core.source.as_slice());

        let mut imports = vec![];
        while let Some(m) = matches.next() {
            let mut name = String::new();
            let mut line = 0;
            for cap in m.captures {
                if cap.index == import_idx {
                    line = cap.node.start_position().row + 1;
                }
                if let Some(name_i) = name_idx {
                    if cap.index == name_i {
                        name = core.text(cap.node).to_string();
                    }
                }
            }
            if !name.is_empty() {
                imports.push(ImportInfo {
                    _name: name.clone(),
                    path: name,
                    _is_wildcard: false,
                    line,
                });
            }
        }
        imports.sort_by_key(|i| i.line);
        imports
    }
}
