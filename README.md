# catcode

A CLI tool for reading code structure via AST parsing, designed for progressive code exploration with LLMs.

## Installation

```bash
uv pip install -e .
```

## Commands

```bash
catcode -f <file>               # list all methods (grouped by class)
catcode -f <file> -m <method>   # show method body, class, and outgoing calls
catcode -f <file> -c            # list all classes / structs
catcode -f <file> -c <class>    # show class skeleton (fields + methods)
```

---

## Language Support

### Java `.java`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with modifiers + annotations + return type + params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ class / interface / enum |
| Class skeleton | ✅ class-level annotations, extends, implements, fields (with annotations), methods (with annotations) |

```bash
catcode -f UserService.java
catcode -f UserService.java -m createUser
catcode -f UserService.java -c
catcode -f UserService.java -c UserService
```

Example method list:
```
**UserService**
- `@Autowired public UserService(UserRepository repo)`
- `@GetMapping("/users") public List<User> getUsers()`
- `@PostMapping public User createUser(@RequestBody UserDto dto)`
```

Example class skeleton:
```
`@Service` `@Transactional`
## class `UserService` extends `BaseService` implements `UserOps`

**Fields**
- `@Autowired private final UserRepository repo`

**Methods**
- `@Autowired public UserService(UserRepository repo)`
- `@GetMapping("/users") public List<User> getUsers()`
```

---

### Python `.py`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with decorators + params + return type annotation |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ❌ |

```bash
catcode -f views.py
catcode -f views.py -m get_users
catcode -f views.py -c
```

Example method list:
```
**UserView**
- `@login_required @permission_required("admin") def get_users(self, request: Request) -> Response`

**(top-level)**
- `def helper(x: int) -> str`
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

Example method list:
```
**Adaptor**
- `func (a *Adaptor) ConvertRequest(c *gin.Context, info *RelayInfo) (any, error)`
- `func (a *Adaptor) GetRequestURL(info *RelayInfo) (string, error)`

**(top-level)**
- `func shouldAppendQuery(info *RelayInfo) bool`
```

---

### Rust `.rs`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by impl type, with visibility + attributes + params + return type |
| Show method | ✅ body, line range, impl type, outgoing calls |
| List types | ✅ struct / impl |
| Type skeleton | ❌ |

```bash
catcode -f client.rs
catcode -f client.rs -m send_request
catcode -f client.rs -c
```

Example method list:
```
**Client**
- `#[tracing::instrument] pub fn send_request(&self, url: &str) -> Result<Response>`
- `pub fn new(config: Config) -> Self`
```

---

### TypeScript `.ts` `.tsx`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with decorators + params + return type |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ❌ |

```bash
catcode -f user.service.ts
catcode -f user.service.ts -m getUsers
catcode -f user.service.ts -c
```

Example method list:
```
**UserService**
- `@Get("/users") getUsers(query: QueryDto): Promise<User[]>`
- `@Post() createUser(@Body() dto: CreateUserDto): Promise<User>`
```

---

### JavaScript `.js` `.mjs` `.cjs`

| Feature | Support |
|---------|---------|
| List methods | ✅ grouped by class, with params |
| Show method | ✅ body, line range, class, outgoing calls |
| List classes | ✅ |
| Class skeleton | ❌ |

---

### C `.c` `.h` / C++ `.cpp` `.cc` `.cxx` `.hpp`

| Feature | Support |
|---------|---------|
| List methods | ✅ return type + name + params (no class grouping) |
| Show method | ✅ body, line range, outgoing calls |
| List classes | ❌ |
| Class skeleton | ❌ |

---

### Ruby `.rb` / Kotlin `.kt` `.kts` / Swift `.swift` / C# `.cs` / PHP `.php`

| Feature | Support |
|---------|---------|
| List methods | ✅ method name only |
| Show method | ✅ body, line range |
| List classes | ❌ |
| Class skeleton | ❌ |
