use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct MethodInfo {
    pub signature: String,
    pub class_name: Option<String>,
    pub body: String,
    pub start_line: usize,
    pub end_line: usize,
    pub params: Vec<String>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ClassInfo {
    pub name: String,
    pub kind: String,
    pub annotations: Vec<String>,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub extends: Vec<String>,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportInfo {
    #[serde(skip)]
    pub _name: String,
    pub path: String,
    #[serde(skip)]
    pub _is_wildcard: bool,
    pub line: usize,
}

