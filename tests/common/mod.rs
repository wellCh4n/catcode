use std::path::PathBuf;
use std::process::Command;

pub fn catcode(args: &[&str]) -> String {
    let bin = PathBuf::from(env!("CARGO_BIN_EXE_catcode"));
    let out = Command::new(bin)
        .args(args)
        .output()
        .expect("failed to run catcode");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

pub fn fixture(lang: &str, file: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join(lang)
        .join(file)
        .to_string_lossy()
        .into_owned()
}

pub fn count_occurrences(haystack: &str, needle: &str) -> usize {
    haystack.matches(needle).count()
}
