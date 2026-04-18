# Code Review TODO

## Unnecessary Code

- `Point::Display` ([floats.rs:23-27](src/floats.rs#L23-L27)) — never called anywhere in the codebase.
- `Line::Display` ([floats.rs:38-42](src/floats.rs#L38-L42)) — never called anywhere in the codebase.
- `indoc` in `[dependencies]` ([Cargo.toml:8](Cargo.toml#L8)) — used only in tests; move to `[dev-dependencies]`.

## Unnecessary Publics

- `SVGLine` ([svg.rs:46](src/svg.rs#L46)) — only used within `svg.rs` and its own tests; make `pub(crate)` or private.
- `SVGLine::new` ([svg.rs:59](src/svg.rs#L59)) — only called in tests; make `pub(crate)`.
- `SVGLine::from_line` ([svg.rs:62](src/svg.rs#L62)) — called only from within `svg.rs`; make private.
- `CanvasSize` and its fields `x`, `y` ([svg.rs:53-56](src/svg.rs#L53-L56)) — used only inside `svg.rs`; no reason to be `pub`.
- `Svg::add_line` ([svg.rs:21](src/svg.rs#L21)) — only used internally and in tests; make `pub(crate)` or private.

## Tests to Create

- `f2canvas` with out-of-range floats: negative values and values `> 1.0` should return an error — currently the `NumCast` cast silently fails with a generic "Cast error" message.
- `f2canvas` with non-finite inputs (`NaN`, `±infinity`) — these are valid `f32` values and the current code does not guard against them.
- `SVGLine::from_line` — verify that float-space coordinates map correctly to pixel coordinates for a known canvas size.
- `Svg::add_float_line` — end-to-end test that a `Line<f32>` produces the expected SVG output.
- `Line::new_random` — assert that generated coordinates are within `[0.0, 1.0]`.

## Risks

- **No range validation on float inputs** ([svg.rs:62-69](src/svg.rs#L62-L69)): `from_line` does not reject floats outside `[0.0, 1.0]`. A value `> 1.0` produces a pixel coordinate exceeding the canvas dimension; a negative value causes a failed `NumCast`. The error message ("Cast error") gives no indication of which coordinate is the culprit.
- **`SVGLine::new` parameter order** ([svg.rs:59](src/svg.rs#L59)): signature is `(x1, x2, y1, y2)` — grouping by axis rather than by point. The conventional expectation is `(x1, y1, x2, y2)`. Easy to pass arguments in the wrong order, and nothing in the type system catches it.
- **`u16` overflow on canvas math**: if a float slightly above `1.0` slips through (e.g. after a rounding step), `f2canvas` can produce a value that overflows `u16` silently on release builds (Rust panics in debug, wraps in release with `--release`).

## Three Potential Improvements

1. **Validate float coordinates at the boundary** — add a guard in `add_float_line` or `from_line` that checks `0.0 <= coord <= 1.0` before conversion, returning a descriptive error (e.g. `"x1 out of range: 1.3"`).
2. **Normalise `SVGLine::new` parameter order to `(x1, y1, x2, y2)`** — matches SVG attribute order and the mental model of "start point, end point", reducing the chance of silent argument swaps.
3. **Replace the `println!` output with file I/O** — writing SVG to stdout forces shell redirection and makes the tool hard to compose; accepting an output path (or writing to a named file) makes intent explicit and easier to test.
