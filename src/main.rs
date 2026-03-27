mod ext_map;
mod parser;
mod scanner;
mod types;

use std::path::Path;

use anyhow::Result;
use clap::Parser as ClapParser;

use ext_map::ext_to_lang;
use parser::make_parser;
use scanner::{scan_callers, scan_class, scan_list_classes, scan_list_methods, scan_method};
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

    /// Method name to look up
    #[arg(short = 'm', long)]
    method: Option<String>,

    /// Class name (omit value to list all)
    #[arg(short = 'c', long, num_args = 0..=1, default_missing_value = "__list__")]
    class: Option<String>,

    /// Find callers of the given method name
    #[arg(short = 'r', long)]
    reverse: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(file) = &cli.file {
        handle_file(file, &cli)?;
    } else if let Some(dir) = &cli.dir {
        handle_dir(dir, &cli)?;
    } else {
        eprintln!("Error: provide -f FILE or -d DIR");
        std::process::exit(1);
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

    if let Some(reverse) = &cli.reverse {
        let callers = parser.find_callers(reverse);
        if callers.is_empty() {
            println!("No callers of `{}` found in {}", reverse, file);
            return Ok(());
        }
        println!("## Callers of `{}` in {}\n", reverse, file);
        for c in &callers {
            let method = c.caller_method.as_deref().unwrap_or("<top-level>");
            let class = c.caller_class.as_deref();
            if let Some(cls) = class {
                println!("- `{}::{}` > L{}-{}", cls, method, c.start_line, c.end_line);
            } else {
                println!("- `{}` > L{}-{}", method, c.start_line, c.end_line);
            }
        }
        return Ok(());
    }

    if let Some(name) = &cli.method {
        match parser.get_method(name) {
            Some(info) => print_method_detail(file, name, &info),
            None => println!("method '{}' not found in {}", name, file),
        }
    }

    match &cli.class {
        Some(name) if name == "__list__" => {
            let classes = parser.list_classes();
            if classes.is_empty() {
                println!("No classes found in {}", file);
            } else {
                println!("## Classes in {}\n", file);
                for cls in &classes {
                    println!("- `{}`", cls);
                }
            }
        }
        Some(name) => {
            match parser.get_class_skeleton(name) {
                Some(info) => print_class_skeleton(file, &info),
                None => println!("class '{}' not found in {}", name, file),
            }
        }
        None => {}
    }

    // Default: list all methods when no flag given
    if cli.method.is_none() && cli.class.is_none() && cli.reverse.is_none() {
        print_method_list(file, &parser.list_methods());
    }

    Ok(())
}

fn handle_dir(dir: &str, cli: &Cli) -> Result<()> {
    let path = Path::new(dir);

    if let Some(reverse) = &cli.reverse {
        let callers = scan_callers(path, reverse);
        if callers.is_empty() {
            println!("No callers of `{}` found in {}", reverse, dir);
            return Ok(());
        }
        println!("## Callers of `{}`\n", reverse);
        for c in &callers {
            let method = c.caller_method.as_deref().unwrap_or("<top-level>");
            let class = c.caller_class.as_deref();
            if let Some(cls) = class {
                println!(
                    "- `{}` — `{}::{}` > L{}-{}",
                    c.file, cls, method, c.start_line, c.end_line
                );
            } else {
                println!("- `{}` — `{}` > L{}-{}", c.file, method, c.start_line, c.end_line);
            }
        }
        return Ok(());
    }

    if let Some(name) = &cli.method {
        let results = scan_method(path, name);
        if results.is_empty() {
            println!("method '{}' not found in {}", name, dir);
        } else {
            for (file, info) in &results {
                print_method_detail(&file.to_string_lossy(), name, info);
            }
        }
    }

    match &cli.class {
        Some(name) if name == "__list__" => {
            let results = scan_list_classes(path);
            for (file, classes) in &results {
                println!("## Classes in {}\n", file.display());
                for cls in classes {
                    println!("- `{}`", cls);
                }
                println!();
            }
        }
        Some(name) => {
            let results = scan_class(path, name);
            if results.is_empty() {
                println!("class '{}' not found in {}", name, dir);
            } else {
                for (file, info) in &results {
                    print_class_skeleton(&file.to_string_lossy(), &info);
                }
            }
        }
        None => {}
    }

    // Default: list all methods when no flag given
    if cli.method.is_none() && cli.class.is_none() && cli.reverse.is_none() {
        let results = scan_list_methods(path);
        for (file, methods) in &results {
            print_method_list(&file.to_string_lossy(), methods);
        }
    }

    Ok(())
}

fn print_method_list(file: &str, methods: &[(Option<String>, String)]) {
    if methods.is_empty() {
        return;
    }
    println!("## {}\n", file);
    // Group by class
    let mut current_class: Option<&str> = None;
    for (class, sig) in methods {
        let cls = class.as_deref();
        if cls != current_class {
            if let Some(c) = cls {
                println!("### `{}`\n", c);
            }
            current_class = cls;
        }
        println!("- `{}`", sig);
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
        "## {}\n\n{}**`{}`**\n\n> L{}-{}\n\n```\n{}\n```",
        file,
        class_prefix,
        info.signature,
        info.start_line,
        info.end_line,
        info.body
    );

    if !info.calls.is_empty() {
        println!("\n**Calls:** {}", info.calls.join(", "));
    }
    println!();
}

fn print_class_skeleton(file: &str, info: &ClassInfo) {
    println!("## {} — `{}`\n", file, info.name);

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
