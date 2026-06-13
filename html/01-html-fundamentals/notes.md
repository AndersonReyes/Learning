# 01. HTML Fundamentals

HTML describes the **structure and meaning** of content — not its appearance
(that's CSS) or behavior (that's JS). Each example below is a standalone
`.html` file — open it directly in a browser to see it rendered.

## Document structure

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Page Title</title>
  </head>
  <body>
    <!-- visible content -->
  </body>
</html>
```

- `<!DOCTYPE html>` — tells the browser to use standards mode (no quirks mode).
- `lang` on `<html>` — screen readers and translators rely on this.
- `<meta charset="UTF-8">` — should be the FIRST thing in `<head>` (so the
  browser doesn't mis-decode earlier bytes).
- `viewport` meta — without it, mobile browsers render at a fixed desktop
  width and zoom out (everything looks tiny).
- `<title>` — shown in the browser tab/bookmark, NOT rendered in the page body.

See [`examples/01-document-structure.html`](./examples/01-document-structure.html).

## Text content

```html
<h1>Page title</h1>
<h2>Section heading</h2>
<!-- h1-h6, used in order, one h1 per page (page's main heading) -->

<p>A paragraph of text.</p>

<strong>important (bold + semantic)</strong>
<em>emphasized (italic + semantic)</em>
<!-- prefer strong/em over <b>/<i> — they carry meaning, not just styling -->

<br />  <!-- line break, no closing content -->
<hr />  <!-- thematic break (horizontal rule) -->

<blockquote cite="https://example.com">
  <p>A quoted block of text.</p>
</blockquote>

<code>inline code</code>
<pre>preformatted text — preserves whitespace/line breaks</pre>
```

Heading levels communicate **document outline**, not font size — don't skip
levels (`h1` -> `h3`) just to get smaller text; use CSS for that.

See [`examples/02-text-and-semantic-elements.html`](./examples/02-text-and-semantic-elements.html).

## Semantic layout elements

Plain `<div>`/`<span>` carry no meaning — prefer semantic elements where one
fits, so assistive tech and search engines understand the page's regions:

```html
<header>  <!-- intro/nav for a page or section -->
<nav>     <!-- primary navigation links -->
<main>    <!-- the page's unique content (one per page) -->
<article> <!-- self-contained, independently distributable content -->
<section> <!-- thematic grouping, usually with its own heading -->
<aside>   <!-- tangential content (sidebar, pull quote) -->
<footer>  <!-- footer for a page or section -->
```

`<div>`/`<span>` are still correct for purely-presentational
grouping/styling hooks that have no semantic meaning of their own.

See [`examples/06-semantic-layout.html`](./examples/06-semantic-layout.html).

## Links and images

```html
<a href="https://example.com">absolute URL</a>
<a href="/about">root-relative (from domain root)</a>
<a href="about.html">relative (from current file's directory)</a>
<a href="#section-2">same-page anchor (id="section-2" target)</a>
<a href="https://example.com" target="_blank" rel="noopener noreferrer">
  opens in new tab
</a>
```

`target="_blank"` should be paired with `rel="noopener noreferrer"` — without
it, the opened page can access `window.opener` (security/perf issue).

```html
<img src="photo.jpg" alt="Description of the image" width="400" height="300" />

<figure>
  <img src="chart.png" alt="Quarterly revenue chart" />
  <figcaption>Fig 1. Revenue by quarter</figcaption>
</figure>
```

- `alt` is **required** — describes the image for screen readers and when the
  image fails to load. Empty `alt=""` is valid for purely decorative images
  (tells assistive tech to skip it).
- `width`/`height` attributes (even if CSS also sets size) let the browser
  reserve space before the image loads, preventing layout shift.

See [`examples/03-links-and-images.html`](./examples/03-links-and-images.html).

## Lists and tables

```html
<ul>            <!-- unordered list -->
  <li>Item</li>
</ul>

<ol>            <!-- ordered (numbered) list -->
  <li>First</li>
</ol>

<dl>            <!-- description list -->
  <dt>Term</dt>
  <dd>Definition</dd>
</dl>
```

```html
<table>
  <caption>Monthly sales</caption>
  <thead>
    <tr><th scope="col">Month</th><th scope="col">Total</th></tr>
  </thead>
  <tbody>
    <tr><td>Jan</td><td>100</td></tr>
  </tbody>
</table>
```

`<th scope="col">`/`scope="row"` associates header cells with the data cells
they describe — important for screen readers reading tables cell-by-cell.
Don't use `<table>` for visual page layout (that's CSS Grid/Flexbox — see the
`css/` track).

See [`examples/04-lists-and-tables.html`](./examples/04-lists-and-tables.html).

## Forms

```html
<form action="/submit" method="post">
  <label for="name">Name</label>
  <input type="text" id="name" name="name" required placeholder="Ada Lovelace" />

  <label for="email">Email</label>
  <input type="email" id="email" name="email" required />

  <fieldset>
    <legend>Preferred contact</legend>
    <label><input type="radio" name="contact" value="email" /> Email</label>
    <label><input type="radio" name="contact" value="phone" /> Phone</label>
  </fieldset>

  <label for="plan">Plan</label>
  <select id="plan" name="plan">
    <option value="free">Free</option>
    <option value="pro" selected>Pro</option>
  </select>

  <label for="bio">Bio</label>
  <textarea id="bio" name="bio" rows="3"></textarea>

  <button type="submit">Submit</button>
</form>
```

- `<label for="...">` must match the input's `id` — clicking the label
  focuses/activates the input, and screen readers announce the label when
  the input is focused. (Or nest the input inside the label, no `for` needed.)
- `required`, `placeholder`, `type="email"`/`type="number"`/etc. give free
  browser-side validation and the right mobile keyboard.
- `<fieldset>`/`<legend>` group related controls (e.g., a radio button set)
  with a group label.
- `method="post"` for state-changing submissions, `method="get"` for
  searches/filters (params end up in the URL).

See [`examples/05-forms.html`](./examples/05-forms.html).

## Further Reading (MDN)

- [HTML basics](https://developer.mozilla.org/en-US/docs/Learn/Getting_started_with_the_web/HTML_basics)
- [HTML elements reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element)
- [Document and website structure](https://developer.mozilla.org/en-US/docs/Learn/HTML/Introduction_to_HTML/Document_and_website_structure)
- [HTML forms](https://developer.mozilla.org/en-US/docs/Learn/Forms)
- [`<img>` reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
- [`<table>` reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/table)
