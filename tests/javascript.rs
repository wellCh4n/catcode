mod common;

fn file() -> String {
    common::fixture("javascript", "sample.js")
}

#[test]
fn lists_classes_and_methods() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Class Animal"), "got:\n{out}");
    assert!(out.contains("fn getName"), "got:\n{out}");
    assert!(out.contains("Class Dog"), "got:\n{out}");
    assert!(out.contains("fn bark"), "got:\n{out}");
}

#[test]
fn method_bark_shows_body() {
    let out = common::catcode(&[&file(), "-m", "bark"]);
    assert!(out.contains("Woof"), "got:\n{out}");
}

#[test]
fn method_of_finds_both() {
    let out = common::catcode(&[&file(), "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 2, "got:\n{out}");
}

#[test]
fn scoped_dog_of_returns_one() {
    let out = common::catcode(&[&file(), "-c", "Dog", "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 1, "got:\n{out}");
    assert!(out.contains("breed"), "got:\n{out}");
}

#[test]
fn class_lookup_shows_full_body() {
    let out = common::catcode(&[&file(), "-c", "Animal"]);
    assert!(out.contains("getName"), "got:\n{out}");
}
