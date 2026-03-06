---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 8: Flask + SQLAlchemy + SQLite (Micro Blogging)

Slides: `instructions/08-flask-sqlalchemy-slides.html`  
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
- Explain what **Flask** does (routes → functions) and what a **database** does (durable state).
- Use **SQLAlchemy ORM** to model tables + relationships.
- Render dynamic pages with **Jinja2 templates** (server-side HTML).
- Implement a working demo of micro blogging features

---

## Part 1 — Tune In (5 minutes)

In turn, answer quickly:
- What “state” does a microblog need to keep **after** the server restarts?
- What’s the difference between:
  - data stored in a Python variable
  - data stored in SQLite
- What’s one microblog feature you use every day?

---

## Part 2 — Activity (40 minutes)

Goal: develop microblog, implement at least **two features**:

- **Posting**: create a post
- **Reading**: read your timeline (you + followed users)
- **Following**: follow/unfollow (optional if time)
- Any other feature you are familiar with

---

## Step 0 — Setup (5 minutes)

If you’re coding locally:

```bash
python -m venv .venv
source .venv/bin/activate
pip install flask sqlalchemy flask-sqlalchemy

flask --app app run --debug
```

Note: Flask uses **Jinja2** for templating (already included with Flask).

---

## Step 1 — Feature shortlist

Make a shortlist. Pick any **Basic** / **Advanced** features.

<div class="slide">
<div class="col text-sm">

Basic
- Create post (text)
- Timeline feed
- Follow/unfollow
- Profile page

</div>
<div class="col text-sm">

Advanced
- Likes / bookmarks
- Replies / threads
- Hashtags
- Search
- Pagination / infinite scroll
- Basic auth (even “fake login”)

</div>
</div>

---

## Step 2 — Data model (5 minutes)

Define the set of tables

<div class="slide">
<div class="col text-sm">

Starting schema:
- `User` (who)
- `Post` (what)
- `Follow` (who follows whom)

</div>
<div class="col text-sm">

Relationships:
- one user -> many posts
- many users <-> many users (follow)

</div>
</div>

---

## Example models (SQLAlchemy ORM)

<div class="text-xs">

```py
from datetime import datetime
from sqlalchemy import ForeignKey, String, func
from sqlalchemy.orm import Mapped, mapped_column, relationship

class User(db.Model):
  __tablename__ = "user"

  id: Mapped[int] = mapped_column(primary_key=True)
  username: Mapped[str] = mapped_column(String(32), unique=True, nullable=False)
  created_at: Mapped[datetime] = mapped_column(server_default=func.now())

  posts: Mapped[list["Post"]] = relationship(back_populates="user")

class Post(db.Model):
  __tablename__ = "post"

  id: Mapped[int] = mapped_column(primary_key=True)
  user_id: Mapped[int] = mapped_column(ForeignKey("user.id"), nullable=False)
  body: Mapped[str] = mapped_column(String(280), nullable=False)
  created_at: Mapped[datetime] = mapped_column(server_default=func.now(), index=True)

  user: Mapped["User"] = relationship(back_populates="posts")

class Follow(db.Model):
  __tablename__ = "follow"

  follower_id: Mapped[int] = mapped_column(ForeignKey("user.id"), primary_key=True)
  followed_id: Mapped[int] = mapped_column(ForeignKey("user.id"), primary_key=True)
  created_at: Mapped[datetime] = mapped_column(server_default=func.now())
```
</div>

---

## Step 3 — Routes (5 minutes)

Create a “route” list

<div class="slide">
<div class="col text-xs">

Read
- `GET /` → timeline feed
- `GET /u/<username>` → profile

</div>
<div class="col text-xs">

Write
- `POST /post` → create post
- `POST /follow/<username>` → follow
- `POST /unfollow/<username>` → unfollow

</div>
</div>

---

## Step 4 — Implementing the Logic 

- create a post, stored and persisted in database
- see it appear in a timeline

```py
app = Flask(__name__)
app.config["SQLALCHEMY_DATABASE_URI"] = "sqlite:///database.sqlite"
db = SQLAlchemy(app)

@app.get("/")
def timeline():
  ...

@app.post("/post")
def create_post():
  ...
```

---

## Jinja2 templating

Why templates?
- Route returns **HTML**, but the HTML should be **data-driven** (posts, usernames, timestamps).
- Jinja2 help combine: **query results** + **HTML** safely.

Convention:
- Put templates in a folder named `templates/`
- Use `render_template("page.html", data=...)`

