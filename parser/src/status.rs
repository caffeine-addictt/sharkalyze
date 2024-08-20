use lazy_static::lazy_static;
use url::Url;

lazy_static! {
    pub static ref TERM_WIDTH: usize = term_size::dimensions().map(|(w, _)| w).unwrap_or(80) - 40;
}

pub struct Status<'a> {
    url: &'a Url,
}

impl<'a> Status<'a> {
    pub fn new(url: &'a Url) -> Self {
        Status { url }
    }

    // Format strings for the progress bar
    // Messages are blue, and time is either blue, yellow or red
    // Format:
    // URL - message (s)          1 second
    pub fn format(&self, s: &str) -> String {
        format!("{} - \x1b[94m{s}\x1b[0m", self.url)
    }
}
