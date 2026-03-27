---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 10: AJAX + `fetch()` with Flask

Slides: `slides/09_ajax.pdf`  
Time: **50 minutes** (Tune In **5** / Activity **40** / Reflection + Submit **5**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work today

Work in pairs (coding + reflections).

Keep 3 tabs open:
- this recitation instruction
- your code editor + terminal
- a Google Doc for reflections. Create new tab for your pairs in the [Reflection Doc](https://docs.google.com/document/d/1Cub34Ci6NLqXZ0qxY4v-MuUN5PWI_F0NdKoEQf3RPys/edit?usp=sharing)

---

## Learning goals (today)

By the end you should be able to:
- Explain the lecture flow: request -> status -> response.
- Use `fetch()` with `.then(...).catch(...)` to get JSON from Flask.
- Send a `POST` request using `application/x-www-form-urlencoded`.
- Check `response.ok` / `response.status` correctly.
- Update the DOM without a full page reload.
- Add basic polling if you finish early.

---

## Part 1 — Tune In (5 minutes)

In turn, answer quickly:
- Why does reloading the whole page feel less dynamic?
- Why is JSON easier to work with than XML here?
- In `js16_ajax.html`, where do you see:
  - the request start
  - the status check
  - the response handling
- True/False: `fetch()` rejects on HTTP 404.

---

## Part 2 — Activity (40 minutes)

Goal: finish a JavaScript app:
- `GET /items` returns JSON, handle request error
- `POST /new_item` adds one item, auto refresh, handle error
- the page updates **without full-page reload**
- optional: polling if you finish early

Starter code is in: `week10-fetch-recitation`

---

## Setup

From `week10-fetch-recitation`:

```bash
python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

flask --app app run --debug
```

Open: http://127.0.0.1:5000/

---

Reference example code:
- [javascript/js16_ajax.html](http://github.com/nfarnan/cs1520_examples/blob/main/javascript/js16_ajax.html)
- [javascript/js17_ajax_in_action/](https://github.com/nfarnan/cs1520_examples/tree/main/javascript/js17_ajax_in_action)
- [javascript/js18_ajax_poll/](http://github.com/nfarnan/cs1520_examples/tree/main/javascript/js18_ajax_poll)

Read `app.py` once before you start.
- `GET /` serves the page
- `GET /items` returns JSON
- `POST /new_item` updates the list and returns JSON

Recitation file is `static/app.js`.

---

## Part 3 — Reflection + Submit (5 minutes)

No grammar check. Don’t overcorrect. Answer these:

- What did you build?
- What was the biggest gotcha when implementing `fetch()`?
- Beside the loading status, what other things should be handled when using `fetch()` to make the UI more user friendly?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

<!-- _class: slide-title -->

# HINTS

---

## Step 1 — Map the lecture example to today (5 minutes)

Use `js16_ajax.html` as the reference pattern:

<div class="slide">
<div class="col text-xs">

### Lecture example

- create request
- send request
- wait for completion
- check `status`
- read `responseText`

</div>
<div class="col text-xs">

### Today

- `fetch("/items")`
- `.then(...)`
- check `response.ok`
- `return response.json()`
- render the result into the page

</div>
</div>

---

## Step 2 — Read data (GET) (10 minutes)

<div class="slide">
<div class="col text-xs">

Open `static/app.js`.

Implement:
- `loadItems()` -> `fetch("/items")`
- Check `response.ok`
- `return response.json()`
- Render items into the DOM

</div>
<div class="col text-xs">

Pattern:

```js
fetch("/items")
  .then((response) => {
    if (!response.ok) throw new Error(`HTTP ${response.status}`);
    return response.json();
  })
  .then((data) => {
    render(data.items);
  });
```

</div>
</div>

Checkpoint: clicking **Load Items** shows the list from the server.

---

## Step 3 — Write data (POST) (10 minutes)

<div class="text-sm">

Implement `addItem()`:
- form submit handler
- `fetch("/new_item", { method: "POST", ... })`
- send the body in the same format as the lecture slide
- render the updated list after success

```js
fetch("/new_item", {
  method: "POST",
  headers: { "Content-Type": "application/x-www-form-urlencoded" },
  // Automatically converted to "text=hot%20dog"
  body: new URLSearchParams({ text: text }),
  // body: `text=${encodeURIComponent(text)}`,
});
```

Checkpoint: adding an item updates the list without page reload.

</div>

---

## Step 4 — Test the `fetch()` gotcha (5 minutes)

<div class="text-sm">

`fetch()` only rejects on **network errors**.

HTTP errors (404/500) still resolve the Promise, so you must do:
- `if (!response.ok) { ... }`
- use `response.status` for the code

Quick test:
- check the **Simulate Failed Load / Post** box
- click **Load Items**
- confirm your error message shows `HTTP 500`

</div>

---

## Step 5 — Optional challenge: polling (5 minutes)

Add auto-refresh every ~2–5 seconds.

```js
window.setInterval(loadItems, 3000);
```

Discussion:
- When would polling make sense?
- When would it be wasteful?

---

## Debug checklist

If it “does not work”:
1. Open DevTools -> **Console** (JS errors?)
2. DevTools -> **Network** tab (request made? status code?)
3. Check the Flask terminal output
4. Check `response.ok` / `response.status`

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

section.slide-title h1 {
  display: grid;
  place-items: center;
  height: 100vh;
  text-align: center;
}

</style>
