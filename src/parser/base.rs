use anyhow::Result;
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor, Tree};

use crate::types::{CallerInfo, ClassInfo, MethodInfo};

pub struct CoreParser {
    pub source: Vec<u8>,
    pub tree: Tree,
    pub query: Query,
    pub classes_query: Option<Query>,
    pub calls_query: Option<Query>,
    #[allow(dead_code)]
    pub language: Language,
}

impl CoreParser {
    pub fn new(
        source: Vec<u8>,
        language: Language,
        query_str: &str,
        classes_query_str: Option<&str>,
        calls_query_str: Option<&str>,
    ) -> Result<Self> {
        let mut parser = Parser::new();
        parser.set_language(&language)?;
        let tree = parser
            .parse(&source, None)
            .ok_or_else(|| anyhow::anyhow!("parse failed"))?;
        let query = Query::new(&language, query_str)?;
        let classes_query = classes_query_str
            .map(|s| Query::new(&language, s))
            .transpose()?;
        let calls_query = calls_query_str
            .map(|s| Query::new(&language, s))
            .transpose()?;
        Ok(Self {
            source,
            tree,
            query,
            classes_query,
            calls_query,
            language,
        })
    }

    pub fn text<'a>(&'a self, node: Node<'a>) -> &'a str {
        node.utf8_text(&self.source).unwrap_or("")
    }

    /// Collect all @method nodes from the main query.
    pub fn method_nodes(&self) -> Vec<Node<'_>> {
        let method_idx = match self.query.capture_index_for_name("method") {
            Some(i) => i,
            None => return vec![],
        };
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&self.query, self.tree.root_node(), self.source.as_slice());
        let mut nodes = vec![];
        while let Some(m) = matches.next() {
            for cap in m.captures {
                if cap.index == method_idx {
                    nodes.push(cap.node);
                }
            }
        }
        nodes
    }

    /// Get the name of a method node via the @name capture in a match.
    /// Falls back to child_by_field_name("name").
    pub fn method_name_from_node<'a>(&'a self, node: Node<'a>) -> Option<&'a str> {
        if let Some(name_node) = node.child_by_field_name("name") {
            return Some(self.text(name_node));
        }
        None
    }

    /// Get calls within a node's byte range using the calls_query.
    pub fn get_calls(&self, node: Node) -> Vec<String> {
        let calls_q = match &self.calls_query {
            Some(q) => q,
            None => return vec![],
        };
        let call_idx = match calls_q.capture_index_for_name("call") {
            Some(i) => i,
            None => return vec![],
        };
        let mut cursor = QueryCursor::new();
        cursor.set_byte_range(node.byte_range());
        let mut matches =
            cursor.matches(calls_q, self.tree.root_node(), self.source.as_slice());
        let mut calls = vec![];
        while let Some(m) = matches.next() {
            for cap in m.captures {
                if cap.index == call_idx {
                    calls.push(self.text(cap.node).to_string());
                }
            }
        }
        calls.sort();
        calls.dedup();
        calls
    }
}

/// Trait that every language parser must implement.
pub trait LanguageParser: Send + Sync {
    fn core(&self) -> &CoreParser;

    fn class_node_types(&self) -> &[&str] {
        &[]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_definition", "method_definition", "function_declaration"]
    }

    fn build_signature(&self, node: Node) -> String;

    fn get_annotations(&self, node: Node) -> Vec<String> {
        let _ = node;
        vec![]
    }

    /// Walk parent chain to find enclosing class name.
    fn enclosing_class(&self, node: Node) -> Option<String> {
        let class_types = self.class_node_types();
        if class_types.is_empty() {
            return None;
        }
        let mut parent = node.parent();
        while let Some(p) = parent {
            if class_types.contains(&p.kind()) {
                if let Some(name_node) = p.child_by_field_name("name") {
                    return Some(self.core().text(name_node).to_string());
                }
            }
            parent = p.parent();
        }
        None
    }

    fn list_methods(&self) -> Vec<(Option<String>, String)> {
        let core = self.core();
        let mut results = vec![];
        for node in core.method_nodes() {
            let sig = self.build_signature(node);
            let class = self.enclosing_class(node);
            results.push((class, sig));
        }
        results
    }

    fn get_method(&self, name: &str) -> Option<MethodInfo> {
        let core = self.core();
        for node in core.method_nodes() {
            let node_name = core.method_name_from_node(node)?;
            if node_name == name {
                let sig = self.build_signature(node);
                let class_name = self.enclosing_class(node);
                let body = core.text(node).to_string();
                let start_line = node.start_position().row + 1;
                let end_line = node.end_position().row + 1;
                let calls = core.get_calls(node);
                return Some(MethodInfo {
                    signature: sig,
                    class_name,
                    body,
                    start_line,
                    end_line,
                    calls,
                });
            }
        }
        None
    }

