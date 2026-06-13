# 02. Flexbox and Grid

Two modern layout systems. **Flexbox** is one-dimensional (a row OR a
column); **Grid** is two-dimensional (rows AND columns at once). Both
replace older float/table-based layout hacks.

## Flexbox basics

```css
.container {
  display: flex;            /* children become flex items */
  flex-direction: row;      /* row (default) | row-reverse | column | column-reverse */
}
```

- **Main axis**: the direction set by `flex-direction` (`row` = horizontal).
- **Cross axis**: perpendicular to the main axis (`row` → vertical).
- All direct children of a `display: flex` container become flex items —
  they lay out along the main axis by default, shrinking/growing as needed.

See [`examples/01-flexbox-basics.html`](./examples/01-flexbox-basics.html).

## Aligning flex items

```css
.container {
  display: flex;
  justify-content: flex-start; /* main-axis alignment:
    flex-start | flex-end | center | space-between | space-around | space-evenly */
  align-items: stretch;        /* cross-axis alignment (single line):
    stretch | flex-start | flex-end | center | baseline */
  align-content: flex-start;   /* cross-axis alignment of WRAPPED LINES as a group
    (only matters with flex-wrap: wrap and multiple lines) */
}
```

- `justify-content` distributes space along the main axis (between/around
  items).
- `align-items` aligns items along the cross axis WITHIN their line.
- `align-content` aligns the lines themselves when there's extra cross-axis
  space and `flex-wrap: wrap` is set.
- A single flex item can override `align-items` for itself with
  `align-self: center` etc.

## Wrapping

```css
.container {
  display: flex;
  flex-wrap: nowrap;  /* default: items shrink to fit on one line, may overflow */
  flex-wrap: wrap;    /* items wrap onto new lines when they don't fit */
}
```

## Sizing flex items: grow, shrink, basis

```css
.item {
  flex-grow: 0;     /* default 0: don't grow to absorb extra space */
  flex-shrink: 1;   /* default 1: allow shrinking below basis if needed */
  flex-basis: auto; /* default auto: starting size = content size (or width/height) */

  /* shorthand: flex: <grow> <shrink> <basis>; */
  flex: 1;          /* = flex: 1 1 0%  — common "fill remaining space equally" pattern */
  flex: 0 0 200px;  /* fixed 200px, never grow or shrink — common "sidebar" pattern */
}
```

- `flex-grow: 1` on multiple items makes them share extra space **equally**;
  `flex-grow: 2` on one item makes it take twice the extra space of a
  sibling with `flex-grow: 1` (proportions of the LEFTOVER space, not the
  total).
- `flex-shrink: 0` prevents an item from shrinking below its `flex-basis`
  even if the container is too small — useful for icons/avatars that
  shouldn't squish.
- `flex-basis: 0` + `flex-grow: 1` on all items → items end up equal width
  regardless of content (content size ignored as the starting point).

## Gap and order

```css
.container {
  display: flex;
  gap: 16px;        /* space BETWEEN items (not on the outer edges) — also works in Grid */
}
.item-first {
  order: -1;        /* default order is 0; lower values render first, visually
                        reordering WITHOUT changing the underlying HTML/DOM order
                        (the DOM/tab order is unchanged — accessibility caveat) */
}
```

See [`examples/02-flexbox-layout.html`](./examples/02-flexbox-layout.html)
for a navbar + card layout combining `flex-grow`/`flex-shrink`/`flex-basis`,
`gap`, and `order`.

## Grid basics

```css
.container {
  display: grid;
  grid-template-columns: 200px 1fr 1fr; /* 3 columns: fixed 200px, then two equal flexible columns */
  grid-template-rows: auto 1fr auto;    /* 3 rows: size-to-content, fill remaining, size-to-content */
  gap: 16px;                             /* row and column gap */
}
```

- **`fr`** = a fraction of the remaining space after fixed-size tracks are
  subtracted — `1fr 1fr` splits remaining space 50/50; `1fr 2fr` splits it
  1:2.
- **`repeat(n, size)`** avoids repetition: `repeat(3, 1fr)` = `1fr 1fr 1fr`.
- Children are placed into the grid automatically, row by row, unless
  explicitly positioned.

See [`examples/03-grid-basics.html`](./examples/03-grid-basics.html).

## Placing items: grid-area & named template areas

```css
.container {
  display: grid;
  grid-template-columns: 200px 1fr;
  grid-template-rows: auto 1fr auto;
  grid-template-areas:
    "sidebar header"
    "sidebar main"
    "sidebar footer";
}
.sidebar { grid-area: sidebar; }
.header  { grid-area: header; }
.main    { grid-area: main; }
.footer  { grid-area: footer; }
```

- `grid-template-areas` lays out a named "map" of the grid — each named
  region must form a rectangle. Children opt into a region via
  `grid-area: <name>`.
- Alternative: position items by line numbers —
  `grid-column: 1 / 3;` (span from column line 1 to line 3, i.e. 2 columns),
  `grid-row: 2 / span 2;` (start at row line 2, span 2 rows).

## Responsive grids: auto-fit / auto-fill + minmax()

```css
.cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}
```

- `minmax(200px, 1fr)`: each column is at least 200px, growing to fill
  available space (up to `1fr`).
- `repeat(auto-fit, ...)`: as many columns as fit; existing columns
  **stretch** to fill leftover space (collapses empty tracks).
- `repeat(auto-fill, ...)`: as many columns as fit, but leaves **empty**
  tracks instead of stretching existing items into the leftover space.
- This single rule replaces a media-query-per-breakpoint approach for
  card/grid layouts — the browser recalculates the column count on resize.

See [`examples/04-grid-layout.html`](./examples/04-grid-layout.html).

## Flexbox vs Grid — when to use which

- **Flexbox**: one-dimensional flow (navbars, button groups, lists of cards
  that wrap in one direction, centering a single item).
- **Grid**: two-dimensional layout (overall page structure with header/
  sidebar/main/footer, photo galleries, dashboards) — especially when rows
  AND columns both need to align.
- They compose: a Grid layout's cells often contain Flexbox containers for
  their internal content.

## Further Reading (MDN)

- [Flexbox](https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Flexbox)
- [`flex` (shorthand)](https://developer.mozilla.org/en-US/docs/Web/CSS/flex)
- [Grids](https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Grids)
- [`grid-template-areas`](https://developer.mozilla.org/en-US/docs/Web/CSS/grid-template-areas)
- [`minmax()`](https://developer.mozilla.org/en-US/docs/Web/CSS/minmax)
- [`repeat()`](https://developer.mozilla.org/en-US/docs/Web/CSS/repeat)
