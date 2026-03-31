use std::path::Path;

use rust_code_analysis::{FuncSpace, SpaceKind, get_from_ext, get_function_spaces, LANG};

// ── field extraction ──────────────────────────────────────────────────────────

struct FieldInfo {
    name: String,
    ty: String,
    line: usize,
    /// For Rust struct fields: the name of the owning struct, so they can be
    /// shown under the matching `impl` block even though they live outside it.
    parent: Option<String>,
}

/// Returns node text as a trimmed string, collapsing inner whitespace to a
/// single space so multi-line type annotations stay readable on one line.
fn node_text(node: tree_sitter::Node, code: &[u8]) -> String {
    node.utf8_text(code)
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_fields(lang: &LANG, code: &[u8]) -> Vec<FieldInfo> {
    let ts_lang = match lang {
        LANG::Java => tree_sitter_java::language(),
        LANG::Rust => tree_sitter_rust::language(),
        LANG::Python => tree_sitter_python::language(),
        LANG::Javascript | LANG::Mozjs => tree_sitter_javascript::language(),
        LANG::Typescript | LANG::Tsx => tree_sitter_typescript::language_typescript(),
        LANG::Cpp => tree_sitter_mozcpp::language(),
        _ => return vec![],
    };

    let mut parser = tree_sitter::Parser::new();
    parser.set_language(ts_lang).expect("set language");
    let tree = match parser.parse(code, None) {
        Some(t) => t,
        None => return vec![],
    };

    let mut fields = Vec::new();
    collect_fields(lang, tree.root_node(), code, &mut fields);
    fields
}

fn collect_fields(lang: &LANG, node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    match lang {
        LANG::Java => collect_java_fields(node, code, out),
        LANG::Rust => collect_rust_fields(node, code, out),
        LANG::Javascript | LANG::Mozjs | LANG::Typescript | LANG::Tsx => {
            collect_js_fields(node, code, out)
        }
        LANG::Cpp => collect_cpp_fields(node, code, out),
        _ => {}
    }
}

fn collect_java_fields(node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    if node.kind() == "field_declaration" {
        let line = node.start_position().row + 1;
        let ty = node
            .child_by_field_name("type")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        let name = node
            .child_by_field_name("declarator")
            .and_then(|d| d.child_by_field_name("name"))
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        if !name.is_empty() {
            out.push(FieldInfo { name, ty, line, parent: None });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_java_fields(child, code, out);
    }
}

fn collect_rust_fields(node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    if node.kind() == "struct_item" {
        let parent = node
            .child_by_field_name("name")
            .map(|n| node_text(n, code));
        if let Some(body) = node.child_by_field_name("body") {
            let mut cursor = body.walk();
            for child in body.children(&mut cursor) {
                if child.kind() == "field_declaration" {
                    let line = child.start_position().row + 1;
                    let name = child
                        .child_by_field_name("name")
                        .map(|n| node_text(n, code))
                        .unwrap_or_default();
                    let ty = child
                        .child_by_field_name("type")
                        .map(|n| node_text(n, code))
                        .unwrap_or_default();
                    if !name.is_empty() {
                        out.push(FieldInfo { name, ty, line, parent: parent.clone() });
                    }
                }
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_rust_fields(child, code, out);
    }
}

fn collect_js_fields(node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    if matches!(node.kind(), "field_definition" | "public_field_definition") {
        let line = node.start_position().row + 1;
        let name = node
            .child_by_field_name("property")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        let ty = node
            .child_by_field_name("type")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        if !name.is_empty() {
            out.push(FieldInfo { name, ty, line, parent: None });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_js_fields(child, code, out);
    }
}

fn collect_cpp_fields(node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    if node.kind() == "field_declaration" {
        let line = node.start_position().row + 1;
        let ty = node
            .child_by_field_name("type")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        let name = node
            .child_by_field_name("declarator")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        if !name.is_empty() {
            out.push(FieldInfo { name, ty, line, parent: None });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_cpp_fields(child, code, out);
    }
}

// ── display ───────────────────────────────────────────────────────────────────

fn print_spaces(space: &FuncSpace, indent: usize, all_fields: &[FieldInfo]) {
    let prefix = "  ".repeat(indent);
    let name = space.name.as_deref().unwrap_or("<anonymous>");

    match space.kind {
        SpaceKind::Function => {
            println!(
                "{}fn {} (lines {}-{})",
                prefix, name, space.start_line, space.end_line
            );
        }
        SpaceKind::Class | SpaceKind::Struct | SpaceKind::Impl | SpaceKind::Interface => {
            println!(
                "{}{:?} {} (lines {}-{})",
                prefix, space.kind, name, space.start_line, space.end_line
            );

            // Collect fields that belong to this space.
            // Primary: fields within this space's line range but not inside a child space.
            // Secondary (Rust impl): fields whose `parent` name matches this space's name
            //   (struct fields live outside the impl block's line range).
            let child_ranges: Vec<(usize, usize)> = space
                .spaces
                .iter()
                .map(|s| (s.start_line, s.end_line))
                .collect();

            let mut own_fields: Vec<&FieldInfo> = all_fields
                .iter()
                .filter(|f| {
                    let by_range = f.line >= space.start_line
                        && f.line <= space.end_line
                        && !child_ranges.iter().any(|(s, e)| f.line >= *s && f.line <= *e);
                    let by_parent = space.kind == SpaceKind::Impl
                        && f.parent.as_deref() == Some(name);
                    by_range || by_parent
                })
                .collect();
            own_fields.sort_by_key(|f| f.line);

            for f in own_fields {
                if f.ty.is_empty() {
                    println!("{}  field {} (line {})", prefix, f.name, f.line);
                } else {
                    println!("{}  field {}: {} (line {})", prefix, f.name, f.ty, f.line);
                }
            }
        }
        SpaceKind::Unit => {}
        _ => {
            println!(
                "{}{:?} {} (lines {}-{})",
                prefix, space.kind, name, space.start_line, space.end_line
            );
        }
    }

    for child in &space.spaces {
        print_spaces(child, indent + 1, all_fields);
    }
}

// ── find / detail ─────────────────────────────────────────────────────────────

const CLASS_KINDS: &[SpaceKind] = &[
    SpaceKind::Class,
    SpaceKind::Struct,
    SpaceKind::Impl,
    SpaceKind::Interface,
];

fn find_all_spaces<'a>(
    space: &'a FuncSpace,
    name: &str,
    kind_filter: &[SpaceKind],
    results: &mut Vec<&'a FuncSpace>,
) {
    if space.name.as_deref() == Some(name) && kind_filter.contains(&space.kind) {
        results.push(space);
    }
    for child in &space.spaces {
        find_all_spaces(child, name, kind_filter, results);
    }
}

/// Find methods named `method_name` that are direct or nested children of a
/// class space named `class_name`.
fn find_methods_in_class<'a>(
    space: &'a FuncSpace,
    class_name: &str,
    method_name: &str,
    results: &mut Vec<&'a FuncSpace>,
) {
    if space.name.as_deref() == Some(class_name) && CLASS_KINDS.contains(&space.kind) {
        // Only match direct Function children — do not recurse into nested classes.
        for child in &space.spaces {
            if child.name.as_deref() == Some(method_name) && child.kind == SpaceKind::Function {
                results.push(child);
            }
        }
        return;
    }
    for child in &space.spaces {
        find_methods_in_class(child, class_name, method_name, results);
    }
}

fn print_method_detail(space: &FuncSpace, lines: &[&str]) {
    let name = space.name.as_deref().unwrap_or("<anonymous>");
    let start = space.start_line.saturating_sub(1);
    let end = space.end_line.min(lines.len());

    println!("Name:  {}", name);
    println!("Kind:  {:?}", space.kind);
    println!("Lines: {}-{}", space.start_line, space.end_line);
    println!("{}", "─".repeat(60));

    for (i, line) in lines[start..end].iter().enumerate() {
        println!("{:4} │ {}", start + i + 1, line);
    }
}

// ── main logic ────────────────────────────────────────────────────────────────

struct Query {
    class: Option<String>,
    method: Option<String>,
}

fn analyze_file(path: &Path, query: Option<&Query>) {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    let lang = match get_from_ext(ext) {
        Some(l) => l,
        None => {
            eprintln!("Unsupported file extension: .{}", ext);
            return;
        }
    };

    let code = match std::fs::read(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}", path.display(), e);
            return;
        }
    };

    let root = match get_function_spaces(&lang, code.clone(), path, None) {
        Some(r) => r,
        None => {
            println!("(no spaces found)");
            return;
        }
    };

    if let Some(q) = query {
        let mut matches = Vec::new();
        match (&q.class, &q.method) {
            (Some(class), Some(method)) => {
                find_methods_in_class(&root, class, method, &mut matches);
            }
            (None, Some(method)) => {
                find_all_spaces(&root, method, &[SpaceKind::Function], &mut matches);
            }
            (Some(class), None) => {
                find_all_spaces(&root, class, CLASS_KINDS, &mut matches);
            }
            (None, None) => {}
        }
        if matches.is_empty() {
            eprintln!("not found in {}", path.display());
        } else {
            let src = String::from_utf8_lossy(&code);
            let lines: Vec<&str> = src.lines().collect();
            for (i, space) in matches.iter().enumerate() {
                if i > 0 {
                    println!();
                }
                print_method_detail(space, &lines);
            }
        }
    } else {
        let fields = extract_fields(&lang, &code);
        println!("File: {} ({:?})", path.display(), lang);
        println!("{}", "─".repeat(60));
        print_spaces(&root, 0, &fields);
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: catcode <file> [file...] [-m <method>]");
        std::process::exit(1);
    }

    let mut files: Vec<&str> = Vec::new();
    let mut class: Option<String> = None;
    let mut method: Option<String> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-m" || args[i] == "-c" {
            let flag = args[i].clone();
            i += 1;
            if i < args.len() {
                let name = args[i].clone();
                if flag == "-m" {
                    method = Some(name);
                } else {
                    class = Some(name);
                }
            } else {
                eprintln!("{} requires a name", flag);
                std::process::exit(1);
            }
        } else {
            files.push(&args[i]);
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("Usage: catcode <file> [file...] [-c <class>] [-m <method>]");
        std::process::exit(1);
    }

    let query = if class.is_some() || method.is_some() {
        Some(Query { class, method })
    } else {
        None
    };

    for file in &files {
        analyze_file(Path::new(file), query.as_ref());
    }
}
