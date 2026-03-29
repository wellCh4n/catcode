use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/swift/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_swift_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("Service.swift")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("getUser"));
    assert!(stdout.contains("deleteUser"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_swift_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("Service.swift"), "-m", "createUser"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("createUser"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Classes ============
#[test]
fn test_swift_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("Service.swift"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("User"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_swift_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("Service.swift"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("createUser"));
}

