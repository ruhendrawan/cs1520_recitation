---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 6: Python

Slides: `slides/05_python.pdf`  
Time: **50 minutes** (Tune In **10** / Activity **35** / Reflection + Submit **5**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work

Work in pairs
- Online Python coding
- 1 driver, 1 navigator/ note taker: switch driver every ~5 minutes
- talk out loud (what you’re trying, error you saw, you expect)

- a Google Doc for reflections. Create new tab for your pairs in the [Reflection Doc](https://docs.google.com/document/d/1Cub34Ci6NLqXZ0qxY4v-MuUN5PWI_F0NdKoEQf3RPys/edit?usp=sharing)
  - Paste your shared Online Python link there (Part 2)

---

## Part 1 — Tune In (10 minutes)

In turn, present brifely the status of your Project 1.

<!--
---

## ARCHIVED SLIDE
## Skip this, and go to Part 2

Pick **1–2 concepts** from `slides/05_python.pdf` and explain them in your own words (fast + messy is fine).

<div class="text-xs">

- dynamic typing vs strong typing
- list/dict/set basics
- list comprehensions
- iterators + generators
- exceptions (`try/except/else/finally`)
- classes + dunder methods (`__init__`, `__eq__`)

What is it (in one sentence)? What’s a common bug this concept prevents? Where would you use it in a real app (web, data, automation)?

</div>
-->

---

## Part 2 — Activity (35 minutes)

Learn one modern Python feature: **structural pattern matching** (`match` / `case`).

But, WHYY?
- In web apps you constantly “dispatch” based on **shape** of data (routes, JSON payloads, events).
- Without `match`, you end up with long `if/elif` chains that are harder to read and easier to get wrong.

---

## If you’re stuck

Try these (in order):
1. Print intermediate values (`print(...)`) to confirm your assumptions.
2. Read the traceback from the top and find the first line in *your* code.
3. Reduce the problem: make the smallest example that still fails.
4. Ask the TA with: the error + what you tried + what you expected.

---

## Step 0: Shared code link (3 minutes)

<div class="slide">
<div class="col text-xs">

Open: https://www.online-python.com/

1. Paste the starter below.
2. Click **Run** once.
3. Click **Share** and copy the link.
4. Paste that link into your Google Doc team tab. 

Checkpoint!

✅ you have a share link in your Google Doc
✅ your partner can open your link and run the code

</div>
<div class="col text-xs">

Write a function `handle(event)` where `event` is a `dict`. 

```py
def handle(event):
  # TODO: implement
  return "TODO"

tests = [
  {"type": "click", "x": 10, "y": 20},
  {"type": "key", "key": "Enter"},
  {"type": "quit"},
  {"type": "click", "x": -1, "y": 0},
  {"type": "click", "x": 1},
  {"type": "???"},
]

for t in tests:
  print(t, "=>", handle(t))
```

</div>
</div>

---

## Step 1: Dispatching (10 minutes)

❗ Handle these shapes. Do it using `if/elif`.
- `{"type": "click", "x": <int>, "y": <int>}`
- `{"type": "key", "key": <str>}`
- `{"type": "quit"}`
- anything else (unknown)

Checkpoint!
✅ all dict items can be printed properly, but the code feels “branchy”
✅ you had to manually check keys exist to avoid `KeyError`

---

## Step 2: Structural Pattern Matching (15 min)

❗ Now rewrite `handle(event)` using `match event:`.

<div class="slide">
<div class="col text-xs">

- Include a mapping pattern (dict shape):
  `case {"type": "click", "x": x, "y": y}:`
- Include at least 1 guard:
  only accept clicks where `x` and `y` are non-negative
- Include a default:
  `case _:` for unknown/invalid events

</div>
<div class="col">

```py
def handle(event):
  match event:
    # Implement the other sphapes
    case {"type": "quit"}:
      return "quit"
    case _:
      return "unknown/invalid"
```

</div>
</div>


<div class="text-xs">

Checkpoint!
✅ your code reads like a "rules book" 
✅ invalid shapes safely fall through to the default case

</div>

---

Readings

- [8 levels of using structural pattern matching](https://medium.com/techtofreedom/8-levels-of-using-structural-pattern-matching-in-python-d76282d5630f?sk=bc75658e9c10fc24789bd4479c358f86)
- [Structural pattern matching](https://benhoyt.com/writings/python-pattern-matching/)
- [PEP 636 quick intro](https://peps.python.org/pep-0636/#appendix-a-quick-intro)
- [Python reference (`match` statement)](https://docs.python.org/3/reference/compound_stmts.html#the-match-statement)

---

## Step 3: Data Validation (5 min)

<div class="slide text-xs">
<div class="col">

In web apps, inputs are messy.

Your backend will get JSON that is:
- missing keys
- wrong types
- extra keys
- values out of range

</div>
<div class="col">

❗ Update `handle(event)` so it returns **explicit error messages** for invalid data.

Add at least these validations:
- Click must have both `x` and `y`, and both must be `int`
- `x` and `y` must be `>= 0`
- Key events must have `key` as a non-empty string

</div>
</div>

<div class="text-sm">
Checkpoint:<br/>
✅ at least 2 different invalid inputs, shall return different useful error strings (raised exception, or printed log)
</div>

---

## Part 3 — Reflection + Submit (5 minutes)

No grammar check. Don’t overcorrect. Answer these:

- For a small script: is `match` necessary, or just nicer?
- For a web app that handles many event/request shapes: what goes wrong with long `if/elif` chains?
- Where do we see “shape-based dispatch” in web technology?
  - routes (`/users/<id>`), JSON payloads, UI events, API responses

Put in on the Google Doc, then
submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## Project 1: React (UI or state)

Prepare for submission!


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

