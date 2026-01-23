## Week 2: Responsive design

Slides: `slides/01_responsive.pdf`  
Time: **50 minutes** (Tune In **5** / Activity **35** / Reflection **10**)

Get this instruction: https://github.com/ruhendrawan/cs1520_recitation

---

## How to work today

- Work in pairs or alone
  - If in pairs: work with your peer on the first and last part
  - If alone: self reflect and write down on the worksheet
- You do not need a grammar check
- Don’t overcorrect
- Write like you’re saying it out loud

---

## Reflection and Submission

Submit in the google form: https://forms.gle/tYEtKjJunM1wb2we6

You need to submit the zip file if you're working alone, otherwise just fill in the form.

Upload zip files named as `<PITT ID>-<week>.zip`, e.g. `rah225-02.zip`, containing:

- This worksheet
- `page.html` (or any files related to your work)

---

## Part 1 — Tune In (5 minutes)

Pairs: talk in turns. Max **1 minute** per person before switching so everyone gets a chance to talk.

Working alone? Download the worksheet here and write your responses there:
https://github.com/ruhendrawan/cs1520_recitation/blob/main/recitations/02-responsive-design-worksheet.md

Pick **1–3 concepts** from this week’s lecture and explain them in your own words (fast + messy is fine).

Concept ideas (pick any 1–3):

- Visual viewport vs layout viewport
- CSS pixels (when a pixel isn’t a pixel)
- The `<meta name="viewport" ...>` tag (why it exists, what it changes)
- CSS media queries (`min-width`, `max-width`, `orientation`)
- “Use relative sizes” + “breakpoints” (mobile-first thinking)

---

## Part 1 — Starter questions

- What is it (in one sentence)?
- Why do we need it on phones vs laptops?
- What problem does it solve?
- What would break if we *didn’t* use it?
- Where would you use it in a real app/site you care about?

---

## Part 2 — Activity (35 minutes)

Do **one** of the three tracks below (or split into groups and compare results at the end).

- Track A — Follow-along (guided)
- Track B — Challenge
- Track C — Choose your own

---

## Track A — Follow along and build a page

Choose **Option 1** or **Option 2**.

Option 1: Beginner in HTML CSS

1. [Your first website](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Your_first_website)
2. [Creating the content](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Your_first_website/Creating_the_content)
3. [Styling the content](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Your_first_website/Styling_the_content)


- [Finding color, images, and fonts](https://developer.mozilla.org/en-US/docs/Learn_web_development/Getting_started/Your_first_website/What_will_your_website_look_like)

Option 2: Understand the basics of HTML and CSS

1. [Responsive design (MDN Learn)](https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/CSS_layout/Responsive_Design)
2. [Media queries (MDN Learn)](https://developer.mozilla.org/en-US/docs/Learn_web_development/Core/CSS_layout/Media_queries)

---

## Track A — Checkpoints

- You built a page (HTML + CSS) and can show it in the browser
- You changed something and saw the browser update (small win)
- If you did Option 2: you can explain one media query you used (in plain language)

---

## Track B — Challenge (pick one)

1) Responsive “student dashboard”
2) No-scroll mobile page
3) Breakpoint story

---

## Track B — 1) Responsive “student dashboard”

- Goal: Build a single-page dashboard layout that feels good on phone + desktop
- Constraints:
  - Use at least **2 breakpoints**
  - Use at least **one relative unit** (%, `rem`, `vw`, etc.)
  - Navigation changes layout across breakpoints
- Stretch:
  - Add `orientation` media query to improve landscape mode

---

## Track B — 2) No-scroll mobile page

- Goal: Make a landing page that never causes horizontal scrolling on mobile
- Constraints:
  - Test at very narrow width (like 320px)
  - Images/cards scale down properly
  - No fixed-width containers that force overflow
- Stretch:
  - Add centered desktop layout with `max-width` + auto margins

---

## Track B — 3) Breakpoint story

- Goal: Layout changes “story” at different widths (grid → sidebar, etc.)
- Constraints:
  - 3 layouts total: small / medium / large
  - Each layout changes content emphasis
  - Keep HTML the same; only CSS changes
- Stretch:
  - Explain why your breakpoints are where they are (2–3 sentences)

---

## Track C — Choose your own

Use this week’s topics: **relative sizes, breakpoints, viewport/meta viewport, and media queries**.

Examples:
- Personal homepage / portfolio grid
- Resume/CV page
- Study tracker (today/this week/done)
- Budget tracker dashboard
- Event flyer page
- “Link hub” page for online presence

Definition of done:
- Viewport meta tag used
- 2+ media queries
- Relative sizing used (%, `rem`, `max-width`, etc.)
- Readable on narrow window (no horizontal scrolling)
- You can explain what changes at each breakpoint and why

---

## Part 3 — Reflection (10 minutes)

Pairs: talk in turns. Max **1 minute** per person before switching so everyone gets a chance to talk.

Working alone? Download the worksheet here and write your responses there:
https://github.com/ruhendrawan/cs1520_recitation/blob/main/recitations/02-responsive-design-worksheet.md

### A) What you did
- Quick checklist is fine, no overcorrection needed

### B) Connect it back (prior lecture)
Prior lecture idea to connect: **HTML metadata + the `<meta>` tag**

- What is an HTML `<meta>` tag for?
- How does the viewport meta tag change what the browser does on mobile?
- How did it affect your work today?

---

## If you’re stuck

- Which screen size are you targeting first (smallest)?
- Are you using any fixed widths that force overflow?
- Did you include the viewport meta tag?
- What is each breakpoint trying to improve (readability, spacing, layout)?
