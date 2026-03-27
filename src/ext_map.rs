pub fn ext_to_lang(ext: &str) -> Option<&'static str> {
    match ext {
        "java" => Some("java"),
        "py" => Some("python"),
        "js" | "mjs" | "cjs" => Some("javascript"),
        "ts" => Some("typescript"),
        "tsx" => Some("tsx"),
        "go" => Some("go"),
        "rs" => Some("rust"),
        "c" | "h" => Some("c"),
        "cpp" | "cc" | "cxx" | "hpp" => Some("cpp"),
        "rb" => Some("ruby"),
        "kt" | "kts" => Some("kotlin"),
        "swift" => Some("swift"),
        "cs" => Some("csharp"),
        "php" => Some("php"),
        _ => None,
    }
}
