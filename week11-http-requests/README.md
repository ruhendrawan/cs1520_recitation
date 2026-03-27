# Week 11 — HTTP clients

This folder has the starter files for the Week 11 recitation.

## Files

- `raw_http.py` reviews a raw HTTP request over a socket.
- `api_ex.py` is the Week 11 Flask todo API from `cs1520_examples`.
- `client.py` is your starter for `requests`-based API calls.

## Setup

```bash
cd recitations/week11-http-requests

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

## Run the API server

```bash
flask --app api_ex run --debug
```

The server runs at `http://127.0.0.1:5000`.

## Work on the client

In a second terminal:

```bash
cd recitations/week11-http-requests
source .venv/bin/activate
python client.py
```

Open `client.py` and fill in the TODOs.

## Endpoints

- `GET /todos`
- `POST /todos`
- `GET /todos/<todo_id>`
- `PUT /todos/<todo_id>`
- `DELETE /todos/<todo_id>`
- `POST /count`
- `POST /mark`

## Optional CLI comparison

If you have HTTPie installed:

```bash
http GET :5000/todos
http GET :5000/todos/todo1
http POST :5000/todos task="rewrite in python"
http PUT :5000/todos/todo5 task="study HTTP" done:=true
http POST :5000/count type=done
http POST :5000/mark id=todo5 status:=false
http DELETE :5000/todos/todo5
```

If you have curlie installed:

```bash
curlie GET :5000/todos
curlie GET :5000/todos/todo1
curlie POST :5000/todos task="rewrite in python"
curlie PUT :5000/todos/todo5 task="study HTTP" done:=true
curlie POST :5000/count type=done
curlie POST :5000/mark id=todo5 status:=false
curlie DELETE :5000/todos/todo5
```
