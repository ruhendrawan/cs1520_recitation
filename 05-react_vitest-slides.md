---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 5: React Testing Library 

Slides: `slides/03_react.pdf`  
Time: **50 minutes** (Tune In **10** / Activity **35** / Reflection + Submit **5**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work

Work in pairs (discuss and share what you know/ assume/ learn)

---

## Part 1 — Tune In (10 minutes)

Pick **1–2 concepts** from `slides/03_react.pdf` and explain them in your own words (fast + messy is fine).

<div class="text-xs">

- App.js vs index.js (or main.js)
- document.getElementById()
- createRoot()
- useState
- props in the component function
- lifting state

What is it (in one sentence)? What’s a common bug this concept prevents? Where would you use it in a real app (web, data, automation)?

</div>

---

## Part 2 — Activity (35 minutes)

Keep working in pairs.
Develop the code individually in https://classroom.github.com/a/fd5rEEZc

Learn to write `vitest` for React project using [React Testing Library](https://testing-library.com/docs/react-testing-library/intro).

But, WHYY?
- Creates shared understanding, between team members, of expected component behavior
- Tests UI component behavior (what renders and how it responds to state changes)
- Confident in refactoring (improve code readability without modifying behaviors)

---

## If you’re stuck

Try these (in order):
1. Print intermediate values (`print(...)`) to confirm your assumptions.
2. Read the traceback from the top and find the first line in *your* code.
3. Reduce the problem: make the smallest example that still fails.
4. Ask the TA with: the error + what you tried + what you expected.

Read
- [React Testing Library](https://testing-library.com/docs/react-testing-library/intro).
- [React Testing Library Tutorial](https://www.robinwieruch.de/react-testing-library/)

---

## Test the Tic-Tac-Toe project

`src/App.jsx` contains the code from the React Tic-Tac-Toe tutorial. Write some tests in `src/App.test.jsx` to verify it is working correctly.

You should write one test to verify a case when X would win the game, and another to verify a case when O would win the game.

You are free to modify the application in `App.jsx` to better support testing
Refer to the documentation for React Testing Library. 

Specifically note that you can use the aria-label property of HTML elements to help match an element by role and name: https://testing-library.com/docs/queries/byrole/#api

Refer to the provided tests from `Project 1` for additional usage examples.

Note that liberal use of `regular expressions` are made in Testing Library. `/pattern/i` will perform case-insensistive matches looking for the string "pattern".

---

## Part 3 — Submit (5 minutes)

Submit your attendance in the google form: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## Project 1: React (UI or state)

Start working on your React final project individually:
- UI: lay out the core screens/components (even static)
- State: define your state shape + wire up one “happy path”

Pick any small deliverable: a new component renders with placeholder data; or one `useState` flow updates UI on click.

You'll be presenting (casually) your progress and personal challenge next week, to each other.



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

