use lazy_static::lazy_static;

lazy_static! {
    pub static ref TERM_WIDTH: usize = term_size::dimensions().map(|(w, _)| w).unwrap_or(80) - 40;
}

pub struct Status {
    url: String,
}

impl Status {
    pub fn new(url: String) -> Self {
        Status { url }
    }

    // Format strings for the progress bar
    // Messages are blue, and time is either blue, yellow or red
    // Format:
    // URL - message (s)          1 second
    pub fn format(&self, s: &str) -> String {
        format!("{:>20} - \x1b[94m{s}\x1b[0m", self.url)
    }
}
