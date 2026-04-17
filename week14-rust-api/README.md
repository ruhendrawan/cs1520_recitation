# Week 14 — Rust API

This folder has the starter files for the Week 14 recitation.

This week you will extend a small Axum todo API.

Slides: [Week 14 instructions](../instructions/14-rust-api-slides.html)

## Files

- `src/main.rs` wires routes together and creates the starter in-memory data.
- `src/util.rs` defines the `Todo` types shared across handlers.
- `src/todos.rs` handles the `/todos` collection endpoints and includes a starter
  signature for the new sorted route.
- `src/todo.rs` handles the `/todos/{todo}` item endpoints.
- `src/rpc.rs` contains the extra RPC-style endpoints.
- `tester.sh` has sample `curlie` commands for both the existing API and the two
  new week14 tasks.

## Setup

```bash
cd recitations/week14-rust-api
cargo run
```

The starter should build immediately. Then open the `src/` files and complete the
two recitation tasks.

## Recitation tasks

1. Add due-date support to todos.
2. Add a new endpoint that returns the todo list ordered by the nearest due date.

## Hints

1. `HashMap` does not preserve order, so the new sorted endpoint will probably need
   to return a `Vec<_>` or a new response type instead of `TodoList`.
2. If you want the starter data and old JSON payloads to keep working while you add
   due dates, make the new field optional.
3. Rust iterator methods from last week are useful here: `values()`, `cloned()`,
   `collect()`, and `sort_by()` / `sort_by_key()`.
4. There is already a starter signature for `get_sorted_by_due_date()` in
   `src/todos.rs` and matching requests in `tester.sh`.
