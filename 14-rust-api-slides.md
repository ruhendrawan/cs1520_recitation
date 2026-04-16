---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 14: Rust API with Axum

Date: **April 17, 2026**  
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
- read Axum route + handler signatures
- extend serde structs with optional JSON fields
- convert `HashMap` data into an ordered JSON response
- test local API behavior with `curlie` and `tester.sh`

---

## Connection to last week

Last week:
- you used iterators and closures to transform collections
- you practiced `filter`, `map`, `collect`, and consumers like `sum`

This week:
- those same iterator ideas help build API responses
- you will turn `HashMap` entries into a `Vec`
- then sort the results by nearest due date

---

## Starter project

- `week14-rust-api`
- work mainly in `src/main.rs`, `src/util.rs`, and `src/todos.rs`

Run:

```bash
cd recitations/week14-rust-api
cargo run
```

In another terminal:

```bash
bash tester.sh http://localhost:5000
```

---

## Current API 

<div class="slide">
<div class="col text-sm">

Already in the starter:
- `GET /todos`
- `POST /todos`
- `GET /todos/{todo}`
- `PUT /todos/{todo}`
- `DELETE /todos/{todo}`
- `POST /count`
- `POST /mark`

</div>
<div class="col text-sm">

You will add:
- due-date support in todo JSON payloads
- `GET /todos/by_due_date`

</div>
</div>

---

## Task 1: Add due-date support

<div class="slide">
<div class="col text-sm">

```rs
#[derive(Serialize, Deserialize, Clone)]
pub struct NewTodo {
    task: String,
    // add: due_date: Option<String>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    task: String,
    pub done: bool,
    // add the same optional field here too
}
```

</div>
<div class="col text-sm pl-1">

Work in `src/util.rs`.

- update both structs
- update `Todo::new(...)`
- update `impl From<NewTodo> for Todo`

</div>
</div>

---

## Task 2: Add the sorted endpoint

Work in `src/todos.rs` and `src/main.rs`.

```rs
pub async fn get_sorted_by_due_date(
    State(_todo_list): State<SharedTodoList>,
) -> (StatusCode, Json<Vec<TodoWithId>>) {
    todo!("implement GET /todos/by_due_date")
}
```

Also connect the route in `main.rs`:

```rs
.route("/todos/by_due_date", get(todos::get_sorted_by_due_date))
```

---

## Part 2: Reflection + Submit

No grammar check. Don't overcorrect. Answer these:

- Where did week13 iterator methods show up in today's API work?
- What was the trickiest Axum or serde detail?
- How did you decide to handle todos with no due date?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## Hint

If you are stuck, come here later.

---

## Why not return a `HashMap`?

- `HashMap` does not preserve any meaningful order
- a sorted response should be a JSON array
- `TodoWithId` keeps the todo id after leaving the map

Example response:

```json
[
  { "id": "todo2", "task": "finish slides", "done": false, "due_date": "2026-04-17" },
  { "id": "todo4", "task": "submit lab", "done": false, "due_date": "2026-04-18" }
]
```

---

## Suggested order

1. Add `due_date` to `NewTodo` and `Todo`.
2. Update `Todo::new` and `From<NewTodo> for Todo`.
3. Test `POST` and `PUT` with a `due_date`.
4. Add the new route in `main.rs`.
5. Implement `get_sorted_by_due_date()`.
6. Re-run `tester.sh`.

---

## Useful iterator pattern

```rs
let mut items: Vec<TodoWithId> = todo_list
    .iter()
    .map(|(id, todo)| TodoWithId {
        id: id.clone(),
        todo: todo.clone(),
    })
    .collect();
```

Then sort the vector before returning it:

```rs
items.sort_by_key(|item| {
    // choose a key that puts earlier due dates first
});
```

---

## JSON examples

Create with a due date:

```json
{ "task": "submit lab", "due_date": "2026-04-18" }
```

Update with a due date:

```json
{ "task": "finish slides", "done": false, "due_date": "2026-04-17" }
```

If you use ISO strings like `YYYY-MM-DD`, simple string ordering matches date ordering.

---

## Testing workflow  `tester.sh`

- `POST /todos` with a `due_date`
- `PUT /todos/{todo}` with earlier and later dates
- `POST /todos` without a due date
- `GET /todos/by_due_date`

Use it to verify:
- old requests still work
- due dates appear in JSON responses
- todos are ordered the way you intended

---

## Common gotchas

- forgetting to update both `NewTodo` and `Todo`
- forgetting to copy `due_date` in `From<NewTodo>`
- adding the handler but not wiring the route in `main.rs`
- trying to return a sorted `HashMap`
- not deciding what to do with `None` due dates

---

## Q/A

- `Option<String>` is enough for this recitation. You do not need a date crate.
- ISO date strings such as `2026-04-17` sort correctly as plain strings.
- One reasonable approach is to sort by a tuple that puts `None` last.
- Read the compiler error literally. In Rust it usually tells you the exact type mismatch.

<style>
.slide{
    display: flex;
}
.col{
    flex: 1;
}
.pl-1 { padding-left: 0.25rem; }
.text-xxxs { font-size: 0.5rem; }
.text-xxs { font-size: 0.625rem; }
.text-xs { font-size: 0.75rem; }
.text-sm { font-size: 0.875rem; }
.text-base { font-size: 1rem; }
.text-lg { font-size: 1.125rem; }
.text-xl { font-size: 1.25rem; }
.text-2xl { font-size: 1.5rem; }
</style>
