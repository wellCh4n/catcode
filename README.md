# catcode

A CLI tool to extract functions, classes, and member variables from source files, powered by [rust-code-analysis](https://github.com/mozilla/rust-code-analysis).

## Usage

```
# List all functions, classes, and fields in a file
catcode <file> [file...]

# Show the full body of a specific method
catcode <file> -m <method>

# Show the full body of a specific class
catcode <file> -c <class>

# Show a method scoped to a specific class (useful for overloaded methods)
catcode <file> -c <class> -m <method>
```

## Examples

```
$ catcode src/main.rs
File: src/main.rs (Rust)
────────────────────────────────────────────────────────────
  Impl Point (lines 6-18)
    field x: f64 (line 2)
    field y: f64 (line 3)
    fn new (lines 7-9)
    fn distance (lines 11-13)

$ catcode Sample.java -m of
Name:  of
Kind:  Function
Lines: 19-21
────────────────────────────────────────────────────────────
  19 │     public static Animal of(String name, int age) {
  ...

$ catcode Sample.java -c Dog -m of
# returns only Dog's of(), not Animal's
```

## Supported Languages

| Language   | Extensions                                                           |
|------------|----------------------------------------------------------------------|
| Rust       | `.rs`                                                                |
| Python     | `.py`                                                                |
| JavaScript | `.js` `.jsm` `.mjs` `.jsx`                                           |
| TypeScript | `.ts` `.jsw` `.jsmw`                                                 |
| TSX        | `.tsx`                                                               |
| C / C++    | `.c` `.h` `.cc` `.cpp` `.cxx` `.hh` `.hxx` `.hpp` `.inc` `.m` `.mm` |
| Java       | `.java`                                                              |

## Development

```bash
cargo build
cargo test
```
