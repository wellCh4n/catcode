use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/java/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_java_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("Service.java")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("getUser"));
    assert!(stdout.contains("deleteUser"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_java_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("Service.java"), "-m", "createUser"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("L")); // line number
    assert!(stdout.contains("public"));
}

// ============ Feature: List Classes ============
#[test]
fn test_java_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("Service.java"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("AdminService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_java_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("Service.java"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("getUser"));
}

// ============ Feature: Inheritance ============
#[test]
fn test_java_inheritance() {
    let output = catcode()
        .args(["-f", &fixture("Service.java"), "-I", "AdminService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AdminService"));
    assert!(stdout.contains("extends"));
    assert!(stdout.contains("UserService"));
}

// ============ Feature: Imports ============
#[test]
fn test_java_imports() {
    let output = catcode()
        .args(["-f", &fixture("Service.java"), "-i"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("java.util"));
}

