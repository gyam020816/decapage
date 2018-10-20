## notes to self

- use `cargo new myprojectname` to create a subdirectory called `myprojectname`.
- emoji:
  - 🌟 *"Huh, interesting feature"*
  - 🎵 *"Different to some other language"*
  - 🤔 *"Not sure about my own wording"*

### ch02

https://doc.rust-lang.org/book/2018-edition/ch02-00-guessing-game-tutorial.html

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
- 🌟 Variable names can be shadowed on the same 🤔 block.

### ch03

https://doc.rust-lang.org/book/2018-edition/ch03-02-data-types.html

- Integer overflows causes a panic (crash) during development, but NOT during release builds.
  - Overflows are considered errors, not desired behavior.
- Arrays have a fixed size.
  - Vectors are more flexible.
  - The type of an array is `[type; number]`
  - Invalid array access causes a panic (🎵 not undefined behavior).


https://doc.rust-lang.org/book/2018-edition/ch03-03-how-functions-work.html

- Statements do not return a value, and end with `;`.
- Expression return a value, 🎵 and do *not* end with `;`. 
  - 🎵 A block of code made of `{}` is an expression
    when the last line within it is an expression (do *not* end with `;`).
```
let y = {
    let x = 3;
    x + 1
};
// y is equal to 4
```
- Functions can return early with `return` but 🎵 they return the last expression.

https://doc.rust-lang.org/book/2018-edition/ch03-05-control-flow.html

- The condition in an `if` statement must be of type `bool`.
- `if` is an expression, so it can be assigned with `let`,
  and all ending expressions in the arms must be of the same type. 
- `loop` is an expression, and the returned value is at the right of `break`.
- `for` only has the improved iterable loop style

## ch04

https://doc.rust-lang.org/book/2018-edition/ch04-01-what-is-ownership.html

- 🌟 **ownership** is a memory management strategy at compile time
- 🌟 ownership features doe not slow down program when running

- stack and heap back to basics
  - stack is fast because the data is always at the top
  - data on the stack have a known and fixed size
  - heap is slower because we need to follow a pointer
  - heap is slower because related data may be scattered all over the place
  - function arguments and local variables are pushed on the stack,
    and popped when it is over

- 🌟 ownership rules:
  - for each value, there is a variable that is the owner of that value
  - there can only be one owner
  - when the owner is out of *scope* the value is dropped

- The type `String` is allocated on the heap
- With `mut` we can mutate strings
- string literals are fast because they are known at compile time

- 🌟 memory is freed when the owner variable goes out of scope
- the function that frees memory is called `drop` and Rust calls it as soon as
  an owning variable goes out of scope
  - this is similar to *RAII* in C++

- in the following, the data on the heap is not duplicated,
  and there are NO distinct pointers; this is a move operation:
```
let s1 = String::from("hello");
let s2 = s1;
```
- 🌟 given a move operation, the variable it was moved from is no longer valid
  and cannot be used
- this is not a shallow copy since the variable it was moved from is no longer valid

- 🌟 deep copies are never done by the language automatically
- some types are special like integers,
  they are stored on the stack (may be stored? 🤔) and have a `Copy` trait
- if a type has a `Copy` trait then they can't have a `Drop` trait

- 🌟 passing a variable to a function is a move, the variable is no longer valid

https://doc.rust-lang.org/book/2018-edition/ch04-02-references-and-borrowing.html

- `&myvariablename` creates a reference that refers to the value of the *myvariablename*
  but does not own it
- when a reference goes out of scope, the value is not dropped
- for functions accepting references, the type must be like `&String`
- functions accepting mutable references are `&mut String`
  - 🌟 there can only be one mutable reference to a value in a given scope
    - this is to prevent a data race in concurrency situations, at compile time
- the following is valid:
```
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```
- 🌟 we cannot hold a mutable reference at the same time as an immutable reference
  in a given scope
- *returning a reference to a local variable will be explained in a later chapter*

https://doc.rust-lang.org/book/2018-edition/ch04-03-slices.html

- slice is a reference to part of a string (in `let s = "hello world"`,
  the slice `&s[0..5]` and `&s[0..=4]` refer to `"hello"`)
- it's possible to slice the entire string or String with `&s[..]`
- string in code are just slices of the executable binary itself
- functions taking or returning `&str` type are preferred/idiomatic than using `String`
- slicing a non-string `i32` array bears the type `&[i32]`

## ch05

https://doc.rust-lang.org/book/2018-edition/ch05-01-defining-structs.html

- `struct` are data holders that have *fields*
- a field ends with a comma
- by convention the name of the struct start with an uppercase
- instances are created with brackets and look just like the struct
- fields are accessed with the perios `.` like `my_instance.my_field_name`:
```
let user1 = User {
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};
let mut user1 = User {
    email: String::from("someone@example.com"),
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```
- if the instance is mutable, all the fields are mutable
- if a variable in the scope has the same name as a struct field,
  then we can skip explicitly naming the field name:
```
fn build_user(email: String) -> User {
    User {
        email,
        sign_in_count: 1,
    }
}
```
- to create a copy of another struct instance,
  use `..my_other_instance` as the last line of the struct instanciation without a comma
- tuple structs have no field names, but are typed independently
```
struct Color(i32, i32, i32);
let black = Color(0, 0, 0);
```
- in structs, strings have the `String` type because the instance of
  the struct is the owner of its fields
  - *structs with reference fields will be explored later*

https://doc.rust-lang.org/book/2018-edition/ch05-02-example-structs.html

- the values inside a tuple can be accessed with `my_tuple.0` and so on
- use `#[derive(Debug)]` before the struct definition to print it with the `{:?}` capture
  - use `{:#?}` for multiline capture

https://doc.rust-lang.org/book/2018-edition/ch05-03-method-syntax.html

- with the `impl` block we can attach methods to a struct, enum, or trait
  - the first parameter is always `&self`
  - it is possible but rare to use `self` to instead to take ownership
- if a method doesn't take `self` then it's not a method, it's an *associated function*
  - think static method / companion object function
- it's possible to have multiple impl blocks

## ch06

https://doc.rust-lang.org/book/2018-edition/ch06-00-enums.html

- enums are closer to algebraic data types, see F# or Haskell

https://doc.rust-lang.org/book/2018-edition/ch06-01-defining-an-enum.html

- turns out `enum`s are almost like Kotlin's `sealed class` with inheriting suclasses
