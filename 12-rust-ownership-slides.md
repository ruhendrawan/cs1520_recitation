---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 12: Rust Ownership with a Linked List

Slides: `slides/11_rust.pdf`  
Date: **April 3, 2026**  
Time: **50 minutes** (Ownership Review **10** / Linked List Work **35** / Reflection + Submit **5**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work today

Work in pairs (coding + reflections).

Keep 3 tabs open:
- this recitation instruction
- your code editor + terminal
- a Google Doc for reflections. Create a new tab for your pair in the [Reflection Doc](https://docs.google.com/document/d/1Cub34Ci6NLqXZ0qxY4v-MuUN5PWI_F0NdKoEQf3RPys/edit?usp=sharing)

---

## Learning goals (today)

By the end you should be able to:
- Explain why recursive data structures in Rust need `Box<T>`.
- Use `enum` + `match` to represent and process a linked list.
- Practice ownership and borrowing while traversing/mutating a list.
- Implement `new()`, `append()`, `length()`, and `Display` for a generic list.

---

## Part 1 — Ownership review (10 minutes)

Look at the ownership hint:

- [rs07_ownership example](https://github.com/nfarnan/cs1520_examples/blob/main/rust/rs07_ownership/src/main.rs)

Questions:
- Why can’t a recursive enum store itself directly?
- Why does `append()` probably need `&mut self`?
- When should `length()` borrow immutably instead?

---

## The task

Your task is to implement a generic linked list using Rust.
You only have to implement functionality for the following things:

1. Creating a new empty list via `new()`

2. Adding element to the end of a list via `append()`

3. Finding the length of the list via `length()`

4. Printing the linked list using `println!`

---

## Starter project

```rs
use std::fmt;

enum List<T> {
    // Your code here
}

impl<T> List<T> {
    // Your code here
    // Define at least three functions here, new (creates empty list), append, length
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Your code here
    }
}

fn main() {
    // This should run without any errors

    let mut list = List::<i32>::new();
    list.append(10);
    list.append(20);
    list.append(30);
    list.append(40);
    list.append(50);
    println!("{}", list);
    println!("List Length :: {}", list.length());
}
```

---

## Suggested print format

Any readable format is fine if it is consistent.

Examples:

```text
10 -> 20 -> 30 -> Nil
```

or

```text
[10, 20, 30]
```

Pick one and keep it simple.

---

If you get stuck:
- read the compiler error carefully
- check whether you need `&self` or `&mut self`
- check whether you are moving a value when you only meant to borrow it

---

## Resources

- Box and recursive types: [Rust Book ch15.1](https://doc.rust-lang.org/book/ch15-01-box.html)
- `match`: [Rust by Example: match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- Ownership hint: [rs07_ownership](https://github.com/nfarnan/cs1520_examples/blob/main/rust/rs07_ownership/src/main.rs)
- Rust book: [The Rust Programming Language](https://doc.rust-lang.org/book/)

---

## Part 2 — Reflection + Submit (5 minutes)

No grammar check. Don’t overcorrect. Answer these:

- Which ownership rule caused the most trouble?
- What part of the list implementation felt easiest?
- What part still feels confusing?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## Hint

If you are stuck, come here later.

---

## What is ownership?

Ownership is Rust’s rule system for who is responsible for a value at any moment.

- every value has one owner
- moving a value transfers ownership
- after a move, the old variable can no longer use that value
- when the owner goes out of scope, Rust drops the value automatically

---

Borrowing lets you use a value without taking ownership:
- `&x` -> immutable borrow
- `&mut x` -> mutable borrow

For today’s linked list:
- `append()` needs `&mut self` because it changes the list
- `length()` uses `&self` because it only reads
- `Box<T>` gives recursive nodes a clear owner on the heap

---

Populate an `enum` which has `Cons` and `Nil` variants. You should use the `Box<T>` smart pointer for this task to allocate values on the heap. You should take a look at the link below for more details on this:

<https://doc.rust-lang.org/book/ch15-01-box.html>

---

For implementing the printing, implement the `Display` trait for the list. There is provided starter code which is missing a `match` statement that you have to complete.

<https://doc.rust-lang.org/rust-by-example/flow_control/match.html>

---

For append and length, you may also use `match` but the function headers for those can be a bit tricky. Take a look at this example for some hints on ownership rules:

<https://github.com/nfarnan/cs1520_examples/blob/main/rust/rs07_ownership/src/main.rs>

---

If you are stuck, think function-by-function:

- `new()` returns the empty list
- `append()` walks to the end and replaces the empty case
- `length()` returns `0` for empty and `1 + ...` for non-empty
- `Display` should match on the same enum shape

Question:
- which one of these needs mutation?

---

## Common Rust gotchas

- recursive enum without `Box<T>` -> size problem
- using `self` instead of `&mut self` in `append()`
- moving out of a borrowed value by mistake
- forgetting a `match` case for `Nil`
- confusing `Box<List<T>>` with `List<T>`


<style>
.slide{
    display: flex;
}
.col{
    flex: 1;
}
.text-xxxs { font-size: 0.5rem; }
.text-xxs { font-size: 0.625rem; }
.text-xs { font-size: 0.75rem; }
.text-sm { font-size: 0.875rem; }
.text-base { font-size: 1rem; }
.text-lg { font-size: 1.125rem; }
.text-xl { font-size: 1.25rem; }
.text-2xl { font-size: 1.5rem; }
</style>
