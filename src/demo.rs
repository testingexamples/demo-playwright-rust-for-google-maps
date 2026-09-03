//! Demo of Playwright browser automation with Rust, against Google Maps.
//!
//! CAUTION — read AGENTS.md and README.md before touching this file:
//! Google's Terms of Service restrict automated querying of its services,
//! including Google Maps. These tests exist to show the syntax and
//! interaction pattern of `playwright-rs`, matching the sibling
//! JavaScript/Python/TypeScript `-for-google-maps` demos test-for-test.
//! They are not meant to be run repeatedly, or at all, against the live
//! Google Maps. Do not add a CI job, pre-commit hook, or anything else
//! that executes `cargo test` in this repo against the real site.
//!
//! Unlike the plain `demo-playwright-rust` walkthrough (which only logs
//! what it finds), this repo demonstrates a REAL test with real
//! assertions, matching the sibling `-for-google-maps` repos.
//!
//! ## Tracking
//!
//!   * Package: demo-playwright-rust-for-google-maps
//!   * Version: 1.0.0
//!   * Created: 2026-09-03T00:00:00Z
//!   * Updated: 2026-09-03T00:00:00Z
//!   * License: GPL-2.0-or-greater or for custom license contact us
//!   * Contact: Joel Parker Henderson (joel@joelparkerhenderson.com)

#[cfg(test)]
mod tests {
    use playwright_rs::Playwright;
    use regex::Regex;

    /// Extracts the zoom level from a Google Maps URL, e.g. the `15` in
    /// `.../@51.4816,-3.1791,15z/...`. Google Maps encodes the current
    /// zoom as the third `@lat,lng,ZOOMz` path segment.
    fn zoom_level(url: &str) -> Option<f64> {
        let re = Regex::new(r"@-?\d+\.\d+,-?\d+\.\d+,(\d+(?:\.\d+)?)z").ok()?;
        re.captures(url)?.get(1)?.as_str().parse().ok()
    }

    /// Test 1: the Google Maps page title contains "Google Maps".
    #[tokio::test]
    async fn google_maps_title_contains_google_maps() -> anyhow::Result<()> {
        let pw = Playwright::launch().await?;
        let browser = pw.chromium().launch().await?;
        let page = browser.new_page().await?;

        // 1. Visit.
        page.goto("https://www.google.com/maps", None).await?;

        let title = page.title().await?;
        assert!(
            title.contains("Google Maps"),
            "Expected page title {title:?} to contain \"Google Maps\""
        );

        browser.close().await?;
        Ok(())
    }

    /// Test 2: searching for a place updates the URL to contain the
    /// search query.
    #[tokio::test]
    async fn google_maps_search_url_contains_query() -> anyhow::Result<()> {
        let pw = Playwright::launch().await?;
        let browser = pw.chromium().launch().await?;
        let page = browser.new_page().await?;

        page.goto("https://www.google.com/maps", None).await?;

        // 2. Search, then submit.
        let query = "Cardiff Castle";
        let search_box = page.locator("[aria-label=\"Search Google Maps\"]");
        search_box.fill(query, None).await?;
        search_box.press("Enter", None).await?;

        // Give the single-page app a moment to update the URL after the
        // search — Google Maps does not perform a full navigation here,
        // so there is no `wait_for_load_state` transition to await.
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let url = page.url();
        let normalized_url = url.to_lowercase();
        for word in query.to_lowercase().split_whitespace() {
            assert!(
                normalized_url.contains(word),
                "Expected URL {url:?} to contain {word:?} from the search query {query:?}"
            );
        }

        browser.close().await?;
        Ok(())
    }

    /// Test 3: clicking the zoom-in control changes the zoom level
    /// embedded in the URL.
    #[tokio::test]
    async fn google_maps_zoom_in_changes_url_zoom_parameter() -> anyhow::Result<()> {
        let pw = Playwright::launch().await?;
        let browser = pw.chromium().launch().await?;
        let page = browser.new_page().await?;

        page.goto("https://www.google.com/maps", None).await?;

        let search_box = page.locator("[aria-label=\"Search Google Maps\"]");
        search_box.fill("Cardiff Castle", None).await?;
        search_box.press("Enter", None).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let url_before = page.url();
        let zoom_before = zoom_level(&url_before)
            .expect("Expected the URL to contain a Google Maps @lat,lng,zoomz segment");

        // 5. Zoom in.
        let zoom_in = page.locator("[aria-label=\"Zoom in\"]");
        zoom_in.click(None).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        let url_after = page.url();
        let zoom_after = zoom_level(&url_after)
            .expect("Expected the URL to still contain a Google Maps @lat,lng,zoomz segment");

        assert!(
            zoom_after > zoom_before,
            "Expected zoom level to increase after clicking Zoom in: {zoom_before} -> {zoom_after}"
        );

        browser.close().await?;
        Ok(())
    }
}
