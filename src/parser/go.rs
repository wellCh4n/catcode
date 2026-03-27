use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, QueryCursor};

use crate::types::ClassInfo;

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    [
      (function_declaration name: (identifier) @name) @method
      (method_declaration name: (field_identifier) @name) @method
    ]
"#;

const CLASSES_QUERY: &str = r#"
    (type_declaration
      (type_spec name: (type_identifier) @name
        type: (struct_type)) @class)
"#;

const CALLS_QUERY: &str = r#"
    (call_expression function: [
      (identifier) @call
      (selector_expression field: (field_identifier) @call)
    ])
"#;

pub struct GoParser(pub CoreParser);

impl GoParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_go::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for GoParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["type_spec"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_declaration", "method_declaration"]
    }

    fn enclosing_class(&self, node: Node) -> Option<String> {
        // Go: receiver field encodes the type
        let receiver = node.child_by_field_name("receiver")?;
        for child in receiver.children(&mut receiver.walk()) {
            if child.kind() == "parameter_declaration" {
                if let Some(type_node) = child.child_by_field_name("type") {
                    if type_node.kind() == "pointer_type" {
                        // *T -> find named child
                        for t in type_node.children(&mut type_node.walk()) {
                            if t.is_named() {
                                return Some(self.0.text(t).to_string());
                            }
                        }
                    } else {
                        return Some(self.0.text(type_node).to_string());
                    }
                }
            }
        }
        None
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let mut parts = vec!["func".to_string()];
        if let Some(receiver) = node.child_by_field_name("receiver") {
            parts.push(core.text(receiver).to_string());
        }
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        parts.push(format!("{}{}", name, params));
        if let Some(result) = node.child_by_field_name("result") {
            parts.push(core.text(result).to_string());
        }
        parts.join(" ")
    }

    fn get_class_skeleton(&self, name: &str) -> Option<ClassInfo> {
        let core = &self.0;
        let classes_q = core.classes_query.as_ref()?;

        let class_idx = classes_q.capture_index_for_name("class")?;
        let name_idx = classes_q.capture_index_for_name("name")?;

        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(classes_q, core.tree.root_node(), core.source.as_slice());

        let mut type_spec_node = None;
        while let Some(m) = matches.next() {
            let mut found_name = false;
            let mut cn = None;
            for cap in m.captures {
                if cap.index == name_idx && core.text(cap.node) == name {
                    found_name = true;
                }
                if cap.index == class_idx {
                    cn = Some(cap.node);
                }
            }
            if found_name {
                type_spec_node = cn;
                break;
            }
        }

        let type_spec_node = type_spec_node?;

        // Extract struct fields
        let mut fields = vec![];
        if let Some(struct_type) = type_spec_node.child_by_field_name("type") {
            if struct_type.kind() == "struct_type" {
                for child in struct_type.children(&mut struct_type.walk()) {
                    if child.kind() == "field_declaration_list" {
                        for decl in child.children(&mut child.walk()) {
                            if decl.kind() == "field_declaration" {
                                fields.push(core.text(decl).to_string());
                            }
                        }
                    }
                }
            }
        }

        // Collect methods whose receiver type matches
        let methods: Vec<String> = core
            .method_nodes()
            .into_iter()
            .filter(|n| self.enclosing_class(*n).as_deref() == Some(name))
            .map(|n| self.build_signature(n))
            .collect();

        Some(ClassInfo {
            name: name.to_string(),
            kind: "struct".to_string(),
            annotations: vec![],
            superclass: None,
            interfaces: vec![],
            fields,
            methods,
        })
    }
}
