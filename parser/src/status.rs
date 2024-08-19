use std::time::{Duration, Instant};

use indicatif::HumanDuration;
use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref TERM_WIDTH: usize = term_size::dimensions().map(|(w, _)| w).unwrap_or(80) - 40;
}

pub struct Status<'a> {
    start: &'a Instant,
    url: &'a Url,
}

impl<'a> Status<'a> {
    pub fn new(start: &'a Instant, url: &'a Url) -> Self {
        Status { start, url }
    }

    // Format strings for the progress bar
    // Messages are blue, and time is either blue, yellow or red
    // Format:
    // URL - message (s)          1 second
    pub fn format(&self, s: &str) -> String {
        let msg = format!("{} - \x1b[94m{s}\x1b[0m", self.url);
        let width = usize::max(*TERM_WIDTH - msg.len() + 15, 0);

        let elap = self.start.elapsed();
        let mut elap_str = String::from(HumanDuration(elap).to_string().trim());

        if elap > Duration::from_secs(60 * 5) {
            // Red color if > 5 minutes
            elap_str = format!("\x1b[91m{elap_str}\x1b[0m");
        } else if elap > Duration::from_secs(60) {
            // Yellow color if > 1 minute
            elap_str = format!("\x1b[93m{elap_str}\x1b[0m");
        } else {
            // Blue
            elap_str = format!("\x1b[94m{elap_str}\x1b[0m");
        }

        format!("{msg}{:>width$}", elap_str)
    }
}
