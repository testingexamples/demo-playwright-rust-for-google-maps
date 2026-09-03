# Demo Playwright Rust for Google Maps

> **Read this before running anything.** Google's [Terms of Service](https://www.google.com/policies/terms/)
> restrict automated querying of its services, including Google Maps. The
> tests in this repo exist to show the syntax and interaction *pattern* of
> Playwright's Rust binding — they are not meant to be run repeatedly, or
> at all, against the live Google Maps. This repo's own history never runs
> `cargo build`, `cargo check`, or `cargo test` against it. If you want to
> practise these same patterns hands-on, point a similar script at
> [testingexamples.github.io](https://testingexamples.github.io) instead
> (see the sibling repo `demo-playwright-rust`), which was built exactly
> for that: stable ids, names, classes, and text that don't shift under
> you.
>
> Google Maps is also a much harder automation target than a typical page:
> most of the map itself renders to a `<canvas>` element (or WebGL), so
> the individual map pins and controls generally aren't addressable DOM
> elements the way ordinary page content is. The tests below stick to the
> chrome around the map (the search box, the zoom button) and to the URL,
> which Google Maps keeps in sync with the current view.

Demonstration of:

* [Playwright](https://www.playwright.dev/) browser automation testing
* [Rust](https://www.rust-lang.org/) programming language
* [Cargo](https://doc.rust-lang.org/cargo/) build tool and package manager
* [Chromium](https://www.chromium.org/) open source web browser

Playwright ships official bindings for JavaScript, Python, .NET, and Java.
Rust is community-maintained. This demo uses [`playwright-rs`](https://crates.io/crates/playwright-rs)
(`padamson/playwright-rust`), which is actively maintained but still
pre-1.0 and stabilising its API. Be careful which crate you install: an
older, unrelated crate published on crates.io simply as `playwright`
(`octaltree/playwright-rust`) has been abandoned since 2022 — don't reach
for that one.

The exact scenario this demo describes (target URL, selectors, assertions)
is specified in [spec/index.md](spec/index.md); the code and spec must
agree.

## What this demo tests

Unlike the plain `demo-playwright-rust` walkthrough (which only logs what
it finds), this repo demonstrates a REAL test with real assertions —
matching the sibling `demo-playwright-javascript-for-google-maps` and
`demo-playwright-python-for-google-maps` repos test-for-test:

1. **Title test**: the Google Maps page title contains `Google Maps`.
2. **Search test**: filling the search box
   (`[aria-label="Search Google Maps"]`) with a query and pressing Enter
   updates the URL to contain that query.
3. **Zoom test**: clicking the zoom-in control
   (`[aria-label="Zoom in"]`) increases the zoom level embedded in the
   URL's `@lat,lng,zoomz` segment.

## Install

### Install Rust and Cargo

Install Rust (which includes Cargo) from <https://www.rust-lang.org/tools/install>,
typically via `rustup`.

```sh
rustc --version
cargo --version
```

### Dependencies

This repo's [Cargo.toml](Cargo.toml) declares `playwright-rs`, `tokio`,
and `anyhow` as ordinary dependencies, the same as any other Playwright
Rust project — so the code reads and would build like normal Rust. But per
the caution above, this repo does not actually run `cargo build` or
`cargo test` against the live site as part of its own maintenance.

## Run

Do not run this against the live Google Maps. If you have adapted this
pattern to point at a site you're allowed to test, the usual commands
apply:

```sh
cargo build
cargo test
```

## Tracking

* Package: demo-playwright-rust-for-google-maps
* Version: 1.0.0
* Created: 2026-09-03T00:00:00Z
* Updated: 2026-09-03T00:00:00Z
* License: GPL-2.0-or-greater or for custom license contact us
* Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)
