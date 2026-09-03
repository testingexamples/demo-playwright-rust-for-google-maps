---
name: demo-playwright-rust-for-google-maps-skill
description: Explains the Playwright + Rust (playwright-rs) test-pattern demo against Google Maps; invoke when someone wants to understand, review, or adapt these tests — never to run them against the live Google Maps.
---

# Demo Playwright Rust for Google Maps — skill

## What this demo teaches

This repo demonstrates `playwright-rs` (Rust's community-maintained
Playwright binding, `padamson/playwright-rust`, pre-1.0) syntax and
interaction patterns against Google Maps, matching the sibling
`demo-playwright-javascript-for-google-maps` and
`demo-playwright-python-for-google-maps` repos test-for-test:

1. The page title contains `Google Maps`.
2. Searching `[aria-label="Search Google Maps"]` for a place and pressing
   Enter updates the URL to contain the query.
3. Clicking the zoom-in control (`[aria-label="Zoom in"]`) increases the
   zoom level embedded in the URL's `@lat,lng,zoomz` segment.

Because most of the map renders to `<canvas>`/WebGL rather than
addressable DOM, these tests deliberately stick to the chrome around the
map and to the URL, rather than trying to click map pins directly.

## The one rule that matters

**Never run `cargo build`, `cargo check`, or `cargo test` in this repo**,
and never let anything else do so either. Google's Terms of Service
restrict automated querying of its services, including Google Maps.
Review `src/demo.rs` by reading it against `playwright-rs`'s documented
API (see `spec/index.md`'s Sources) — not by compiling it.

## Adapting the pattern to a site you can actually test

1. Copy `src/demo.rs`'s three-test structure.
2. Point `page.goto(...)` at a site you're allowed to test — for hands-on
   practice, use <https://testingexamples.github.io> (see the sibling
   `demo-playwright-rust` repo).
3. Update every selector and assertion, and update `spec/index.md` in the
   same change.
4. Only then run `cargo build`/`cargo test`.

This skill summarizes the repo. `AGENTS.md` and `spec/index.md` are the
source of truth — if this skill's summary ever disagrees with those, they
win.
