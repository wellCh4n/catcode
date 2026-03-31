---
name: catcode
description: "Progressive code reading tool for token-efficient source exploration. Instead of reading entire files, first get a structural overview (classes, functions, fields with line numbers), then drill down into only the specific method or class you care about. Use this skill whenever you need to understand or navigate a source file — it reduces token consumption significantly compared to reading the full file."
---

# catcode Skill

A progressive code reading tool. Rather than loading an entire source file into context, catcode lets you explore code in layers:

1. **Overview** — get the file's full structure (classes, methods, fields, line numbers) in minimal tokens
2. **Drill down** — read only the specific method or class body you actually need

This two-step approach avoids pulling irrelevant code into context, making code reading faster and cheaper.

## Binary

The `catcode` binary is located in the `bin/` subdirectory:

```bash
CATCODE="$(dirname "$0")/bin/catcode"
```

## Commands

### List file structure

Shows all functions, classes, and member variables with line numbers.

```bash
./bin/catcode <file>
```

**Example output:**
```
File: Environment.java (Java)
────────────────────────────────────────────────────────────
  Class Environment (lines 25-138)
    field id: String (line 32)
    field name: String (line 41)
    fn generateId (lines 35-39)
    Class KubernetesApiServer (lines 56-96)
      field url: String (line 61)
      fn of (lines 69-74)
      fn isValid (lines 88-95)
```

### Find a method by name (`-m`)

Shows the full body of every method matching the name (handles overloads).

```bash
./bin/catcode <file> -m <method>
```

**Example output:**
```
Name:  of
Kind:  Function
Lines: 69-74
────────────────────────────────────────────────────────────
  69 │         public static KubernetesApiServer of(String url, String token) {
  70 │             KubernetesApiServer server = new KubernetesApiServer();
  71 │             server.setUrl(url);
  72 │             server.setToken(token);
  73 │             return server;
  74 │         }
```

### Find a class by name (`-c`)

Shows the full body of a class.

```bash
./bin/catcode <file> -c <class>
```

### Scope a method to a class (`-c` + `-m`)

When multiple classes have a method with the same name, use `-c` to narrow the search.

```bash
./bin/catcode <file> -c <class> -m <method>
```

**Example:** `Environment.java` has two `of()` methods in different inner classes:
```bash
./bin/catcode Environment.java -m of              # returns both
./bin/catcode Environment.java -c KubernetesApiServer -m of   # returns only that one
./bin/catcode Environment.java -c ImageRepository -m of       # returns only that one
```

## Supported Languages

| Language   | Extensions                                                           |
|------------|----------------------------------------------------------------------|
| Java       | `.java`                                                              |
| Rust       | `.rs`                                                                |
| Python     | `.py`                                                                |
| JavaScript | `.js` `.jsm` `.mjs` `.jsx`                                           |
| TypeScript | `.ts` `.jsw` `.jsmw`                                                 |
| TSX        | `.tsx`                                                               |
| C / C++    | `.c` `.h` `.cc` `.cpp` `.cxx` `.hh` `.hxx` `.hpp` `.inc` `.m` `.mm` |

## Recommended reading workflow

```
# Step 1: get the map (cheap)
./bin/catcode <file>

# Step 2: read only what you need (targeted)
./bin/catcode <file> -m <method>
./bin/catcode <file> -c <class>
./bin/catcode <file> -c <class> -m <method>
```

Avoid jumping straight to `Read` on a large file. Use catcode first to locate the relevant code, then read only those lines.

## When to use

- Before editing a file you haven't seen — get the structure first
- When looking for a specific method implementation in a large file
- When you only need one class or method, not the whole file
- When a method name is overloaded across multiple classes and you need the right one

## When NOT to use

- The file extension is not in the supported list above (use Read or Grep instead)
- You need to search across multiple files (use Grep instead)
- You already know exactly which lines to read (use Read with offset/limit instead)
