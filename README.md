# catcode

A CLI tool to extract functions and classes from source files, powered by [rust-code-analysis](https://github.com/mozilla/rust-code-analysis).

## Usage

```
# List all functions and classes in a file
catcode <file> [file...]

# Show the full body of a specific method
catcode <file> -m <method_name>
```

## Supported Languages

| Language   | Extensions                                      |
|------------|-------------------------------------------------|
| Rust       | `.rs`                                           |
| Python     | `.py`                                           |
| JavaScript | `.js` `.jsm` `.mjs` `.jsx`                      |
| TypeScript | `.ts` `.jsw` `.jsmw`                            |
| TSX        | `.tsx`                                          |
| C / C++    | `.c` `.h` `.cc` `.cpp` `.cxx` `.hh` `.hxx` `.hpp` `.inc` `.m` `.mm` |
| Java       | `.java`                                         |
