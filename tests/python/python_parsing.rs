use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/python/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_python_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("classes.py")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("get_user"));
    assert!(stdout.contains("delete_user"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_python_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("classes.py"), "-m", "create_user"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Classes ============
#[test]
fn test_python_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("classes.py"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("OrderService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_python_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("classes.py"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("create_user"));
    assert!(stdout.contains("get_user"));
}

// ============ Feature: Inheritance ============
#[test]
fn test_python_inheritance() {
    let output = catcode()
        .args(["-f", &fixture("inheritance.py"), "-I", "AdminUser"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("AdminUser"));
    assert!(stdout.contains("extends"));
    assert!(stdout.contains("User"));
}

// ============ Feature: Imports ============
#[test]
fn test_python_imports() {
    let output = catcode()
        .args(["-f", &fixture("imports.py"), "-i"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("os"));
    assert!(stdout.contains("sys"));
    assert!(stdout.contains("typing"));
}

