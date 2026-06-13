# 03. Responsive and Modern CSS

Making layouts adapt to viewport size, user preferences, and adding motion —
plus a few modern CSS features that reduce the need for media queries and
JS-based theming.

## Media queries (mobile-first)

```css
/* base styles = mobile (no media query, applies to ALL sizes) */
.container {
  display: block;
  padding: 8px;
}

/* min-width: applies at this width AND ABOVE — "mobile-first" */
@media (min-width: 768px) {
  .container {
    display: flex;
    padding: 24px;
  }
}

@media (min-width: 1200px) {
  .container {
    max-width: 1140px;
    margin: 0 auto;
  }
}
```

- **Mobile-first**: write base (unprefixed) styles for the smallest screens,
  then use `min-width` queries to progressively enhance for larger screens.
  This is preferred over `max-width` "desktop-first" because most sites have
  more mobile traffic and it avoids overriding desktop styles repeatedly.
- Common breakpoints (not official, just conventions): `480px` (large phone),
  `768px` (tablet), `1024px`/`1200px` (desktop).
- Other useful media features:
  ```css
  @media (orientation: landscape) { }
  @media (prefers-color-scheme: dark) { }   /* user's OS theme */
  @media (prefers-reduced-motion: reduce) { } /* user disabled animations */
  ```

See [`examples/01-media-queries.html`](./examples/01-media-queries.html).

## Responsive units recap

| Unit | Use for |
|---|---|
| `%` | widths relative to parent |
| `vw` / `vh` | full-viewport sections, hero banners |
| `rem` | font sizes, spacing (scales with user's root font-size) |
| `ch` | width based on character count (e.g. `max-width: 60ch` for readable line length) |

## CSS custom properties (variables)

```css
:root {
  --primary-color: #2266cc;
  --spacing-unit: 8px;
  --max-width: 1200px;
}

.button {
  background-color: var(--primary-color);
  padding: calc(var(--spacing-unit) * 2);
}

/* fallback if the variable isn't defined */
.fallback-demo {
  color: var(--undefined-var, black);
}
```

- Declared with `--name: value`, read with `var(--name)` (optionally with a
  fallback as the second argument).
- `:root` = the `<html>` element — declaring variables there makes them
  available everywhere (custom properties **inherit** down the DOM tree).
- Can be **overridden in any scope** — redeclaring `--primary-color` inside
  `.dark-theme { --primary-color: #88aaff; }` changes it for everything
  inside `.dark-theme`, without rewriting every rule that uses it.
- Combine with `prefers-color-scheme` for dark mode without JS:
  ```css
  :root {
    --bg: white;
    --fg: black;
  }
  @media (prefers-color-scheme: dark) {
    :root {
      --bg: #1a1a1a;
      --fg: #eee;
    }
  }
  body {
    background-color: var(--bg);
    color: var(--fg);
  }
  ```
- Unlike Sass/Less variables (compile-time, static), custom properties are
  **live in the browser** — `calc()` and `var()` re-evaluate when the
  variable's value changes (e.g. via JS `element.style.setProperty` or a
  different media query matching).

See [`examples/02-custom-properties.html`](./examples/02-custom-properties.html).

## clamp(), min(), max()

```css
h1 {
  /* clamp(MIN, PREFERRED, MAX) — responsive font size with hard limits,
     no media query needed */
  font-size: clamp(1.5rem, 4vw + 1rem, 3rem);
}

.container {
  /* never wider than 1200px, but shrink on small viewports */
  width: min(100% - 32px, 1200px);
}

.sidebar {
  /* at least 200px, but allow growing with the viewport */
  width: max(200px, 15vw);
}
```

- `clamp(min, preferred, max)`: uses `preferred` unless it falls outside
  `[min, max]`, in which case it's clamped to the nearest bound. The
  `preferred` value is often a `vw`-based expression so it scales
  continuously with viewport width between the two bounds.
- `min()`/`max()` pick the smaller/larger of a comma-separated list of
  values — useful for "whichever constraint is tighter" sizing.
- All three accept mixed units (`px`, `%`, `vw`, `rem`, etc.) and can replace
  many `@media`-query-based size adjustments.

## aspect-ratio

```css
.video-container {
  aspect-ratio: 16 / 9; /* height is computed from width to maintain the ratio */
  width: 100%;
}

img {
  aspect-ratio: 1 / 1; /* reserves a square box before the image loads */
  width: 100%;
  object-fit: cover;   /* crop to fill the box without distortion */
}
```

- Reserves layout space for media (video embeds, images) before content
  loads, preventing layout shift — without the old "padding-top percentage
  hack."

See [`examples/03-modern-functions.html`](./examples/03-modern-functions.html).

## Transitions

```css
.button {
  background-color: #2266cc;
  transition: background-color 0.2s ease-in-out, transform 0.2s ease-out;
}
.button:hover {
  background-color: #1a4d8f;
  transform: translateY(-2px);
}
```

- `transition: <property> <duration> <timing-function> <delay>;` — animates a
  property's change between two states (e.g. normal → `:hover`/`:focus`).
- List multiple properties comma-separated, each with its own timing.
- `transition: all 0.2s;` is convenient but animates EVERY changing property
  (including ones you didn't intend, like layout-affecting properties) —
  prefer naming specific properties.
- Common timing functions: `ease`, `ease-in`, `ease-out`, `ease-in-out`,
  `linear`, or a custom `cubic-bezier(...)`.

## Animations (@keyframes)

```css
@keyframes spin {
  from { transform: rotate(0deg); }
  to   { transform: rotate(360deg); }
}

@keyframes pulse {
  0%   { opacity: 1; }
  50%  { opacity: 0.4; }
  100% { opacity: 1; }
}

.spinner {
  animation: spin 1s linear infinite;
}
```

- `@keyframes` defines named animation steps (`from`/`to` or `0%`-`100%`
  percentages); `animation: <name> <duration> <timing> <iteration-count>`
  applies it.
- `animation-iteration-count: infinite` loops forever; a number runs that
  many times.
- Respect user preferences — wrap non-essential motion:
  ```css
  @media (prefers-reduced-motion: reduce) {
    * {
      animation-duration: 0.01ms !important;
      transition-duration: 0.01ms !important;
    }
  }
  ```

See [`examples/04-transitions-and-animations.html`](./examples/04-transitions-and-animations.html).

## Further Reading (MDN)

- [Responsive design](https://developer.mozilla.org/en-US/docs/Learn/CSS/CSS_layout/Responsive_Design)
- [Using media queries](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_media_queries/Using_media_queries)
- [Using CSS custom properties](https://developer.mozilla.org/en-US/docs/Web/CSS/Using_CSS_custom_properties)
- [`clamp()`](https://developer.mozilla.org/en-US/docs/Web/CSS/clamp), [`min()`](https://developer.mozilla.org/en-US/docs/Web/CSS/min), [`max()`](https://developer.mozilla.org/en-US/docs/Web/CSS/max)
- [`aspect-ratio`](https://developer.mozilla.org/en-US/docs/Web/CSS/aspect-ratio)
- [Using CSS transitions](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_transitions/Using_CSS_transitions)
- [Using CSS animations](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_animations/Using_CSS_animations)
- [`prefers-color-scheme`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-color-scheme), [`prefers-reduced-motion`](https://developer.mozilla.org/en-US/docs/Web/CSS/@media/prefers-reduced-motion)
