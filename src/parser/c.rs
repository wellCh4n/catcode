use anyhow::Result;
use tree_sitter::{Language, Node};

use super::base::{CoreParser, LanguageParser};

const C_QUERY: &str = r#"
    (function_definition
      declarator: (function_declarator
        declarator: (identifier) @name)) @method
"#;

const CPP_QUERY: &str = r#"
    (function_definition
      declarator: [
        (function_declarator declarator: (identifier) @name)
        (function_declarator declarator: (qualified_identifier name: (identifier) @name))
      ]) @method
"#;

pub struct CParser(pub CoreParser);
pub struct CppParser(pub CoreParser);

impl CParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_c::LANGUAGE);
        Ok(Self(CoreParser::new(source, language, C_QUERY, None, None)?))
    }
}

impl CppParser {
    pub fn new(source: Vec<u8>) -> Result<Self> {
        let language = Language::from(tree_sitter_cpp::LANGUAGE);
        Ok(Self(CoreParser::new(source, language, CPP_QUERY, None, None)?))
    }
}

fn build_c_signature(core: &CoreParser, node: Node) -> String {
    let ret_type = node
        .child_by_field_name("type")
        .map(|n| core.text(n))
        .unwrap_or("");
    let declarator = node.child_by_field_name("declarator");
    let sig = if let Some(decl) = declarator {
        let name = decl.child_by_field_name("declarator").map(|n| core.text(n)).unwrap_or("");
        let params = decl
            .child_by_field_name("parameters")
            .map(|n| core.text(n))
            .unwrap_or("()");
        format!("{}{}", name, params)
    } else {
        String::new()
    };
    format!("{} {}", ret_type, sig).trim().to_string()
}

impl LanguageParser for CParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_definition"]
    }

    fn build_signature(&self, node: Node) -> String {
        build_c_signature(&self.0, node)
    }
}

impl LanguageParser for CppParser {
    fn core(&self) -> &CoreParser {
        &self.0
    }

    fn class_node_types(&self) -> &[&str] {
        &["class_specifier"]
    }

    fn method_node_types(&self) -> &[&str] {
        &["function_definition"]
    }

    fn build_signature(&self, node: Node) -> String {
        build_c_signature(&self.0, node)
    }
}
