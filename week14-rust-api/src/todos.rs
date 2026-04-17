use crate::util::{NewTodo, SharedTodoList, TodoList, TodoWithId};
use axum::{extract::State, http::StatusCode, Json};
use std::collections::HashMap;

pub async fn get(State(todo_list): State<SharedTodoList>) -> (StatusCode, Json<TodoList>) {
    (StatusCode::OK, Json(todo_list.read().unwrap().clone()))
}

#[allow(dead_code)]
pub async fn get_sorted_by_due_date(
    State(_todo_list): State<SharedTodoList>,
) -> (StatusCode, Json<Vec<TodoWithId>>) {
    // Recitation task 2:
    // 1. Read the todo HashMap from shared state.
    // 2. Convert it into a Vec<TodoWithId> so each result still has its ID.
    // 3. Sort the Vec so the earliest due date appears first.
    // 4. Decide how todos with `None` due dates should be ordered.
    // 5. Return the ordered Vec as JSON.
    todo!("implement GET /todos/by_due_date")
}

pub async fn post(
    State(todo_list): State<SharedTodoList>,
    Json(body): Json<NewTodo>,
) -> (StatusCode, Json<TodoList>) {
    let new_id = todo_list
        .read()
        .unwrap()
        .keys()
        .max()
        .unwrap_or(&String::from("todo0"))
        .chars()
        .skip(4)
        .collect::<String>()
        .parse::<i32>()
        .unwrap()
        + 1;

    let new_id = format!("todo{new_id}");
    todo_list
        .write()
        .unwrap()
        .insert(new_id.clone(), body.clone().into());

    (
        StatusCode::CREATED,
        Json(HashMap::from([(new_id, body.into())])),
    )
}
