# 01. CSS Fundamentals

CSS describes **presentation** — how HTML content looks. Each example below
is a standalone `.html` file (one pairs with a `.css` file) — open it
directly in a browser to see it rendered.

## Linking CSS

```html
<!-- inline: one element, highest practical specificity, avoid for anything reusable -->
<p style="color: red;">Inline styled</p>

<!-- internal: <style> in <head>, fine for small demos/single pages -->
<style>
  p { color: red; }
</style>

<!-- external: <link>, the standard approach — cacheable, reusable across pages -->
<link rel="stylesheet" href="styles.css" />
```

See [`examples/04-linked-stylesheet.html`](./examples/04-linked-stylesheet.html)
+ [`examples/04-linked-stylesheet.css`](./examples/04-linked-stylesheet.css).

## Selectors

```css
/* type — matches all <p> elements */
p { }

/* class — matches elements with class="highlight" */
.highlight { }

/* id — matches the element with id="main" (must be unique per page) */
#main { }

/* attribute */
input[type="email"] { }
a[target="_blank"] { }

/* pseudo-class — state/position based */
a:hover { }
li:first-child { }
li:nth-child(2n) { }  /* even items */
input:focus { }
input:disabled { }

/* pseudo-element — targets a generated sub-part */
p::first-line { }
p::before { content: "» "; }

/* combinators */
section p     { }  /* descendant: any <p> inside <section>, any depth */
section > p   { }  /* child: <p> that is a DIRECT child of <section> */
h2 + p        { }  /* adjacent sibling: <p> immediately after an <h2> */
h2 ~ p        { }  /* general sibling: any <p> after an <h2>, same parent */

/* grouping — comma-separated selectors share the same rule */
h1, h2, h3 { font-family: sans-serif; }
```

See [`examples/01-selectors-and-cascade.html`](./examples/01-selectors-and-cascade.html).

## The cascade & specificity

When multiple rules target the same element, the **most specific** wins.
Specificity is calculated as a tuple `(inline, ids, classes/attrs/pseudo-classes, types/pseudo-elements)`,
compared left to right:

| Selector | Specificity |
|---|---|
| `style="..."` (inline) | (1, 0, 0, 0) — always wins over stylesheet rules |
| `#main` | (0, 1, 0, 0) |
| `.highlight` | (0, 0, 1, 0) |
| `input[type="email"]` | (0, 0, 1, 0) — attribute = class weight |
| `a:hover` | (0, 0, 1, 0) — pseudo-class = class weight |
| `p` | (0, 0, 0, 1) |
| `p::before` | (0, 0, 0, 2) — pseudo-element adds another type-weight |

- Higher specificity wins regardless of source order.
- **Equal specificity** → last rule in source order wins.
- `!important` overrides normal specificity entirely — avoid it; it makes
  future overrides need their own `!important`, escalating.
- `*` (universal selector) has specificity (0,0,0,0) — loses to everything.

## The box model

Every element is a box made of four nested regions, outside-in:

```
margin (outside the box, transparent, can collapse between siblings)
  border (visible edge)
    padding (space between border and content)
      content (text/children, sized by width/height)
```

```css
.box {
  width: 200px;
  height: 100px;
  padding: 16px;
  border: 2px solid black;
  margin: 8px;
}
```

- **`box-sizing: content-box`** (default) — `width`/`height` apply to
  **content only**; padding and border are ADDED on top, so the box's
  rendered size = `width + padding*2 + border*2`.
- **`box-sizing: border-box`** — `width`/`height` include padding and
  border. The box's rendered size = exactly `width`/`height`. Much easier to
  reason about — commonly set globally:
  ```css
  *, *::before, *::after { box-sizing: border-box; }
  ```
- **Margin collapsing**: vertical margins between adjacent block siblings
  collapse to the LARGER of the two (not the sum). Margins don't collapse
  for flex/grid children.
- `margin: auto` on a block element with a set `width` centers it
  horizontally.

See [`examples/02-box-model.html`](./examples/02-box-model.html).

## Units

| Unit | Relative to | Notes |
|---|---|---|
| `px` | absolute (CSS pixel) | predictable, doesn't scale with user font settings |
| `%` | parent element's corresponding dimension | `width: 50%` = half the parent's width |
| `em` | the element's OWN `font-size` (for `font-size` itself, relative to PARENT's) | **compounds** when nested — `1.2em` inside `1.2em` inside `1.2em` = `1.728em` of the root |
| `rem` | the ROOT (`<html>`) element's `font-size` | does NOT compound — predictable, preferred for spacing/typography |
| `vw` / `vh` | 1% of viewport width/height | useful for full-screen sections; `100vh` can exceed visible area on mobile (browser UI) |
| `vmin` / `vmax` | smaller/larger of `vw`/`vh` | |

- Prefer `rem` for font sizes and spacing (predictable, respects user's
  browser font-size setting for accessibility); `px` for borders/shadows
  where sub-pixel precision doesn't matter; `%`/`vw`/`vh` for
  layout-relative sizing.

## Typography & color

```css
body {
  font-family: "Helvetica Neue", Arial, sans-serif; /* fallback stack */
  font-size: 1rem;       /* 16px by default */
  font-weight: 400;      /* 400 = normal, 700 = bold */
  line-height: 1.5;      /* unitless = multiplier of font-size, preferred */
  text-align: left;      /* left | right | center | justify */
}

h1 {
  color: #1a1a1a;             /* hex */
  color: rgb(26 26 26);       /* rgb (modern space-separated syntax) */
  color: hsl(0 0% 10%);       /* hue, saturation, lightness */
  color: rgb(26 26 26 / 0.5); /* alpha channel */
  background-color: white;
}
```

- `color` sets text color; `background-color` sets the box's fill.
- `font-family` should always end in a generic fallback (`sans-serif`,
  `serif`, `monospace`) in case all named fonts are unavailable.
- Unitless `line-height` (e.g. `1.5`) scales with the element's own
  `font-size` — preferred over `line-height: 24px` which doesn't adapt.

See [`examples/03-typography-and-colors.html`](./examples/03-typography-and-colors.html).

## Display

```css
.block  { display: block; }       /* full width, stacks vertically (div, p, h1) */
.inline { display: inline; }      /* flows within text, ignores width/height (span, a) */
.inline-block { display: inline-block; } /* flows inline but respects width/height/margin */
.hidden { display: none; }        /* removed from layout entirely, not in accessibility tree */
.invisible { visibility: hidden; } /* invisible but still occupies its layout space */
```

- `display: none` vs `visibility: hidden`: the former removes the element
  from layout (surrounding elements shift to fill the gap); the latter
  leaves an invisible gap in its place.
- Flexbox (`display: flex`) and Grid (`display: grid`) are covered in
  [`02-flexbox-and-grid`](../02-flexbox-and-grid).

## Further Reading (MDN)

- [CSS first steps](https://developer.mozilla.org/en-US/docs/Learn/CSS/First_steps)
- [CSS selectors](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_selectors)
- [Cascade, specificity, and inheritance](https://developer.mozilla.org/en-US/docs/Web/CSS/Cascade)
- [The box model](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/The_box_model)
- [CSS values and units](https://developer.mozilla.org/en-US/docs/Learn/CSS/Building_blocks/Values_and_units)
- [`box-sizing`](https://developer.mozilla.org/en-US/docs/Web/CSS/box-sizing)
