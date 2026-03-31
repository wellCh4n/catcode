mod common;

fn file() -> String {
    common::fixture("java", "Sample.java")
}

#[test]
fn lists_class_animal() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Class Animal"), "got:\n{out}");
}

#[test]
fn lists_fields_name_and_age() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("field name"), "got:\n{out}");
    assert!(out.contains("field age"), "got:\n{out}");
}

#[test]
fn lists_fn_get_name() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("fn getName"), "got:\n{out}");
}

#[test]
fn lists_nested_class_dog_with_field() {
    let out = common::catcode(&[&file()]);
    assert!(out.contains("Class Dog"), "got:\n{out}");
    assert!(out.contains("field breed"), "got:\n{out}");
}

#[test]
fn method_lookup_shows_body() {
    let out = common::catcode(&[&file(), "-m", "getName"]);
    assert!(out.contains("return name"), "got:\n{out}");
}

#[test]
fn method_of_finds_both_overloads() {
    let out = common::catcode(&[&file(), "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 2, "got:\n{out}");
}

#[test]
fn scoped_animal_of_returns_one() {
    let out = common::catcode(&[&file(), "-c", "Animal", "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 1, "got:\n{out}");
    assert!(out.contains("Animal of"), "got:\n{out}");
}

#[test]
fn scoped_dog_of_returns_dog_version() {
    let out = common::catcode(&[&file(), "-c", "Dog", "-m", "of"]);
    assert_eq!(common::count_occurrences(&out, "Name:"), 1, "got:\n{out}");
    assert!(out.contains("String breed"), "got:\n{out}");
}

#[test]
fn class_lookup_shows_full_body() {
    let out = common::catcode(&[&file(), "-c", "Dog"]);
    assert!(out.contains("getBreed"), "got:\n{out}");
}
