mod ext_map;
mod parser;
mod types;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use clap::Parser as ClapParser;
use serde_json::json;
use walkdir::WalkDir;

use ext_map::ext_to_lang;
use parser::make_parser;
use types::{ClassInfo, MethodInfo};

#[derive(ClapParser)]
#[command(name = "catcode", about = "Progressive code reading for LLMs")]
struct Cli {
    /// Source file
    #[arg(short = 'f', long)]
    file: Option<String>,

    /// Directory to scan recursively
    #[arg(short = 'd', long)]
    dir: Option<String>,

    /// Max files per language to process (default: 100)
    #[arg(short = 'x', long)]
    max_files: Option<usize>,

    /// Max methods per file to list (default: 50, 0 = unlimited)
    #[arg(short = 'X', long)]
    max_methods: Option<usize>,

    /// Method name to look up
    #[arg(short = 'm', long)]
    method: Option<String>,

    /// Class name (omit value to list all)
    #[arg(short = 'c', long, num_args = 0..=1, default_missing_value = "__list__")]
    class: Option<String>,

    /// Show imports
    #[arg(short = 'i', long)]
    imports: bool,

    /// Show class inheritance hierarchy (no arg = all classes)
    #[arg(short = 'I', long, action = clap::ArgAction::Set, default_value = None, num_args = 0..=1, default_missing_value = "")]
    inheritance: Option<String>,

    /// Output JSON format
    #[arg(long)]
    json: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Validate mutual exclusivity of file and dir
    if cli.file.is_some() && cli.dir.is_some() {
        eprintln!("Error: use either -f FILE or -d DIR, not both");
        std::process::exit(1);
    }

    if let Some(dir) = &cli.dir {
        scan_directory(dir, &cli)?;
    } else if let Some(file) = &cli.file {
        handle_file(file, &cli)?;
    } else {
        eprintln!("Error: provide -f FILE or -d DIR");
        std::process::exit(1);
    }

    Ok(())
}

