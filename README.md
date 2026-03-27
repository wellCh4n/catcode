# catcode

CatCode: Progressive Code Reading Skill for LLMs

![logo.png](images/logo.png)

## Installation

```bash
cargo build --release
# binary at: target/release/catcode
```

## Commands

### Single file

```bash
catcode -f <file>               # list all methods (grouped by class)
catcode -f <file> -m <method>   # show method body, class, and outgoing calls
catcode -f <file> -c            # list all classes / structs
catcode -f <file> -c <class>    # show class skeleton (fields + methods)
catcode -f <file> -r <method>   # find callers of method in this file
```

### Directory (recursive scan)

```bash
catcode -d <dir>                # list all methods across all supported files
catcode -d <dir> -m <method>    # find and show method across all files
catcode -d <dir> -c             # list all classes across all files
catcode -d <dir> -c <class>     # find and show class skeleton across all files
catcode -d <dir> -r <method>    # find all callers of method across all files
```

Results are grouped by file path. Files that fail to parse are silently skipped.

---

## Language Support

### Java `.java`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with modifiers + annotations + return type + params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ class / interface / enum |
| Class skeleton | ✅ class-level annotations, extends, implements, fields, methods |

```bash
catcode -f UserService.java
catcode -f UserService.java -m createUser
catcode -f UserService.java -c
catcode -f UserService.java -c UserService
```

---

### Python `.py`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with decorators + params + return type annotation |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ✅ fields + methods |

```bash
catcode -f views.py
catcode -f views.py -m get_users
catcode -f views.py -c
catcode -f views.py -c UserView
```

---

### Go `.go`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by receiver type, with full signature (receiver + params + return type) |
| Show method | ✅ body, line range, receiver type, outgoing calls |
| List structs | ✅ |
| Struct skeleton | ✅ fields + all methods on the type |

```bash
catcode -f adaptor.go
catcode -f adaptor.go -m ConvertRequest
catcode -f adaptor.go -c
catcode -f adaptor.go -c Adaptor
```

---

### Rust `.rs`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by impl type, with visibility + attributes + params + return type |
| Show method | ✅ body, line range, impl type, outgoing calls |
| List types | ✅ struct / impl |
| Type skeleton | ✅ fields + methods |

```bash
catcode -f client.rs
catcode -f client.rs -m send_request
catcode -f client.rs -c
catcode -f client.rs -c Client
```

---

### TypeScript `.ts` `.tsx`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with decorators + params + return type |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ✅ fields + methods |

```bash
catcode -f user.service.ts
catcode -f user.service.ts -m getUsers
catcode -f user.service.ts -c
catcode -f user.service.ts -c UserService
```

---

### JavaScript `.js` `.mjs` `.cjs`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ✅ methods |

---

### C `.c` `.h` / C++ `.cpp` `.cc` `.cxx` `.hpp`

| Feature | Support |
|---------|---------|
| List methods | ✅ return type + name + params |
| Show method | ✅ body, line range |
| List classes | ❌ |
| Class skeleton | ❌ |

---

### Ruby `.rb` / Kotlin `.kt` `.kts` / Swift `.swift` / C# `.cs` / PHP `.php`

| Feature | Support |
|---------|---------|
| List methods | ✅ |
| Show method | ✅ body, line range |
| List classes | ✅ |
| Class skeleton | ✅ methods |
