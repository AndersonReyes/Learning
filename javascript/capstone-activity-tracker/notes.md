# Capstone: Activity & Route Tracker

> Status: **planned** — this is the design doc, written before any code.
> Implementation follows the build phases below, one at a time, each ending
> in a runnable/verifiable state (same checkpoint discipline as the rest of
> this repo).

## Overview

An Apple Health / Strava-style activity tracker:

- Log ANY activity: type, name, date, duration, notes (no GPS required).
- For GPS-based activities (hike, mountain bike, run, ...), import a **GPX
  file** or paste raw **coordinates** (`lat,lon[,ele]` per line) to attach a
  route.
- Routes get: total distance, elevation gain/loss, an **interactive map**
  with the route traced (Leaflet), and an **elevation profile chart** (D3).
- Dashboard lists all activities with summary stats; clicking one opens its
  detail view (map + chart + stats).

## Architecture

```
┌─────────────────────────┐        ┌──────────────────────────┐
│  public/ (static files)  │  HTTP  │  server/ (node:http)      │
│  index.html  (dashboard)  │◄──────►│  index.js  (router)       │
│  activity.html (detail)   │  JSON  │  db.js     (node:sqlite)  │
│  js/*.js  (fetch + DOM)    │        │  gpx.js    (GPX parsing)  │
│  Leaflet + D3 via CDN      │        │  geo.js    (distance/ele) │
└─────────────────────────┘        └──────────────────────────┘
                                              │
                                       activities.db (SQLite file)
```

- Zero npm dependencies on the backend: `node:http`, `node:sqlite`,
  `node:fs`, `node:path`.
- Frontend loads Leaflet + D3 from CDN `<script>` tags — no bundler, no npm,
  consistent with how `html/`/`css/` examples are opened directly.
- Single Node process serves both the static `public/` files and the
  `/api/*` JSON routes.

## Data model

**`activities` table**

| column | type | notes |
|---|---|---|
| `id` | INTEGER PK | autoincrement |
| `type` | TEXT | `"run" \| "walk" \| "hike" \| "bike" \| "mountain_bike" \| "swim" \| "other"` |
| `name` | TEXT | optional, user-given title |
| `date` | TEXT | ISO 8601 start time |
| `duration_s` | INTEGER | seconds |
| `distance_m` | REAL \| NULL | `NULL` if no route |
| `elevation_gain_m` | REAL \| NULL | `NULL` if no route or no elevation data |
| `elevation_loss_m` | REAL \| NULL | `NULL` if no route or no elevation data |
| `notes` | TEXT \| NULL | |
| `created_at` | TEXT | ISO 8601, set by server |

**`route_points` table** (only rows for activities with a route)

| column | type | notes |
|---|---|---|
| `activity_id` | INTEGER FK | references `activities.id` |
| `seq` | INTEGER | 0-based order along the route |
| `lat` | REAL | |
| `lon` | REAL | |
| `ele` | REAL \| NULL | meters; `NULL` if not in source data |
| `time` | TEXT \| NULL | ISO 8601; `NULL` if not in source data |

`PRIMARY KEY (activity_id, seq)`, `ON DELETE CASCADE` from `activities`.

## GPX format & parsing

GPX is XML. The only shape that matters here:

```xml
<gpx>
  <trk>
    <trkseg>
      <trkpt lat="46.5763" lon="7.9904">
        <ele>1523.4</ele>
        <time>2024-06-01T08:15:00Z</time>
      </trkpt>
      <trkpt lat="46.5765" lon="7.9908">
        <ele>1531.2</ele>
        <time>2024-06-01T08:15:30Z</time>
      </trkpt>
      ...
    </trkseg>
  </trk>
</gpx>
```

**Parsing approach** (`server/gpx.js`, no XML library):

1. Match every `<trkpt ... >...</trkpt>` block:
   `/<trkpt\s+lat="([^"]+)"\s+lon="([^"]+)"[^>]*>([\s\S]*?)<\/trkpt>/g`
   (attribute order can vary — a robust parser extracts `lat`/`lon` by name,
   not position).
2. Within each block's inner content, optionally match
   `<ele>([\d.-]+)<\/ele>` and `<time>([^<]+)<\/time>`.
3. Produce `{ lat: number, lon: number, ele: number | null, time: string | null }[]`,
   in document order (`seq` = array index).

**Plain-coordinate input** (alternative to GPX, for users without a file):
one `lat,lon` or `lat,lon,ele` per line — same output shape, `time: null`.

**Edge cases to handle**: no `<ele>`/`<time>` on some/all points (→ `null`,
not `0`/`""` — `0` is a valid elevation), empty GPX (zero `trkpt`s → empty
route, `distance_m`/`elevation_*` become `null`, not `0`), malformed lines in
coordinate input (skip or reject the whole input — pick one, document it).

## Geo calculations (`server/geo.js`)

**Haversine distance** between two `{lat, lon}` points, in meters
(`R = 6371000`, mean Earth radius):

```
a = sin²(Δlat/2) + cos(lat1)·cos(lat2)·sin²(Δlon/2)
c = 2·atan2(√a, √(1−a))
distance = R·c
```

