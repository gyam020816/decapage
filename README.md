## notes to self

- in `String::new()`, `::` means that the function `new` is implemented on the *type* `String`,
  rather than an instance.
- `mut` lets it be mutable.
- Passing an argument with `&myvariable` is by reference. It may be `&mut myvariable`.
- `Result` is an enum type with variants `Ok` and `Err`.
  - When calling the method `expect` of a `Result`, if that `Result` was an `Err`:
    - then it will crash the program purposefully,
    - otherwise it will return some 🤔 value.
