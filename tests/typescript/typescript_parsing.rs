use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/typescript/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_typescript_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("service.ts")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("getUser"));
    assert!(stdout.contains("deleteUser"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_typescript_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("service.ts"), "-m", "createUser"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Classes ============
#[test]
fn test_typescript_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("service.ts"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_typescript_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("service.ts"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("createUser"));
}

// ============ Feature: Imports ============
#[test]
fn test_typescript_imports() {
    let output = catcode()
        .args(["-f", &fixture("imports.ts"), "-i"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("import"));
}