(`Δlat`, `Δlon`, `lat1`, `lat2` all in radians.)

**Hand-verified sanity check**: two points 0.001° apart in latitude
(~111m/degree) → distance ≈ 111m. Use this for the `node:test` spec.

**Total distance**: sum of `haversineDistance(points[i], points[i+1])` for
all consecutive pairs. Empty or single-point route → `0`.

**Cumulative distances**: running total at each point —
`[0, d(0,1), d(0,1)+d(1,2), ...]`. Used as the D3 elevation chart's x-axis.

**Elevation gain/loss**: walk consecutive `ele` values; sum positive deltas
into `gain`, sum `|negative deltas|` into `loss`. If ANY point has `ele ===
null`, both are `null` (can't compute a meaningful gain/loss from partial
data). **Gotcha**: raw GPS elevation is noisy — every topic's `notes.md` says
"hand-verify before writing the spec," so pick a small, clean sample (e.g.
monotonic ascent) for the test fixture rather than real noisy GPS data.

## API spec

```
GET    /api/activities
  -> [{ id, type, name, date, duration_s, distance_m,
        elevation_gain_m, elevation_loss_m, has_route }]

GET    /api/activities/:id
  -> { ...activity, route: [{ lat, lon, ele, time }, ...] | null }

POST   /api/activities
  body: {
    type, name, date, duration_s, notes,
    gpx?: string,                          // raw GPX file contents
    coordinates?: string,                  // raw "lat,lon,ele" lines
  }
  -> 201 { id }
  - Validates body shape via `validate(schema, value)` (advanced/11 pattern).
  - If `gpx` or `coordinates` present: parse -> route points -> compute
    distance_m / elevation_gain_m / elevation_loss_m -> store route_points.
  - Exactly one of `gpx`/`coordinates` may be set (400 if both).

DELETE /api/activities/:id
  -> 204 (cascades to route_points)
```

Errors: `400` with `{ error: string }` for validation failures (bad schema,
unparseable GPX, both/neither of `gpx`/`coordinates`), `404` for unknown
`:id`.

## Frontend

**`public/index.html`** — dashboard:
- Table/list of activities: date, type, name, duration, distance, elevation
  gain. Sorted by date desc.
- "+ Log Activity" form: type (`<select>`), name, date, duration, notes,
  plus an optional GPX file `<input type="file">` (read via `FileReader`,
  send `.text()` as `gpx` in the POST body) OR a `<textarea>` for pasted
  coordinates.

**`public/activity.html?id=N`** — detail view:
- Stats summary: distance (km), duration (h:mm:ss), pace/speed
  (computed client-side from `distance_m`/`duration_s`), elevation gain/loss.
- If `route` is non-null:
  - **Leaflet map**: OSM tile layer, `L.polyline(route.map(p => [p.lat,
    p.lon]))`, `map.fitBounds(polyline.getBounds())`.
  - **D3 elevation profile**: SVG line chart, x = cumulative distance (km),
    y = elevation (m). Skip this chart entirely if any `ele` is `null`.

## File structure

```
javascript/capstone-activity-tracker/
  notes.md              <- this file
  README.md             <- how to run, screenshots
  package.json          "type": "module"; scripts: start, test
  server/
    index.js            http server + routing
    db.js                node:sqlite schema + queries
    gpx.js               GPX/coordinate parsing
    geo.js               distance/elevation calculations
    geo.test.js
    gpx.test.js
    db.test.js
  public/
    index.html
    activity.html
    css/style.css
    js/
      api.js             fetch wrapper
      dashboard.js
      activity.js
  sample-data/
    sample-hike.gpx      small, hand-verifiable fixture
```

## Build phases (one checkpoint per phase)

1. **`geo.js`** — haversine/total distance/cumulative distances/elevation
   gain-loss, as pure functions with a `node:test` spec (hand-verified
   values, exercise-style).
2. **`gpx.js`** — GPX + coordinate parsing, `node:test` spec against
   `sample-data/sample-hike.gpx` and hand-written edge cases (missing
   ele/time, empty file).
3. **`db.js`** — `node:sqlite` schema + CRUD, `node:test` against an
   in-memory (`:memory:`) database.
4. **`server/index.js`** — HTTP routing wiring `db`/`gpx`/`geo` together;
   verify with `curl` against all 4 endpoints.
5. **Dashboard UI** — activity list + manual logging form (no map yet);
   verify by running the server and using the form in a browser.
6. **Map + elevation chart** — Leaflet route polyline + D3 elevation profile
   on the detail page; verify visually with `sample-data/sample-hike.gpx`.
7. **Polish** — pace/speed display, activity-type icons/filtering on the
   dashboard, responsive CSS.

## Open questions / deferred decisions

- Multi-user / auth: out of scope (single-user, local app).
- Editing an existing activity: not in v1 (delete + re-create instead).
- Elevation-noise smoothing (e.g. ignore sub-1m deltas): note as a possible
  refinement after the basic gain/loss calc is verified against the sample
  fixture.
