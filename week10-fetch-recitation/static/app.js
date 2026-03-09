const statusEl = document.getElementById("status");
const itemsEl = document.getElementById("items");
const formEl = document.getElementById("create-form");
const textEl = document.getElementById("text");
const loadBtn = document.getElementById("load-btn");
const failToggle = document.getElementById("fail-toggle");

function setStatus(message, isError = false) {
  statusEl.textContent = message;
  statusEl.classList.toggle("error", isError);
}

function getApiUrl(path) {
  const failQuery = failToggle.checked ? "?fail=1" : "";
  return `${path}${failQuery}`;
}

function render(items) {
  itemsEl.innerHTML = items.map((item) => `<li class="item">${item}</li>`).join("");
}

function loadItems() {
  setStatus("Loading...");
  const url = getApiUrl("/items");

  // Notes for your worksheet:
  // - Where does the request start?
  // - Where do you check the status?
  // - Where do you use the response JSON?

  // TODO: If the HTTP status is not OK, use setStatus to show the error message.
  // TODO: Use the JSON data to update the page.
  // TODO: What exactly is inside `data`?
  // TODO: Use render() to update the page with the items from the server response.
}

function addItem(e) {
  e.preventDefault();

  const text = textEl.value.trim();
  if (!text) {
    setStatus("Type something first.", true);
    return;
  }

  setStatus("Posting...");
  const url = getApiUrl("/new_item");

  // Notes for your worksheet:
  // - Why is this request a POST instead of a GET?
  // - Why does the lecture use application/x-www-form-urlencoded here?

  // TODO: Send the typed text in the request body.
  // TODO: Repeat the same HTTP status check you used in loadItems().
  // TODO: Render the updated items from the server response.
  // TODO: Clear the text box after a successful POST.
}

loadBtn.addEventListener("click", loadItems);
formEl.addEventListener("submit", addItem);

// Optional polling (try 2000–5000ms)
// window.setInterval(loadItems, 3000);