---

## Timeline route -> template

<div class="slide">
<div class="col text-sm">

Route decides *what* data to show:

```py
from flask import render_template

@app.get("/")
def timeline():
  posts = Post.query.order_by(
      Post.created_at.desc()
    ).limit(50).all()

  return render_template(
    "timeline.html",
    posts=posts
  )
```

</div>
<div class="col text-sm">

Template decides *how* it looks:

```html
<!-- templates/timeline.html -->
<h1>Timeline</h1>
{% for p in posts %}
  <div class="post">
    <b>@{{ p.user.username }}</b>
    — {{ p.created_at }}
    <p>{{ p.body }}</p>
  </div>
{% endfor %}
```

</div>
</div>

---

## Template inheritance (Optional)

<div class="slide">
<div class="col text-xs">

Make a shared layout once.

```html
<!-- templates/base.html -->
<!doctype html>
<title>{% block title %}Microblog{% endblock %}</title>
<nav>
  <a href="{{ url_for('timeline') }}">Home</a>
</nav>
<main>{% block content %}{% endblock %}</main>
```

</div>
<div class="col text-xs">

Reuse in other pages.

```html
<!-- templates/timeline.html -->
{% extends "base.html" %}
{% block title %}Timeline{% endblock %}
{% block content %}
  <h1>Timeline</h1>
  ...
{% endblock %}
```

</div>
</div>

Checkpoint:
✅ you can explain why `base.html` makes your app easier to change

---

## Forms: create a post (Jinja2 + Flask)

HTML form for posting; server reads `request.form`; and redirects back to the timeline page.

```html
<form method="post" action="{{ url_for('create_post') }}">
  <textarea name="body" maxlength="280" required></textarea>
  <button type="submit">Post</button>
</form>
```

<div class="text-xs">

Checkpoint:
✅ understand the web request/ response lifecycle
✅ able to trace: form submit → route handler → DB insert → redirect → timeline render

</div>

---

## Safety: escaping string

Jinja2 escapes by default:
- `{{ p.body }}` is safe (HTML is escaped)
- Avoid `|safe` unless you fully control the content

Don't trust users' input. Treat post text as **plain text**, not HTML

---

## Step 5 — “Friends feed” query (optional)

<div class="text-xs">

Design the query before you code it:
- timeline = your posts + posts by people you follow

Pseudo-query:
- find followed user ids
- fetch recent posts where `user_id IN (followed + me)`
- order newest first

Checkpoint:
✅ you can write the SQL in words (“select posts where…”)  
✅ you can write the ORM query in code (even if it errors at first)

</div>

---

## Debugging ORM in a route with `breakpoint()`

<div class="slide">
<div class="col text-xs">

Place the `breakpoint()` in routed function

```py
@app.get("/")
def timeline():
  q = Post.query.order_by(Post.created_at.desc()).limit(50)
  breakpoint()  # pauses here when you load /
  posts = q.all()
  return render_template("timeline.html", posts=posts)
```

</div>
<div class="col text-xs">

Terminal (where `flask run` is running):

```bash
$ flask --app app run --debug
 * Running on http://127.0.0.1:5000

# hit the route (browser)

> app.py(??)timeline()
-> posts = q.all()
(Pdb) p q
<flask_sqlalchemy.query.Query object at 0x...>
(Pdb) p str(q.statement)
'SELECT post.id, post.user_id, post.body, post.created_at ...'
(Pdb) p q.all()
[<Post 12>, <Post 11>, ...]
(Pdb) c
```

</div>
</div>

Note: remove `breakpoint()` before you submit / deploy.


---

## Debugging tip: inspect SQLite

In a terminal:

```bash
sqlite3 database.sqlite
.tables
.schema user
select * from post order by created_at desc limit 5;
```

Checkpoint:
✅ you can confirm what rows exist without using your web UI

---

Readings

- https://flask.palletsprojects.com/en/stable/quickstart/
- https://flask-sqlalchemy.readthedocs.io/en/stable/models/#defining-models
- https://flask-sqlalchemy.readthedocs.io/en/stable/quickstart/
- https://jinja.palletsprojects.com/en/stable/templates/

---

## Part 3 — Reflection + Submit (5 minutes)

No grammar check. Don’t overcorrect. Answer these:

- What did you make (3–5 features)?
- What concept caused the most confusion, and why?
- Which concept felt easiest to implement? Which felt hardest?

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)



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
