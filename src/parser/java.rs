use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, QueryCursor};

use crate::types::ClassInfo;

use super::base::{CoreParser, LanguageParser};

const QUERY: &str = r#"
    [
      (method_declaration name: (identifier) @name) @method
      (constructor_declaration name: (identifier) @name) @method
    ]
"#;

const CLASSES_QUERY: &str = r#"
    [
      (class_declaration name: (identifier) @name) @class
      (interface_declaration name: (identifier) @name) @class
      (enum_declaration name: (identifier) @name) @class
    ]
"#;

const CALLS_QUERY: &str = r#"
    (method_invocation name: (identifier) @call)
"#;

pub struct JavaParser(pub CoreParser);

impl JavaParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_java::LANGUAGE);
        Ok(Self(CoreParser::new(
            source,
            language,
            QUERY,
            Some(CLASSES_QUERY),
            Some(CALLS_QUERY),
        )?))
    }
}

impl LanguageParser for JavaParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_declaration", "interface_declaration", "enum_declaration"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["method_declaration", "constructor_declaration"]
    }

    fn get_annotations(&self, node: Node) -> Vec<String> {
        // Annotations are children of the modifiers node
        let mut annotations = vec![];
        for child in node.children(&mut node.walk()) {
            if child.kind() == "modifiers" {
                for mod_child in child.children(&mut child.walk()) {
                    if mod_child.kind() == "marker_annotation"
                        || mod_child.kind() == "annotation"
                    {
                        annotations.push(self.0.text(mod_child).to_string());
                    }
                }
            }
        }
        annotations
    }

    fn build_signature(&self, node: Node) -> String {
        let core = &self.0;
        let mut parts = vec![];

        // modifiers (public, static, @Annotations, etc.)
        for child in node.children(&mut node.walk()) {
            if child.kind() == "modifiers" {
                parts.push(core.text(child).to_string());
                break;
            }
        }

        // return type (not for constructors)
        if node.kind() == "method_declaration" {
            if let Some(ret) = node.child_by_field_name("type") {
                parts.push(core.text(ret).to_string());
            }
        }

        // name + parameters
        let name = node.child_by_field_name("name").map(|n| core.text(n)).unwrap_or("");
        let params = node
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        parts.push(format!("{}{}", name, params));

        // throws
        if let Some(throws) = node.child_by_field_name("throws") {
            parts.push(format!("throws {}", core.text(throws)));
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

        let mut class_node = None;
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
                class_node = cn;
                break;
            }
        }

        let class_node = class_node?;

        let kind = match class_node.kind() {
            "interface_declaration" => "interface",
            "enum_declaration" => "enum",
            _ => "class",
        }
        .to_string();

        // Class-level annotations (modifiers before the class declaration)
        let mut class_annotations = vec![];
        if let Some(mods) = class_node.child_by_field_name("modifiers") {
            for child in mods.children(&mut mods.walk()) {
                if child.kind() == "marker_annotation" || child.kind() == "annotation" {
                    class_annotations.push(core.text(child).to_string());
                }
            }
        }

        // superclass
        let superclass = class_node
            .child_by_field_name("superclass")
            .map(|n| core.text(n).trim_start_matches("extends ").to_string());

        // interfaces
        let mut interfaces = vec![];
        if let Some(iface) = class_node.child_by_field_name("interfaces") {
            let text = core.text(iface);
            // "implements Foo, Bar" -> ["Foo", "Bar"]
            let trimmed = text.trim_start_matches("implements").trim();
            for part in trimmed.split(',') {
                let s = part.trim().to_string();
                if !s.is_empty() {
                    interfaces.push(s);
                }
            }
        }

        // fields and methods from body
        let mut fields = vec![];
        let mut methods = vec![];

        if let Some(body) = class_node.child_by_field_name("body") {
            for child in body.children(&mut body.walk()) {
                match child.kind() {
                    "field_declaration" => {
                        fields.push(core.text(child).to_string());
                    }
                    "method_declaration" | "constructor_declaration" => {
                        methods.push(self.build_signature(child));
                    }
                    _ => {}
                }
            }
        }

        Some(ClassInfo {
            name: name.to_string(),
            kind,
            annotations: class_annotations,
            superclass,
            interfaces,
            fields,
            methods,
        })
    }

    fn extract_fields(&self, class_node: Node) -> Vec<String> {
        let core = &self.0;
        let mut fields = vec![];
        if let Some(body) = class_node.child_by_field_name("body") {
            for child in body.children(&mut body.walk()) {
                if child.kind() == "field_declaration" {
                    fields.push(core.text(child).to_string());
                }
            }
        }
        fields
    }
}
