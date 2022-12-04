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
fn intersect(s1: str, s2: str) -> impl Iterator<Item = char> {
    s1.chars().filter(|c| s2.contains(&c.to_string()))
}
```

However, I run into lifetime issues. I have not had time to fully understand what lifetime is yet, but essentially currently, `s1.chars` is tied to `s1`. Since `.filter` returns an iterator that points into `s1.chars()`, this causes an issue as the compiler does not know when the memory location at `&s1` will be invalidated. Therefore, we have to specify the _lifetime_ of the objects, which tells the compiler how long the objects should live for i.e. how long they will stay valid. There are two basic syntaxes for lifetime: `'_` and `'a`, where the former is essentially an anonymous / generic form of the latter. Therefore, we can fix the code above by writing

```rust
fn intersect<'a>(s1: &'a str, s2: &'a str) -> impl Iterator<Item = char> + 'a {
    s1.chars().filter(|c| s2.contains(&c.to_string()))
}
```

Also, note that this problem goes away by converting the iterator into a collectable via `.collect()`, since it essentially clones each object, so the lifetime of the objects in the collectable is tied to the collectable instead.

By the way, the `intersect` function above is not used in the final solution, as part 2 requires intersecting 3 strings, so I figured out that it is probably easier to write a function with signature `fn intersect(v1: Vec<char>, s2: Vec<char>) -> Vec<char>`.

## Day 4

Today's problem is straightforward, but I learned quite a lot of features from Rust. Firstly, I created a `struct Interval` to wrap the endpoints of an interval in a nice data structure. In Rust, it is not possible to define object constructors, as everything should be "explicit". Therefore, I also created a `make_interval` function for that.

Next, I had to check whether an interval covers another intervals entirely, which involves swapping the intervals when the left endpoints are not sorted. This functionality is implemented in `std::mem::swap`. However, due to the "assign once only" property of Rust variables, I have to pass in mutable pointers into the functions instead. The code looks like this in the end:

```rust
fn cover_entire(x: &mut Interval, y: &mut Interval) -> bool {
    // [1, 4] -> [1, 2], [2, 3] but not [2, 5]
    if x.l > y.l {
        mem::swap(x, y);
    }
    (x.l == y.l) || (x.r >= y.r)
}

// ...

cnt += cover_entire(&mut interval1, &mut interval2) as usize;
```

Also since `std::mem` is part of the standard library, I did not have to modify the `Cargo.toml` file.
