use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

/// Vector structure that is generated
#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    id: usize,

    // Url
    url: String,
    is_ssl_https: bool,
    url_entropy: u8,

    // Links
    hyprlinks: Vec<Vector>,
    hyprlinks_count: usize,
    external_link_count: usize,
    samesite_link_count: usize,
    external_samesite_link_ratio: f32,
    /// <a href="" /> or <a href="#" />
    null_hyprlinks_count: usize,
    /// <link />
    link_tag_count: usize,

    // HTML
    html_length: usize,
    html_comments_count: usize,
    title_tag_in_head_section: bool,
    /// ratio
    title_tag_and_url_overlap: f32,
    favicon_is_samesite: bool,
    navbar_present: bool,
    favicon_present: bool,
    meta_description_present: bool,

    // JavaScript
    javascript_count: usize,
    samesite_javascript_count: usize,
    external_javascript_count: usize,
    external_samesite_javascript_ratio: f32,
    /// 200 OK response?
    javascript_reachable_count: usize,
    /// Not 200 OK response?
    javascript_unreachable_count: usize,
    javascript_reachable_ratio: f32,
}

static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);

impl Vector {
    /// Initializes a new Vector with default values
    pub fn new() -> Self {
        let curr_id = CURRENT_ID.fetch_add(1, Ordering::SeqCst);

        Vector {
            id: curr_id,
            url: String::new(),
            is_ssl_https: false,
            url_entropy: 0,
            hyprlinks: vec![],
            hyprlinks_count: 0,
            external_link_count: 0,
            samesite_link_count: 0,
            external_samesite_link_ratio: 0f32,
            null_hyprlinks_count: 0,
            link_tag_count: 0,
            html_length: 0,
            html_comments_count: 0,
            title_tag_in_head_section: false,
            title_tag_and_url_overlap: 0f32,
            favicon_is_samesite: false,
            navbar_present: false,
            favicon_present: false,
            meta_description_present: false,
            javascript_count: 0,
            samesite_javascript_count: 0,
            external_javascript_count: 0,
            external_samesite_javascript_ratio: 0f32,
            javascript_reachable_count: 0,
            javascript_unreachable_count: 0,
            javascript_reachable_ratio: 0f32,
        }
    }
}
