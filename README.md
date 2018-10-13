## notes to self

- in `String::new()`, `::` means that the function `new` is implemented on the *type* `String`,
  rather than an instance.
- `mut` lets it be mutable.
- Passing an argument with `&myvariable` is by reference. It may be `&mut myvariable`.
- `Result` is an enum type with variants `Ok` and `Err`.
  - When calling the method `expect` of a `Result`, if that `Result` was an `Err`:
    - then it will crash the program purposefully,
    - otherwise it will return some 🤔 value.
  - Somehow the compiler knows and warns that we must call `expect` on a `Result`.
- `cargo update` finds update availabilities on the PATCH in semantic versioning,
  and rewrite `Cargo.lock` when successful.
- `match` given a value, checks each arm's pattern against it and runs the first matching expression.
- 🌟 Variable names can be shadowed on the same block 🤔.
