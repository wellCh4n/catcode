pub mod base;
pub mod c;
pub mod csharp;
pub mod go;
pub mod java;
pub mod javascript;
pub mod kotlin;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust_lang;
pub mod swift;
pub mod typescript;

pub use base::LanguageParser;

use anyhow::{anyhow, Result};

/// Create a boxed `LanguageParser` for the given language name and source bytes.
pub fn make_parser(lang: &str, source: Vec<u8>) -> Result<Box<dyn LanguageParser>> {
    match lang {
        "java" => Ok(Box::new(java::JavaParser::new(source)?)),
        "python" => Ok(Box::new(python::PythonParser::new(source)?)),
        "javascript" => Ok(Box::new(javascript::JavaScriptParser::new(source)?)),
        "typescript" => Ok(Box::new(typescript::TypeScriptParser::new(source)?)),
        "tsx" => Ok(Box::new(typescript::TsxParser::new(source)?)),
        "go" => Ok(Box::new(go::GoParser::new(source)?)),
        "rust" => Ok(Box::new(rust_lang::RustLangParser::new(source)?)),
        "c" => Ok(Box::new(c::CParser::new(source)?)),
        "cpp" => Ok(Box::new(c::CppParser::new(source)?)),
        "ruby" => Ok(Box::new(ruby::RubyParser::new(source)?)),
        "kotlin" => Ok(Box::new(kotlin::KotlinParser::new(source)?)),
        "swift" => Ok(Box::new(swift::SwiftParser::new(source)?)),
        "csharp" => Ok(Box::new(csharp::CSharpParser::new(source)?)),
        "php" => Ok(Box::new(php::PhpParser::new(source)?)),
        _ => Err(anyhow!("unsupported language: {}", lang)),
    }
}
