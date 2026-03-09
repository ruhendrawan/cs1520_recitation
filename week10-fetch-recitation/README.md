# AJAX + fetch

Flask + JavaScript app for practicing `fetch()`:
- `GET /items` returns JSON
- `POST /new_item` adds an item
- the browser updates the DOM without reloading the page
- the same helper can simulate success or failure for both requests

## Run it

```bash
cd recitations/week09-fetch-recitation

python -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

flask --app app run --debug
```

Open: http://127.0.0.1:5000/

## Files to edit in recitation

- `static/app.js`
- `app.py` is intentionally small so you can read the API first.
- Use the `Simulate Failed Load / Post` checkbox to test error handling.

## What to finish

- In `loadItems()`, answer these with code:
- Where does the request start?
- Where do you check `response.ok` / `response.status`?
- Where do you use the JSON response to update the page?

- In `addItem()`, answer these with code:
- How do you build the POST body from the input text?
- How do you reuse the same status-check pattern?
- How do you update the page after the POST succeeds?

## Reference

- `cs1520_examples/javascript/js16_ajax.html`
