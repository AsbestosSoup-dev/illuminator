# illuminator

A ray tracer in Rust, built in small chunks that each end with an image.
Work happens in [`ROADMAP.md`](ROADMAP.md); design notes and timings go in [`NOTES.md`](NOTES.md).

## The loop

1. Pick the next unticked step in `ROADMAP.md`.
2. Write the code. Run the milestone's tests: `cargo test --test m00_basics`.
3. Render and look: `cargo run --release > out.ppm && python3 scripts/ppm2png.py out.ppm out.png`.
4. Tick the box, commit, push.
5. Check progress locally any time: `python3 scripts/check.py`.

## What CI does

On every push (`.github/workflows/ci.yml`):

- builds the crate in release mode
- runs `scripts/check.py`: **every fully ticked milestone must pass its tests** (so ticking a box early, or breaking an old milestone, turns CI red)
- checks `cargo fmt` (fix with `cargo fmt`) and runs clippy (advisory)
- runs the renderer, converts the PPM to a PNG, and attaches it to the run as the `render` artifact along with the render time and a progress table in the run summary

## Adding tests for a new milestone

1. Create `tests/mNN_name.rs`, a normal Rust integration test that uses the `illuminator` library.
2. Tag the milestone heading in `ROADMAP.md`: `## M4 Something <!-- test: mNN_name -->`.

Tests only touch the public API listed in the roadmap, so how you structure things inside is up to you.

## Requirements

Rust (stable) and Python 3.8+ (standard library only).
