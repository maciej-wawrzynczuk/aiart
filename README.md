# aiart

Generates random SVG line art by sampling normalized float coordinates and mapping them to pixel canvas coordinates.

## Todo

### Unit tests

- [ ] **`f2canvas` maps 0.0 → 0 and 1.0 → canvas_size** — boundary values; ensures the full coordinate range maps correctly to pixel space without off-by-one errors.

- [ ] **`f2canvas` rounds correctly at midpoints** — e.g. `f2canvas(0.5_f32, 640_u16)` should return `320`; guards against truncation instead of rounding.

- [ ] **`SVGLine::from_line` produces valid SVG attribute string** — given a known `Line` (fixed coordinates), assert `format!("{}", svg_line)` contains the expected `x1`, `y1`, `x2`, `y2` pixel values.

- [ ] **`f2canvas` returns `Err` on out-of-range float** — pass `f32::INFINITY` or `f32::NAN`; the `NumCast` conversion must fail gracefully rather than panic or silently wrap.

- [ ] **`Point` and `Line` `Display` format** — assert `format!("{}", point)` yields `"(x,y)"` and `format!("{}", line)` yields `"(start->end)"` for known values; catches format regressions.
