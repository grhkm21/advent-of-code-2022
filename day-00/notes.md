# Rusty Notes

Here is a note-taking document for myself to reference to. Won't be complete or anything at all.

## Day 1

### Projects

Create new Rust project by `cargo new <project-name>`. This will create directory `project-name`, which is the project root directory. Within is a file `./Cargo.toml`. Here is where you put the dependencies. There is also a `./src` directory, where source code goes in. There exists a `main.rs` already.

To build the project, navigate to the project root and run `cargo run`.

### Rust code

Import items using `use ...`, such as `use std::fs` and `use std::env`.

We can read from file using `fs::read_to_string(file_path) -> std::io::Result<String>` ([docs](https://doc.rust-lang.org/std/fs/index.html)), which in turns also contains the potential error in case it fails (i.e. it doesn't throw RuntimeError). We can do something like this to handle errors:

```rust
let contents = fs::read_to_string(file_path).expect("err: err_msg");
```

For string processing, refer to `std::string::String` [docs](https://doc.rust-lang.org/std/string/struct.String.html). In particular, `s.split(...)` returns an iterator which we can directly iterate over. Also, we can convert string into integers by `s.parse::<i64>()` (handling error with `.expect`) or other types instead of `i64`.

Rust also supports a `match` syntax for handling error. For example,

```rust
let n = "10";
match n.parse::<i64>() {
    Ok(n) => println!("{n} is a number!"),
    Err(e) => println!("{e} is NOT a number!"),
}
```

Of course, Rust supports dynamic size data structures such as `Vec`. It is defined in `std::vec`, but a set of things are auto imported in every program, as defined in `std::prelude`. The full list can be found [here](https://doc.rust-lang.org/std/prelude/index.html). One important method of `Vec` is `.sort_unstable_by`, which takes a mutable comparator that returns an `Ordering` (`FnMut(&T, &T) -> Ordering`) as argument.

## Day 2

In today's code, I also used the `match` syntax from before, this time to unpack / map values. For example, if we know the variable `vec: Vec<usize>` is of length 2, then we can unpack it into two `usize` variables `a` and `b` by writing

```rust
let (a, b) = match vec[..] { // matches tuple (a, b)
    [a, b] => (a, b),        // if vec is of pattern [a, b], then returns (a, b)
    _ => unreachable!(),     // unreachable
};
```

Also, the syntax of declaring a function is `fn <func-name>(<arg-name>: <type-name>, ...) [-> return-type]`. For example,

```rust
fn enhance(mut pixels: Vec<Vec<usize>>, r: usize, c: usize) -> Vec<Vec<usize>> {
    // ...
}
```

Other than that, the syntax required for today's problem is not too advanced.

## Day 3

In today's code, I had to implement finding common elements between two strings. At first, I tried something like

```rust
fn intersect(s1: &str, s2: &str) -> impl Iterator<Type = Char> {
    s1.chars().filter(|c| s2.contains(&c.to_string()))
}
```

However, I run into lifetime issues. I have not had time to fully understand what lifetime is yet, but **TODO: FILL THIS IN**
