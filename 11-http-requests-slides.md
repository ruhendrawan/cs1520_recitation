---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 11: HTTP Clients

Slides: `slides/10_apis.pdf`  
Time: **50 minutes** (Part 1 Review **15** / Part 2 Activity **30** / Reflection + Submit **5**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work today

Work in pairs (coding + reflections).

Keep 3 tabs open:
- this recitation instruction
- your code editor + terminal
- a Google Doc for reflections. Create a new tab for your pair in the [Reflection Doc](https://docs.google.com/document/d/1Cub34Ci6NLqXZ0qxY4v-MuUN5PWI_F0NdKoEQf3RPys/edit?usp=sharing)
- Check "TEMPLATE Week 11"

---

## Learning goals (today)

By the end you should be able to:
- Explain what an HTTP request and response contain.
- Identify method, query string, headers, and body in a raw HTTP exchange.
- Describe the high-level differences between HTTP/1.0, HTTP/1.1, HTTP/2, and HTTP/3.
- Use `requests` to call API endpoints like an integration test.
- Optional: Compare Python `requests` with CLI clients like HTTPie or curlie.

---

## Part 1 — Review raw HTTP in Python

Open, Run and Review: `recitations/week11-http-requests/raw_http.py`
Answer the questions in the Doc.

```py
import socket

s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
s.connect(("example.com", 80))
s.sendall(b"GET / HTTP/1.1\r\nHost: example.com\r\n\r\n")

response = s.recv(4096)
print(response.decode())
```

---

## Part 2 — Build an HTTP client with `requests`

Open: `recitations/week11-http-requests`
- `api_ex.py` -> Week 11 Flask todo API from `cs1520_examples`
- `client.py` -> your starter for integration-style API calls

Goal:
- run the API locally
- write Python functions that call each endpoint
- inspect status, headers, and response bodies

---

## Step 0 — Setup

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

Terminal 1:

```bash
flask --app api_ex run --debug
```

Terminal 2:

```bash
python client.py
```

---

## Step 1 — Inspect the API

Read `api_ex.py` once before coding.

<div class="slide">
<div class="col text-xs">

Endpoints:
- `GET /todos`
- `POST /todos`
- `GET /todos/<todo_id>`
- `PUT /todos/<todo_id>`
- `DELETE /todos/<todo_id>`
- `POST /count`
- `POST /mark`

</div>
<div class="col text-xs">

Make notes on
- method differences
- path parameters
- request body
- response body
- status codes

</div>
</div>

---

## Step 2 — Fill in `client.py`

<div class="slide">
<div class="col text-xs">

Implement these functions:
- `list_todos()`
- `get_todo(todo_id)`
- `create_todo(task)`
- `upsert_todo(todo_id, task, done)`
- `count_todos(todo_type=None)`
- `mark_todo(todo_id, status)`
- `delete_todo(todo_id)`

</div>
<div class="col text-xs">

Use the Requests features that match the HTTP concept:
- `requests.get(url)`
- `requests.post(url, json=...)`
- `requests.put(url, json=...)`
- `requests.delete(url)`
- `response.status_code`
- `response.headers`
- `response.json()`

</div>
</div>

---

## Requests patterns

<div class="text-xs">

### GET / list

```py
requests.get(url)
```

### POST / create

```py
requests.post(url, json={"task": "rewrite in python"})
```

</div>

<div class="text-xs">

### PUT / update or create

```py
requests.put(url, json={"task": "study HTTP", "done": True})
```

### DELETE / remove

```py
requests.delete(url)
```

</div>
</div>

---

## Step 3 — Think like an integration test

- Did the status code match what you expected?
- Did the response body contain the right data?
- If you `PUT` a missing todo, did it create a new resource?
- If you `DELETE` a todo, did you get `204 No Content`?

Optional:
- add `assert` statements after each call
- treat `client.py` like a tiny API test script

---

## Optional CLI comparison

With HTTPie or curlie, compare one endpoint with a CLI client:

```bash
http GET :5000/todos
http GET :5000/todos/todo1
http POST :5000/todos task="rewrite in python"
http PUT :5000/todos/todo5 task="study HTTP" done:=true
http POST :5000/count type=done
http POST :5000/mark id=todo5 status:=false
http DELETE :5000/todos/todo5
```

- What feels easier to inspect in CLI?
- What feels easier to automate in Python?

---

## Resources

<div class="slide">
<div class="col text-xs">

- HTTP basics: [MDN Overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview)
- HTTP messages: [MDN Messages](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Messages)
- Protocol versions: [MDN Evolution of HTTP](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Evolution_of_HTTP)
- Python client: [Requests Quickstart](https://requests.readthedocs.io/en/latest/user/quickstart/)
- [HTTP from scratch using Python sockets](https://hakk.dev/blog/posts/http-from-scratch-python-sockets/)

</div>
<div class="col text-xs">

- HTTPie: [httpie.io/download](https://httpie.io/download)
- HTTPie examples: [HTTPie CLI examples](https://httpie.io/docs/cli/examples)
- curlie: [github.com/rs/curlie](https://github.com/rs/curlie)
- Flask API example: [fl11_api/api_ex.py](https://github.com/nfarnan/cs1520_examples/blob/main/flask/fl11_api/api_ex.py)
- API testing example: [rs11_axum_api/tester.sh](https://github.com/nfarnan/cs1520_examples/blob/main/rust/rs11_axum_api/tester.sh)

</div>
</div>

---

## Part 3 — Reflection + Submit (5 minutes)

No grammar check. Don’t overcorrect. Answer these:

- What part of the raw HTTP example made the most sense?
- Which part of the request/response format still feels confusing?
- Which `requests` feature was most useful today?
- If you had to test an API tomorrow, would you start with Python, HTTPie, or curlie? Why?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

HINTS

---

## What's in `raw_http.py`

- `socket.AF_INET` -> IPv4
- `socket.SOCK_STREAM` -> TCP stream
- `connect(("example.com", 80))` -> open a TCP connection to port 80
- `sendall(...)` -> send raw HTTP request bytes over that TCP connection
- `recv(...)` -> read raw response bytes back from the server
- `decode()` -> turn bytes into a Python string for printing

---

## What is inside the request?

```http
GET / HTTP/1.1
Host: example.com

```

Identify these parts:
- Request line
- Request header
- Blank line between headers and body
- Request body

Question: does this request include a body?

---

## What is inside the response?

Typical structure:

```http
HTTP/1.1 200 OK
Content-Type: text/html
Content-Length: ...

<html>...</html>
```

Identify these parts:
- Response status line
- Response headers
- Blank line
- Response body

---

## HTTP quick sheet

<div class="slide">
<div class="col text-sm">

### Request side

- **Method**: what action? (`GET`, `POST`, `PUT`, `DELETE`, ...)
- **Path**: which resource? (`/api/ping`)
- **Query string**: extra URL data (`?q=http&limit=2`)
- **Headers**: metadata (`Host`, `Accept`, `Authorization`, ...)
- **Body**: extra data sent with the request

</div>
<div class="col text-sm">

### Response side

- **Status code**: success or failure (`200`, `201`, `404`, `500`)
- **Headers**: metadata about the response
- **Body**: the returned content (`HTML`, `JSON`, text, image, ...)

</div>
</div>

---

## Method differences

- `GET` -> ask for data
- `POST` -> send data to create/process something
- `PUT` -> replace/update a resource
- `PATCH` -> partially update a resource
- `DELETE` -> remove a resource

---

## HTTP protocol versions

Use the MDN summary to compare:

- **HTTP/0.9**: one-line `GET`, no headers, no status codes
- **HTTP/1.0**: version in request line, headers, status line
- **HTTP/1.1**: persistent connections, `Host` header, more standardized behavior
- **HTTP/2**: binary, multiplexed, header compression
- **HTTP/3**: same HTTP semantics, but over QUIC instead of TCP

The socket example is written as **HTTP/1.1**.


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
