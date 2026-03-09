from flask import Flask, jsonify, render_template, request

app = Flask(__name__)
items = ["coffee", "bagel", "fried rice", "satay", "tempeh"]


def should_fail():
    return request.args.get("fail") == "1"


@app.get("/")
def index():
    return render_template("index.html")


@app.get("/items")
def get_items():
    if should_fail():
        return jsonify(error="intentional error"), 500
    return jsonify(items=items)


@app.post("/new_item")
def create_item():
    if should_fail():
        return jsonify(error="intentional error"), 500

    text = request.form.get("text", "").strip()

    if not text:
        return jsonify(error="text is required"), 400

    items.append(text)
    return jsonify(items=items)
