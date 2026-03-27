import requests


BASE_URL = "http://127.0.0.1:5000"


def build_url(path):
    return f"{BASE_URL}{path}"


def show_response(label, response):
    print(f"\n=== {label} ===")
    print(f"{response.request.method} {response.request.url}")
    print(f"STATUS: {response.status_code}")
    print("HEADERS:")

    interesting_headers = [
        "Content-Type",
        "Location",
        "X-API-Version",
        "X-Result-Count",
        "X-Seen-Header",
    ]
    for header_name in interesting_headers:
        if header_name in response.headers:
            print(f"  {header_name}: {response.headers[header_name]}")

    print("BODY:")
    try:
        print(response.json())
    except ValueError:
        print(response.text)


def list_todos():
    """
    TODO:
    - Send GET /todos
    - Check the status code
    - Print the response with show_response(...)
    - Return the response
    """
    # Notes:
    # - Why is GET the right method here?
    # - What does the response body look like: list, dict, or something else?
    pass


def get_todo(todo_id="todo1"):
    """
    TODO:
    - Send GET /todos/<todo_id>
    - Print the response
    - Return the response
    """
    # Notes:
    # - What status code should you expect if the todo does not exist?
    pass


def create_todo(task="rewrite in python"):
    """
    TODO:
    - Send POST /todos
    - Use json=... for the request body
    - Print the response
    - Return the response
    """
    # Notes:
    # - Which status code means a resource was created?
    # - Which part of the request is the body?
    pass


def upsert_todo(todo_id="todo5", task="study HTTP", done=True):
    """
    TODO:
    - Send PUT /todos/<todo_id>
    - Use json=... for task and done
    - Print the response
    - Return the response
    """
    # Notes:
    # - When should this return 200?
    # - When should this return 201?
    pass


def count_todos(todo_type=None):
    """
    TODO:
    - Send POST /count
    - If todo_type is provided, send it as JSON with key "type"
    - Print the response
    - Return the response
    """
    # Notes:
    # - Why is this more RPC-style than REST-style?
    # - What body should you send for "done" vs "not_done"?
    pass


def mark_todo(todo_id="todo2", status=True):
    """
    TODO:
    - Send POST /mark
    - Use json=... with id and status
    - Print the response
    - Return the response
    """
    # Notes:
    # - Which fields must be present in the JSON body?
    # - What happens if the todo id does not exist?
    pass


def delete_todo(todo_id="todo3"):
    """
    TODO:
    - Send DELETE /todos/<todo_id>
    - Print the response
    - Return the response
    """
    # Notes:
    # - Which status code should you expect for a successful delete?
    # - Why might the response body be empty here?
    pass


def main():
    print("Start the API first with: flask --app api_ex run --debug")
    print("Then implement and call the functions above.")

    # After you implement them, uncomment and expand this sequence.
    #
    # list_response = list_todos()
    # todo_response = get_todo("todo1")
    # create_response = create_todo()
    # upsert_response = upsert_todo()
    # count_all_response = count_todos()
    # count_done_response = count_todos("done")
    # mark_response = mark_todo()
    # delete_response = delete_todo("todo3")
    #
    # Optional integration-test style checks:
    # assert list_response.status_code == 200
    # assert todo_response.status_code == 200
    # assert create_response.status_code == 201
    # assert upsert_response.status_code in (200, 201)
    # assert count_all_response.status_code == 200
    # assert count_done_response.status_code == 200
    # assert mark_response.status_code == 200
    # assert delete_response.status_code == 204


if __name__ == "__main__":
    main()
