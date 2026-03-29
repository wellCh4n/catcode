use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/go/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_go_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("main.go")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main"));
    assert!(stdout.contains("handler"));
    assert!(stdout.contains("processRequest"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_go_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("main.go"), "-m", "handler"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("handler"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Structs ============
#[test]
fn test_go_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("main.go"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Go doesn't have classes, just structs
    assert!(stdout.contains("main.go") || stdout.contains("No classes"));
}

// ============ Feature: Imports ============
#[test]
fn test_go_imports() {
    let output = catcode()
        .args(["-f", &fixture("main.go"), "-i"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("fmt"));
    assert!(stdout.contains("context"));
}

