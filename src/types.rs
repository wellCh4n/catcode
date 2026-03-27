#[derive(Debug, Clone)]
pub struct MethodInfo {
    pub signature: String,
    pub class_name: Option<String>,
    pub body: String,
    pub start_line: usize,
    pub end_line: usize,
    pub calls: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ClassInfo {
    pub name: String,
    pub kind: String,
    pub annotations: Vec<String>,
    pub superclass: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<String>,
    pub methods: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct CallerInfo {
    pub file: String,
    pub caller_class: Option<String>,
    pub caller_method: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
}
