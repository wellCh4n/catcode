use std::process::Command;

fn catcode() -> Command {
    Command::new(env!("CARGO_BIN_EXE_catcode"))
}

fn fixture(path: &str) -> String {
    format!("tests/csharp/{}", path)
}

// ============ Feature: List Methods (default) ============
#[test]
fn test_csharp_list_methods() {
    let output = catcode()
        .args(["-f", &fixture("Service.cs")])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CreateUser"));
    assert!(stdout.contains("GetUser"));
    assert!(stdout.contains("DeleteUser"));
}

// ============ Feature: Method Detail ============
#[test]
fn test_csharp_method_detail() {
    let output = catcode()
        .args(["-f", &fixture("Service.cs"), "-m", "CreateUser"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("CreateUser"));
    assert!(stdout.contains("L"));
}

// ============ Feature: List Classes ============
#[test]
fn test_csharp_list_classes() {
    let output = catcode()
        .args(["-f", &fixture("Service.cs"), "-c"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("AdminService"));
}

// ============ Feature: Class Skeleton ============
#[test]
fn test_csharp_class_skeleton() {
    let output = catcode()
        .args(["-f", &fixture("Service.cs"), "-c", "UserService"])
        .output()
        .expect("failed to execute catcode");
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("UserService"));
    assert!(stdout.contains("CreateUser"));
}

