# Rust linked-list

Your task is to implement a generic linked list using Rust. You only have to
implement functionality for the following things:

1. Creating a new empty list via `new()`
2. Adding element to the end of a list via `append()`
3. Finding the length of the list via `length()`
4. Printing the linked list using `println!`

## Setup

```bash
cd recitations/week12-rust-linked-list
cargo run
```

The whole skeleton project is already provided.

Work directly in:

- `src/main.rs`

You only need to fill in the missing enum variants, methods, and `Display` body.

## Hints

1. Populate an `enum` which has `Cons` and `Nil` variants. You should use the `Box<T>`
   smart pointer for this task to allocate values on the heap. You should take a
   look at the link below for more details on this:

   <https://doc.rust-lang.org/book/ch15-01-box.html>

2. For implementing the printing, implement the `Display` trait for the list.
   There is provided starter code which is missing a `match`
   statement that you have to complete.

   <https://doc.rust-lang.org/rust-by-example/flow_control/match.html>

3. For append and length, you may also use `match` but the function headers for those
   can be a bit tricky. Take a look at this example for some hints on ownership
   rules:

   <https://github.com/nfarnan/cs1520_examples/blob/main/rust/rs07_ownership/src/main.rs>
