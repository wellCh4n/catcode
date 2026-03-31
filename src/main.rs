use std::path::Path;

use rust_code_analysis::{FuncSpace, SpaceKind, get_from_ext, get_function_spaces, LANG};

// ── field extraction ──────────────────────────────────────────────────────────

struct FieldInfo {
    name: String,
    ty: String,
    line: usize,
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
            out.push(FieldInfo { name, ty, line });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_java_fields(child, code, out);
    }
}

fn collect_rust_fields(node: tree_sitter::Node, code: &[u8], out: &mut Vec<FieldInfo>) {
    if node.kind() == "field_declaration" {
        let line = node.start_position().row + 1;
        let name = node
            .child_by_field_name("name")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        let ty = node
            .child_by_field_name("type")
            .map(|n| node_text(n, code))
            .unwrap_or_default();
        if !name.is_empty() {
            out.push(FieldInfo { name, ty, line });
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
            out.push(FieldInfo { name, ty, line });
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
            out.push(FieldInfo { name, ty, line });
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

            // Collect fields that belong to this space but not to any child space
            let child_ranges: Vec<(usize, usize)> = space
                .spaces
                .iter()
                .map(|s| (s.start_line, s.end_line))
                .collect();

            let mut own_fields: Vec<&FieldInfo> = all_fields
                .iter()
                .filter(|f| {
                    f.line >= space.start_line
                        && f.line <= space.end_line
                        && !child_ranges.iter().any(|(s, e)| f.line >= *s && f.line <= *e)
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

fn find_all_spaces<'a>(space: &'a FuncSpace, name: &str, results: &mut Vec<&'a FuncSpace>) {
    if space.name.as_deref() == Some(name)
        && matches!(space.kind, SpaceKind::Function | SpaceKind::Class)
    {
        results.push(space);
    }
    for child in &space.spaces {
        find_all_spaces(child, name, results);
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

fn analyze_file(path: &Path, method: Option<&str>) {
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

    if let Some(name) = method {
        let mut matches = Vec::new();
        find_all_spaces(&root, name, &mut matches);
        if matches.is_empty() {
            eprintln!("Method '{}' not found in {}", name, path.display());
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
    let mut method: Option<&str> = None;
    let mut i = 0;
    while i < args.len() {
        if args[i] == "-m" {
            i += 1;
            if i < args.len() {
                method = Some(&args[i]);
            } else {
                eprintln!("-m requires a method name");
                std::process::exit(1);
            }
        } else {
            files.push(&args[i]);
        }
        i += 1;
    }

    if files.is_empty() {
        eprintln!("Usage: catcode <file> [file...] [-m <method>]");
        std::process::exit(1);
    }

    for file in &files {
        analyze_file(Path::new(file), method);
    }
}
