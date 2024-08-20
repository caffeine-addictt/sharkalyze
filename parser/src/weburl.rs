use anyhow::{Context, Result};

use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    pub static ref URL_REGEXP: Regex = Regex::new(r"^https?://").unwrap();
    pub static ref SAMESITE_URL_REGEXP: Regex = Regex::new(r#"^/?[^:?#]+"#).unwrap();
    pub static ref HTML_URL: Regex = Regex::new(r#"(?:href|src)\s*=\s*["']([^"']+)["']"#).unwrap();
}

pub fn parse_url(url: &str) -> Result<Url> {
    // Check if following RegExp
    if !URL_REGEXP.is_match(url) {
        anyhow::bail!("malformed URL: {}", url);
    }

    Url::parse(url).with_context(|| format!("failed to parse URL: {url}"))
}
