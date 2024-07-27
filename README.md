# `synext`

A simple extension library for `syn` crate to help developers quickly develop derive macros

## 1. `Acknowledgment`

This project, `synext`, was developed with significant inspiration from the open-source
project [proc_macros](https://github.com/DzenanJupic/proc_macros). Special
thanks to the contributors of [proc_macros](https://github.com/DzenanJupic/proc_macros) for their excellent work.

## 2. `Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
synext = "0.1"
```

## 3. `APIs`

### 3.1.`Fields`

#### 3.1.1. `named`

```rust
// input = TokenStream
let derive_input = try_derive_input(input);
let named_fields = try_parse_named_fields( & derive_input);
```

#### 3.1.2. `unnamed`

```rust
// input = TokenStream
let derive_input = try_derive_input(input);
let unnamed_fields = try_parse_unnamed_fields( & derive_input);
```

#### 3.1.3. `match`

```rust
// input = TokenStream
let derive_input = try_derive_input(input);
let fields = try_match_fields( & derive_input);
```

### 3.2. `Types`

#### 3.2.1. `Option`

unwrap `Option` inner type.

```rust
pub fn try_unwrap_option(ty: &Type) -> &Type { ... }
```

#### 3.2.2. `Vec`

unwrap `Vec` inner type.

```rust
pub fn try_unwrap_vec(ty: &Type) -> &Type { ... }
```

#### 3.2.3. `unwrap_types`

```rust
pub fn try_unwrap_types<'a>(ident: &str, target_types: usize, ty: &'a Type) -> Option<Vec<&'a Type>> { ... }
```

#### 3.2.4. `inner_types`

```rust
pub fn try_extract_inner_types(ty: &Type) -> Option<Vec<&Type>> { ... }
```

### 3.3. `Predicate`

- `Option`

    - ```rust
      pub fn try_predicate_is_option(ty: &Type) -> bool { ... }
      ```

- `Vec`

    - ```rust
      pub fn try_predicate_is_vec(ty: &Type) -> bool { ... }
      ```

- `Ident`

    - ```rust
      pub fn try_predicate_is_not_ident(ident: &str, path: &Path) -> bool { ... }
      pub fn try_predicate_is_ident(ident: &str, path: &Path) -> bool { ... }
      ```

- `segments`

    - ```rust
      pub fn try_predicate_path_segments_is_not_empty(path: &Path) -> bool { ... }
      pub fn try_predicate_path_segments_is_empty(path: &Path) -> bool { ... }
      ```
