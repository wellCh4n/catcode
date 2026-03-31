use std::path::Path;

use rust_code_analysis::{FuncSpace, SpaceKind, get_from_ext, get_function_spaces};

fn print_spaces(space: &FuncSpace, indent: usize) {
    let prefix = "  ".repeat(indent);
    let name = space.name.as_deref().unwrap_or("<anonymous>");

    match space.kind {
        SpaceKind::Function => {
            println!(
                "{}fn {} (lines {}-{})",
                prefix, name, space.start_line, space.end_line
            );
        }
        SpaceKind::Class => {
            println!(
                "{}class {} (lines {}-{})",
                prefix, name, space.start_line, space.end_line
            );
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
        print_spaces(child, indent + 1);
    }
}

fn find_all_spaces<'a>(space: &'a FuncSpace, name: &str, results: &mut Vec<&'a FuncSpace>) {
    if space.name.as_deref() == Some(name) && matches!(space.kind, SpaceKind::Function | SpaceKind::Class) {
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
        println!("File: {} ({:?})", path.display(), lang);
        println!("{}", "─".repeat(60));
        print_spaces(&root, 0);
        println!();
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: catcode <file> [file...] [-m <method>]");
        std::process::exit(1);
    }

    // Split args into files and optional -m <method>
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
