#!/bin/bash

num=1

msg () {
  echo "#=== $num: $1 ==="
  ((num=num+1))
}

msg "GET all"
curlie $1/todos
msg "GET todo1"
curlie $1/todos/todo1
msg "GET (missing) todo4"
curlie $1/todos/todo4
msg "POST new todo"
curlie POST $1/todos task="rewrite in rust"
msg "GET all"
curlie $1/todos
msg "PUT new todo that's done"
curlie PUT $1/todos/todo5 task="...well start it at least" done:=true
msg "GET all"
curlie $1/todos
msg "PUT new todo that's not done"
curlie PUT $1/todos/todo6 task="impress students"
msg "GET all"
curlie $1/todos
msg "POST to /count"
curlie POST $1/count
msg "POST to /count with type=done"
curlie POST $1/count type=done
msg "POST to /count with type=not_done"
curlie POST $1/count type=not_done
msg "POST to /mark to set todo5 to not done"
curlie POST $1/mark id=todo5 status:=false
msg "GET all"
curlie $1/todos
msg "DELETE todo5"
curlie DELETE $1/todos/todo5
msg "GET all"
curlie $1/todos
msg "POST to /mark to set todo6 to done"
curlie POST $1/mark id=todo6 status:=true
msg "GET all"
curlie $1/todos

msg "POST new todo with a due date (task 1)"
curlie POST $1/todos task="submit lab" due_date="2026-04-18"
msg "PUT todo2 with an earlier due date (task 1)"
curlie PUT $1/todos/todo2 task="finish slides" done:=false due_date="2026-04-17"
msg "PUT todo3 with a later due date (task 1)"
curlie PUT $1/todos/todo3 task="profit!" done:=false due_date="2026-04-25"
msg "POST new todo without a due date"
curlie POST $1/todos task="mystery deadline"
msg "GET all to confirm due dates are being stored"
curlie $1/todos
msg "GET todos ordered by nearest due date (task 2)"
curlie $1/todos/by_due_date