    fn list_classes(&self) -> Vec<String> {
        let core = self.core();
        let classes_q = match &core.classes_query {
            Some(q) => q,
            None => return vec![],
        };
        let name_idx = match classes_q.capture_index_for_name("name") {
            Some(i) => i,
            None => return vec![],
        };
        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(classes_q, core.tree.root_node(), core.source.as_slice());
        let mut names = vec![];
        while let Some(m) = matches.next() {
            for cap in m.captures {
                if cap.index == name_idx {
                    names.push(core.text(cap.node).to_string());
                }
            }
        }
        names.sort();
        names.dedup();
        names
    }

    fn get_class_skeleton(&self, name: &str) -> Option<ClassInfo> {
        let core = self.core();
        let classes_q = core.classes_query.as_ref()?;

        let class_idx = classes_q.capture_index_for_name("class")?;
        let name_idx = classes_q.capture_index_for_name("name")?;

        // Find matching class/name captures in the same match
        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(classes_q, core.tree.root_node(), core.source.as_slice());

        let mut class_node: Option<Node> = None;
        while let Some(m) = matches.next() {
            let mut found_name = false;
            let mut found_class_node: Option<Node> = None;
            for cap in m.captures {
                if cap.index == name_idx && core.text(cap.node) == name {
                    found_name = true;
                }
                if cap.index == class_idx {
                    found_class_node = Some(cap.node);
                }
            }
            if found_name {
                class_node = found_class_node;
                break;
            }
        }

        let class_node = class_node?;

        // Collect fields: children named "field" or type "field_declaration"
        let fields = self.extract_fields(class_node);

        // Collect methods whose enclosing class is this one
        let methods: Vec<String> = core
            .method_nodes()
            .into_iter()
            .filter(|n| self.enclosing_class(*n).as_deref() == Some(name))
            .map(|n| self.build_signature(n))
            .collect();

        Some(ClassInfo {
            name: name.to_string(),
            kind: self.class_kind(class_node),
            annotations: self.get_class_annotations(class_node),
            superclass: self.get_superclass(class_node),
            interfaces: self.get_interfaces(class_node),
            fields,
            methods,
        })
    }

    fn extract_fields(&self, class_node: Node) -> Vec<String> {
        let _ = class_node;
        vec![]
    }

    fn class_kind(&self, class_node: Node) -> String {
        match class_node.kind() {
            "interface_declaration" => "interface".to_string(),
            "enum_declaration" => "enum".to_string(),
            "struct_item" | "struct_type" => "struct".to_string(),
            _ => "class".to_string(),
        }
    }

    fn get_class_annotations(&self, class_node: Node) -> Vec<String> {
        let _ = class_node;
        vec![]
    }

    fn get_superclass(&self, class_node: Node) -> Option<String> {
        let _ = class_node;
        None
    }

    fn get_interfaces(&self, class_node: Node) -> Vec<String> {
        let _ = class_node;
        vec![]
    }

    fn find_callers(&self, method_name: &str) -> Vec<CallerInfo> {
        let core = self.core();
        let calls_q = match &core.calls_query {
            Some(q) => q,
            None => return vec![],
        };
        let call_idx = match calls_q.capture_index_for_name("call") {
            Some(i) => i,
            None => return vec![],
        };

        let mut cursor = QueryCursor::new();
        let mut matches =
            cursor.matches(calls_q, core.tree.root_node(), core.source.as_slice());

        let mut results = vec![];
        while let Some(m) = matches.next() {
            for cap in m.captures {
                if cap.index == call_idx && core.text(cap.node) == method_name {
                    let call_node = cap.node;
                    // Find enclosing method
                    let (caller_method, caller_class, start_line, end_line) =
                        self.find_enclosing_method(call_node);
                    results.push(CallerInfo {
                        file: String::new(), // filled in by scanner
                        caller_class,
                        caller_method,
                        start_line,
                        end_line,
                    });
                }
            }
        }
        results
    }

    fn find_enclosing_method(
        &self,
        node: Node,
    ) -> (Option<String>, Option<String>, usize, usize) {
        let method_types = self.method_node_types();
        let mut parent = node.parent();
        while let Some(p) = parent {
            if method_types.contains(&p.kind()) {
                let core = self.core();
                let name = core
                    .method_name_from_node(p)
                    .map(|s| s.to_string());
                let class = self.enclosing_class(p);
                let start = p.start_position().row + 1;
                let end = p.end_position().row + 1;
                return (name, class, start, end);
            }
            parent = p.parent();
        }
        (None, None, node.start_position().row + 1, node.end_position().row + 1)
    }
}
