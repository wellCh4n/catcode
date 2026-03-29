# catcode

CatCode: Progressive Code Reading Skill for LLMs

![logo.png](images/logo.png)

## Installation

```bash
cargo build --release
# binary at: target/release/catcode
```

## Features

| Flag | Description |
|------|-------------|
| `-f <file>` | Source file to analyze |
| `-m <method>` | Show method detail (body, line range, calls) |
| `-c` | List all classes/structs/types |
| `-c <name>` | Show class skeleton (fields + methods) |
| `-i` | List imports/dependencies |
| `-I [<class>]` | Show inheritance hierarchy (all classes or specific one) |
| `-d <dir>` | Scan directory for all files |
| `-x <n>` | Max files to process in directory mode |

### Directory Scanning

```bash
catcode -d ./src                    # scan directory, detect languages
catcode -d ./src -x 10              # limit to 10 files
```

---

## Language Support

### Java `.java`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with modifiers + annotations + return type + params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ class / interface / enum |
| Class skeleton | ✅ class-level annotations, extends, implements, fields, methods |
| Imports | ✅ |
| Inheritance | ✅ |

```bash
catcode -f UserService.java
catcode -f UserService.java -m createUser
catcode -f UserService.java -c
catcode -f UserService.java -c UserService
catcode -f UserService.java -i
catcode -f UserService.java -I          # show all inheritance
catcode -f UserService.java -I AdminService  # show specific class
```

---

### Python `.py`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with decorators + params + return type annotation |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ✅ fields + methods |
| Imports | ✅ |
| Inheritance | ✅ |

```bash
catcode -f views.py
catcode -f views.py -m get_users
catcode -f views.py -c
catcode -f views.py -c UserView
catcode -f views.py -i
catcode -f views.py -I              # show all inheritance
catcode -f views.py -I UserView     # show specific class
```

---

### Go `.go`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by receiver type, with full signature (receiver + params + return type) |
| Show method | ✅ body, line range, receiver type, outgoing calls |
| List structs | ✅ |
| Struct skeleton | ✅ fields + all methods on the type |
| Imports | ✅ |

```bash
catcode -f adaptor.go
catcode -f adaptor.go -m ConvertRequest
catcode -f adaptor.go -c
catcode -f adaptor.go -c Adaptor
catcode -f adaptor.go -i
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
| Imports | ✅ |
| Inheritance | ✅ |

```bash
catcode -f user.service.ts
catcode -f user.service.ts -m getUsers
catcode -f user.service.ts -c
catcode -f user.service.ts -c UserService
catcode -f user.service.ts -i
catcode -f user.service.ts -I              # show all inheritance
catcode -f user.service.ts -I UserService  # show specific class
```

---

### JavaScript `.js` `.mjs` `.cjs`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ✅ methods |

```bash
catcode -f app.js
catcode -f app.js -m getData
catcode -f app.js -c
catcode -f app.js -c App
```

---

### Ruby `.rb`

| Feature | Support |
|---------|---------|
| List methods | ✅ with params |
| Show method | ✅ body, line range, class |
| List classes | ✅ |
| Class skeleton | ✅ methods |

```bash
catcode -f user.rb
catcode -f user.rb -m find_by_id
catcode -f user.rb -c
catcode -f user.rb -c User
```

---

### Kotlin `.kt` `.kts`

| Feature | Support |
|---------|---------|
| List methods | ✅ with params + return type |
| Show method | ✅ body, line range, class |
| List classes | ✅ |
| Class skeleton | ✅ fields + methods |

```bash
catcode -f UserService.kt
catcode -f UserService.kt -m createUser
catcode -f UserService.kt -c
catcode -f UserService.kt -c UserService
```

---

### Swift `.swift`

| Feature | Support |
|---------|---------|
| List methods | ✅ with params + return type |
| Show method | ✅ body, line range |
| List classes | ✅ class / struct |
| Class skeleton | ✅ fields + methods |

```bash
catcode -f UserService.swift
catcode -f UserService.swift -m createUser
catcode -f UserService.swift -c
catcode -f UserService.swift -c UserService
```

---

### C# `.cs`

| Feature | Support |
|---------|---------|
| List methods | ✅ with params + return type |
| Show method | ✅ body, line range, class |
| List classes | ✅ |
| Class skeleton | ✅ fields + methods |

```bash
catcode -f UserService.cs
catcode -f UserService.cs -m CreateUser
catcode -f UserService.cs -c
catcode -f UserService.cs -c UserService
```

---

### PHP `.php`

| Feature | Support |
|---------|---------|
| List methods | ✅ with params |
| Show method | ✅ body, line range, class |
| List classes | ✅ |
| Class skeleton | ✅ methods |

```bash
catcode -f user.php
catcode -f user.php -m createUser
catcode -f user.php -c
catcode -f user.php -c User
```

---

### C `.c` `.h`

| Feature | Support |
|---------|---------|
| List methods | ✅ return type + name + params |
| Show method | ✅ body, line range |

```bash
catcode -f main.c
catcode -f main.c -m main
```

---

### C++ `.cpp` `.cc` `.cxx` `.hpp`

| Feature | Support |
|---------|---------|
| List methods | ✅ return type + name + params |
| Show method | ✅ body, line range |

```bash
catcode -f main.cpp
catcode -f main.cpp -m main
```

---

## Feature Summary by Language

| Language | Methods | Classes | Skeleton | Imports | Inheritance |
|----------|---------|---------|---------|---------|-------------|
| Java | ✅ | ✅ | ✅ | ✅ | ✅ |
| Python | ✅ | ✅ | ✅ | ✅ | ✅ |
| Go | ✅ | ✅ | ✅ | ✅ | ❌ |
| Rust | ✅ | ✅ | ✅ | ❌ | ❌ |
| TypeScript | ✅ | ✅ | ✅ | ✅ | ✅ |
| JavaScript | ✅ | ✅ | ✅ | ❌ | ❌ |
| Ruby | ✅ | ✅ | ✅ | ❌ | ❌ |
| Kotlin | ✅ | ✅ | ✅ | ❌ | ❌ |
| Swift | ✅ | ✅ | ✅ | ❌ | ❌ |
| C# | ✅ | ✅ | ✅ | ❌ | ❌ |
| PHP | ✅ | ✅ | ✅ | ❌ | ❌ |
| C | ✅ | ❌ | ❌ | ❌ | ❌ |
| C++ | ✅ | ❌ | ❌ | ❌ | ❌ |
