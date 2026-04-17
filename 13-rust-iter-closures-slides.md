---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 13: Rust Iterators + Closures

Date: **April 10, 2026**  
Time: **50 minutes** (Review **10** / Activity **30** / Reflection + Submit **10**)

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
- explain what an `Iterator` produces and consumes
- write simple closures such as `|n| n % 2 == 0`
- use `filter`, `map`, `sum`, `product`, `fold`, and optionally `reduce`
- explain the ownership difference between `iter()` and `into_iter()`

---

## Connection to last week

Last week:
- you manually traversed a recursive list with `match`
- you had to think about `Box<T>`, borrowing, and mutation directly

This week:
- the traversal is done with iterator methods
- describe *what happens to each item* with closures
- Rust still enforces ownership, but the API is much more ergonomic

---

## Starter project

Open:

- `week13-rust-iter-closures`
- work in `src/main.rs`

Run:

```bash
cd recitations/week13-rust-iter-closures
cargo run
```

---

## Starter code

```rs
/**
 * Your task for this recitation is to play around with higher order functions
 * (map, reduce, fold, filter) in Rust. Notice that these are methods of
 * Iterator. You can use .collect() to convert the results back into a collection.
 */

fn main() {
    // Problem 1
    // Produce a vector that contains even numbers in the range (1 to 10)

    // Problem 2
    // Use the vector produced in Problem 1 and find the sum of all the numbers

    // Problem 3
    // Repeat Problem 2 but now multiply the numbers

    // Problem 4
    // Produce a vector containing the square of each element of the vector which
    // was produced in Problem 1

    // Problem 5
    // Find the sum of all odd numbers in the range (100 to 200) inclusive
}
```

---

## Readings 

- [Too Many Lists](https://rust-unofficial.github.io/too-many-lists/) is a good follow-up if you want to see how iterators are implemented for a custom collection.
- [Dangerust](https://cliffle.com/p/dangerust/) gives a low-level systems perspective on references, borrowing, and safe wrappers over unsafe details.
- The course example repo has small examples close to today’s syntax:
  [rs08_iter_clos](https://github.com/nfarnan/cs1520_examples/tree/main/rust/rs08_iter_clos)

---

## Part 2 — Reflection + Submit

No grammar check. Don't overcorrect. Answer these:

- Which iterator method felt most natural today?
- Where did ownership or borrowing show up while using iterators?
- What is still unclear about closures?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## Hints

If you are stuck, come here later.

---

## Suggested order

1. Use a range and `filter()` to solve Problem 1.
2. Reuse that vector with `sum()` for Problem 2.
3. Reuse it again with `product()` for Problem 3.
4. Use `map()` and `collect()` for Problem 4.
5. Combine range, `filter()`, and `sum()` for Problem 5.

If you finish early, redo Problems 2 or 3 with `fold()`.

---

## Iterator

Collections and ranges can produce iterators.

At the core is a `next()` method that yields one item at a time:

```rust
fn next(&mut self) -> Option<Self::Item>
```

Most of today will use higher-order methods built on top of that:
- `filter`, `map`, `sum`, `product`, `fold`, `reduce`

---

## closures

Closures are anonymous functions.

Examples:

```rust
|n| n % 2 == 0
|n| n * n
|acc, n| acc + n
```

These are passed into iterator methods.

---

## Closure capture

Closures can also use values from the surrounding scope:

```rust
let cutoff = 5;

let bigger: Vec<i32> = (1..=10)
    .filter(|n| *n > cutoff)
    .collect();
```

That ability to borrow or capture context is what makes closures more flexible than plain function names.

---

## Useful patterns

```rust
(1..=10).filter(|n| /* predicate */)
```

```rust
values.iter().copied().sum::<i32>()
```

```rust
values.iter().map(|n| /* transform */).collect::<Vec<_>>()
```

```rust
values.iter().copied().fold(/* init */, |acc, n| /* combine */)
```

---

## `iter()` vs `into_iter()`

- `iter()` borrows each item
- `iter_mut()` mutably borrows each item
- `into_iter()` consumes the collection and takes ownership of each item

If you still need the original vector later, which one should you choose?

---

## Common gotchas

- forgetting that `collect()` usually needs a target type
- using `into_iter()` and then trying to use the vector again
- mixing `&i32` and `i32` without `copied()`
- writing the closure correctly but choosing the wrong range endpoints

