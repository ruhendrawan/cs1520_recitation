---
theme: gaia
_class: lead
paginate: true
backgroundColor: #fff
backgroundImage: url('https://marp.app/assets/hero-background.svg')
---

## Week 4: React and the DOM

Slides: `slides/03_react.pdf`  
Time: **50 minutes** (Tune In **5** / Cycle 1 **15** / Cycle 2 **15** / Cycle 3 **15**)

Get this instruction: [https://github.com/ruhendrawan/cs1520_recitation](https://github.com/ruhendrawan/cs1520_recitation)

---

## How to work today

Work in pairs (coding + reflections).

Keep 3 tabs open:
- this recitation instruction 
- the tutorial
- a Google Doc for reflections. Create new tab for your pairs in the [Reflection Doc](https://docs.google.com/document/d/1Cub34Ci6NLqXZ0qxY4v-MuUN5PWI_F0NdKoEQf3RPys/edit?usp=sharing)

---

## Part 1 — Tune In (5 minutes)

Pick **1–2 concepts** from `slides/03_react.pdf` that you think you’ll use *immediately* in the tutorial. For example:
- React components render UI
- Vite + npm workflow (`npm create vite@latest`, `npm install`, `npm run dev`)
- JSX basics (`className`, `{}` expressions)
- `useState` → state update → re-render

No grammar check. Fast + messy is fine.

---

## Today’s activity (React tutorial)

We’re doing: https://react.dev/learn/tutorial-tic-tac-toe

- First-time React: ~**90–150 min** total
- Some React experience: ~**45–90 min** total

In recitation you’ll likely finish **part** of it. Continue after recitation until you finish.

---

## What you should understand (checkoff)

<div class="slide">
<div class="col text-xxs">

From the lecture slides:

- What React is (components)
- Node.js + npm + Vite basics
- JSX basics (`className`, `{}` expressions)
- Function components + props
- State with `useState`
- DOM vs “virtual DOM” (high level)
- Events (click handlers)

</div>
<div class="col text-xxs">

From the tutorial (tic-tac-toe):

- `Square` + `Board` components
- Passing data through props
- Click handling → state update → UI changes
- Lifting state up
- Immutability (copy arrays)
- Move history + time travel
- Keys for lists

</div>
</div>

---

## Cycle 1 — Work (10 minutes)

Goal: get a working dev setup + basic click interactivity.

Do:
1. Create a Vite React app, or download the `tictactoe-sandbox.zip`
2. Run the dev server.
3. Start the tutorial: build `Square` and `Board`.
4. Clicking a square change what you see (any “state change” is a win).

---

Checkpoint:
- `npm run dev` works
- clicking causes UI to update

---

## Cycle 1 — Reflection (5 minutes, Google Doc)

Write:
1. What tutorial section(s) did you finish?  
2. What was the most confusing line of code so far?  
3. In your own words: what causes a component to re-render?  
4. One specific next action for Cycle 2.

---

## Cycle 2 — Work (10 minutes)

Goal: make the game logic “real” (state + turns + lifting state up).

Do:
- Move state to the right place (lifting  estate up).
- Make turns alternate (X then O).
- Keep updates immutable (copy arrays instead of mutating).

---

Checkpoint:
- you can explain how to manage UI state (why immutable state)
- you can explain where state lives
- X/O turns work

---

## Cycle 2 — Reflection (5 minutes, Google Doc)

Write:
1. Where is your game state stored right now (which component)?  
2. One example of an immutable update you did (copying array/object).  
3. One bug you hit + how you debugged it (1–2 bullets).  
4. One specific next action for Cycle 3.

---

## Cycle 3 — Work (10 minutes)

Goal: history + time travel (even if it’s not polished).

Do:
- Implement move history.
- Render a list of moves (add `key`).
- Jump back to an earlier move (time travel).

Checkpoint:
- you can jump to an earlier move

---

## Cycle 3 — Reflection + Submit (5 minutes)

In Google Doc:
1. What part of the tutorial is still unfinished for you?  
2. One React concept you understand better now.  
3. One action you’ll take after recitation to finish.

Then submit: [https://forms.gle/tYEtKjJunM1wb2we6](https://forms.gle/tYEtKjJunM1wb2we6)

---

## If you’re stuck

Try one of these (in order):
1. Re-read the last tutorial step and confirm your file matches the expected code.
2. Open DevTools Console and read the error *top to bottom*.
3. Add a `console.log(...)` right before/after the line you think is wrong.
4. Ask the TA with: the error message + what you tried + what you expected.

<style>
.slide{
    display: flex;
}
.col{
    flex: 1;
}

.text-xxs { font-size: 0.625rem; }
</style>