fn scan_directory(dir_path: &str, cli: &Cli) -> Result<()> {
    let max_files = cli.max_files.unwrap_or(100);

    // Skip these common non-source directories
    let exclude_dirs = ["target", "node_modules", ".git", ".idea", ".vscode", "build", "dist", ".gradle", ".maven"];

    fn path_has_excluded(path: &std::path::Path, dir_path: &str, exclude_dirs: &[&str]) -> bool {
        let relative = match path.strip_prefix(dir_path) {
            Ok(r) => r,
            Err(_) => return false,
        };
        relative.components().any(|c| {
            let name = c.as_os_str().to_str().unwrap_or("");
            exclude_dirs.contains(&name)
        })
    }

    // Collect files by language
    let mut files_by_lang: HashMap<&str, Vec<String>> = HashMap::new();

    let mut count = 0;
    for entry in WalkDir::new(dir_path)
        .follow_links(false)
        .into_iter()
        .filter_entry(|e| {
            let result = !path_has_excluded(e.path(), dir_path, &exclude_dirs);
            count += 1;
            result
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if !path.is_file() {
            continue;
        }

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let Some(lang) = ext_to_lang(ext) else {
            continue;
        };

        files_by_lang
            .entry(lang)
            .or_default()
            .push(path.to_string_lossy().to_string());
    }

    eprintln!("WalkDir visited {} entries, found {} source files",
        count, files_by_lang.values().map(|v| v.len()).sum::<usize>());

    if files_by_lang.is_empty() {
        println!("No supported source files found in {}", dir_path);
        return Ok(());
    }

    // Process each language group
    for (lang, files) in &files_by_lang {
        let total = files.len();
        let files_to_process = files.iter().take(max_files);

        println!("\n# {} ({} files)\n", lang.to_uppercase(), total);

        for file in files_to_process {
            if let Err(e) = handle_file(file, cli) {
                eprintln!("Error processing {}: {}", file, e);
            }
        }

        if total > max_files {
            println!("\n... and {} more {} files", total - max_files, lang);
        }
    }

    Ok(())
}

fn handle_file(file: &str, cli: &Cli) -> Result<()> {
    let path = Path::new(file);
    let ext = path
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let lang = ext_to_lang(ext).ok_or_else(|| anyhow::anyhow!("unsupported file type: {}", ext))?;
    let source = std::fs::read(path)?;
    let parser = make_parser(lang, source)?;

    // Handle imports flag
    if cli.imports {
        let imports = parser.list_imports();
        if cli.json {
            println!("{}", serde_json::to_string_pretty(&imports)?);
        } else if imports.is_empty() {
            println!("No imports found");
        } else {
            println!("## Imports\n");
            for imp in &imports {
                println!("- `{}` (L{})", imp.path, imp.line);
            }
        }
        return Ok(());
    }

    // Handle inheritance flag
    match &cli.inheritance {
        Some(name) if !name.is_empty() => {
            // -I <name>: show specific class inheritance
            let classes = parser.list_classes();
            if !classes.contains(name) {
                println!("class '{}' not found in {}", name, file);
                return Ok(());
            }
            if let Some(info) = parser.get_class_skeleton(name) {
                if cli.json {
                    println!("{}", serde_json::to_string_pretty(&info)?);
                } else {
                    print_inheritance(&info);
                }
            }
            return Ok(());
        }
        Some(_) => {
            // -I without arg: show all inheritance relationships
            let classes = parser.list_classes();
            if classes.is_empty() {
                if cli.json {
                    println!("[]");
                } else {
                    println!("No classes found in {}", file);
                }
                return Ok(());
            }
            let mut all_inheritance = vec![];
            for class_name in &classes {
                if let Some(info) = parser.get_class_skeleton(class_name) {
                    if !info.extends.is_empty() || !info.interfaces.is_empty() {
                        if cli.json {
                            all_inheritance.push(json!({
                                "name": info.name,
                                "extends": info.extends,
                                "interfaces": info.interfaces,
                            }));
                        } else {
                            print_inheritance_summary(&info);
                        }
                    }
                }
            }
            if cli.json {
                println!("{}", serde_json::to_string_pretty(&all_inheritance)?);
            }
            return Ok(());
        }
        None => {
            // Flag not provided - do nothing
        }
    }

    if let Some(name) = &cli.method {
        match parser.get_method(name) {
            Some(info) => {
                if cli.json {
                    let output = json!({
                        "file": file,
                        "class_name": info.class_name,
                        "signature": info.signature,
                        "start_line": info.start_line,
                        "end_line": info.end_line,
                        "params": info.params,
                        "return_type": info.return_type,
                        "body": info.body,
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    print_method_detail(file, name, &info);
                }
            }
            None => println!("method '{}' not found in {}", name, file),
        }
        return Ok(());
    }

    match &cli.class {
        Some(name) if name == "__list__" => {
            let classes = parser.list_classes();
            if cli.json {
                let output: Vec<_> = classes.iter().map(|c| json!({"name": c})).collect();
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else if classes.is_empty() {
                println!("No classes found");
            } else {
                println!("## Classes\n");
                for cls in &classes {
                    println!("- `{}`", cls);
                }
            }
        }
        Some(name) => {
            match parser.get_class_skeleton(name) {
                Some(info) => {
                    if cli.json {
                        println!("{}", serde_json::to_string_pretty(&info)?);
                    } else {
                        print_class_skeleton(&info);
                    }
                }
                None => println!("class '{}' not found in {}", name, file),
            }
        }
        None => {}
    }

    // Default: list all methods when no flag given
    if cli.method.is_none() && cli.class.is_none() && !cli.imports && cli.inheritance.is_none() {
        if cli.json {
            let methods = parser.list_methods();
            let output: Vec<_> = methods.iter().map(|(c, s)| json!({
                "class_name": c,
                "signature": s,
            })).collect();
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            print_method_list(file, &parser.list_methods(), cli.max_methods);
        }
    }

    Ok(())
}

fn print_method_list(_file: &str, methods: &[(Option<String>, String)], max_methods: Option<usize>) {
    if methods.is_empty() {
        return;
    }

    let total = methods.len();
    let methods_to_show: Vec<_> = match max_methods {
        Some(n) if n > 0 => methods.iter().take(n).collect(),
        _ => methods.iter().collect(),
    };

    // Group by class
    let mut current_class: Option<&str> = None;
    for (class, sig) in &methods_to_show {
        let cls = class.as_deref();
        if cls != current_class {
            if let Some(c) = cls {
                println!("### `{}`\n", c);
            }
            current_class = cls;
        }
        println!("- `{}`", sig);
    }

    if let Some(n) = max_methods {
        if n > 0 && total > n {
            println!("\n... and {} more methods", total - n);
        }
    }

    println!();
}

fn print_method_detail(file: &str, _name: &str, info: &MethodInfo) {
    let class_prefix = info
        .class_name
        .as_deref()
        .map(|c| format!("### `{}`\n\n", c))
        .unwrap_or_default();

    println!(
        "## {}\n\n{}**`{}`**\n\n> L{}-{}",
        file,
        class_prefix,
        info.signature,
        info.start_line,
        info.end_line
    );

    if !info.params.is_empty() {
        println!("\n**Parameters:**");
        for p in &info.params {
            println!("- `{}`", p);
        }
    }

    if let Some(ret) = &info.return_type {
        println!("\n**Return type:** `{}`", ret);
    }

    println!("\n```\n{}\n```", info.body);
    println!();
}

fn print_class_skeleton(info: &ClassInfo) {
    println!("## `{}`\n", info.name);

    if !info.annotations.is_empty() {
        println!("{}", info.annotations.join("\n"));
    }

    let mut header = format!("{} {}", info.kind, info.name);
    if let Some(sup) = &info.superclass {
        header.push_str(&format!(" extends {}", sup));
    }
    if !info.interfaces.is_empty() {
        header.push_str(&format!(" implements {}", info.interfaces.join(", ")));
    }
    println!("**`{}`**\n", header);

    if !info.fields.is_empty() {
        println!("**Fields:**\n");
        for f in &info.fields {
            println!("- `{}`", f.trim());
        }
        println!();
    }

    if !info.methods.is_empty() {
        println!("**Methods:**\n");
        for m in &info.methods {
            println!("- `{}`", m);
        }
        println!();
    }
}

fn print_inheritance(info: &ClassInfo) {
    println!("## `{}`\n", info.name);

    let mut header = format!("{} {}", info.kind, info.name);
    if !info.extends.is_empty() {
        header.push_str(&format!(" extends {}", info.extends.join(", ")));
    }
    if !info.interfaces.is_empty() {
        header.push_str(&format!(" implements {}", info.interfaces.join(", ")));
    }
    println!("**`{}`**\n", header);

    if !info.extends.is_empty() {
        println!("**Extends:**\n");
        for e in &info.extends {
            println!("- `{}`", e);
        }
        println!();
    }

    if !info.interfaces.is_empty() {
        println!("**Implements:**\n");
        for i in &info.interfaces {
            println!("- `{}`", i);
        }
        println!();
    }

    if info.extends.is_empty() && info.interfaces.is_empty() {
        println!("(no inheritance)");
    }
}

fn print_inheritance_summary(info: &ClassInfo) {
    let mut line = format!("- `{}`", info.name);
    if !info.extends.is_empty() {
        line.push_str(&format!(" extends {}", info.extends.join(", ")));
    }
    if !info.interfaces.is_empty() {
        line.push_str(&format!(" implements {}", info.interfaces.join(", ")));
    }
    println!("{}", line);
}
