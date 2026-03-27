use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use crate::ext_map::ext_to_lang;
use crate::parser::make_parser;
use crate::types::{CallerInfo, ClassInfo, MethodInfo};

/// Yield (path, lang) for all supported files under `directory`.
pub fn iter_source_files(directory: &Path) -> impl Iterator<Item = (PathBuf, &'static str)> {
    WalkDir::new(directory)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            let path = e.into_path();
            let ext = path.extension()?.to_str()?;
            let lang = ext_to_lang(ext)?;
            Some((path, lang))
        })
}

pub fn scan_method(directory: &Path, method_name: &str) -> Vec<(PathBuf, MethodInfo)> {
    let mut results = vec![];
    for (path, lang) in iter_source_files(directory) {
        let Ok(source) = std::fs::read(&path) else { continue };
        let Ok(parser) = make_parser(lang, source) else { continue };
        if let Some(info) = parser.get_method(method_name) {
            results.push((path, info));
        }
    }
    results
}

pub fn scan_class(directory: &Path, class_name: &str) -> Vec<(PathBuf, ClassInfo)> {
    let mut results = vec![];
    for (path, lang) in iter_source_files(directory) {
        let Ok(source) = std::fs::read(&path) else { continue };
        let Ok(parser) = make_parser(lang, source) else { continue };
        if let Some(info) = parser.get_class_skeleton(class_name) {
            results.push((path, info));
        }
    }
    results
}

pub fn scan_list_methods(directory: &Path) -> Vec<(PathBuf, Vec<(Option<String>, String)>)> {
    let mut results = vec![];
    for (path, lang) in iter_source_files(directory) {
        let Ok(source) = std::fs::read(&path) else { continue };
        let Ok(parser) = make_parser(lang, source) else { continue };
        let methods = parser.list_methods();
        if !methods.is_empty() {
            results.push((path, methods));
        }
    }
    results
}

pub fn scan_list_classes(directory: &Path) -> Vec<(PathBuf, Vec<String>)> {
    let mut results = vec![];
    for (path, lang) in iter_source_files(directory) {
        let Ok(source) = std::fs::read(&path) else { continue };
        let Ok(parser) = make_parser(lang, source) else { continue };
        let classes = parser.list_classes();
        if !classes.is_empty() {
            results.push((path, classes));
        }
    }
    results
}

pub fn scan_callers(directory: &Path, method_name: &str) -> Vec<CallerInfo> {
    let mut results = vec![];
    for (path, lang) in iter_source_files(directory) {
        let Ok(source) = std::fs::read(&path) else { continue };
        let Ok(parser) = make_parser(lang, source) else { continue };
        let mut callers = parser.find_callers(method_name);
        let file_str = path.to_string_lossy().to_string();
        for c in &mut callers {
            c.file = file_str.clone();
        }
        results.extend(callers);
    }
    results
}
