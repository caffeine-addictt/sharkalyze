use lazy_static::lazy_static;
use regex::bytes::Regex;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref IMAGE_EXTENSION: Regex =
        Regex::new(r".+\.(jpe?g|png|gif|webp|avif|svg|ico|tiff|bmp)$").unwrap();
    static ref VIDEO_EXTENSION: Regex =
        Regex::new(r".+\.(mp4|webm|mkv|ogv|mpg|mpeg|mov|avi|wmv|flv|webm)$").unwrap();
    static ref AUDIO_EXTENSION: Regex = Regex::new(r".+\.(mp3|wav|ogg|flac|aac)$").unwrap();
    static ref DOCUMENT_EXTENSION: Regex =
        Regex::new(r".+\.(pdf|doc|docx|xls|xlsx|ppt|pptx|odt|ods|odp)$").unwrap();
}

/// Discovered hyprlinks
#[derive(Serialize, Deserialize, Debug)]
pub struct Hyprlink {
    // Url
    pub url: String,                // ok
    pub is_ssl_https: u8,           // ok
    pub url_entropy: f32,           // ok
    pub is_samesite: u8,            // ok
    pub is_external: u8,            // ok
    pub is_successful_response: u8, // ok
    pub request_timed_out: u8,      // ok

    // Generic
    pub is_html: u8,           // ok
    pub is_javascript: u8,     // ok
    pub is_json: u8,           // ok
    pub is_css: u8,            // ok
    pub is_image: u8,          // ok
    pub is_video: u8,          // ok
    pub is_audio: u8,          // ok
    pub url_type_is_known: u8, // ok

    // Explicit url
    pub is_html_from_url: u8,       // ok
    pub is_javascript_from_url: u8, // ok
    pub is_json_from_url: u8,       // ok
    pub is_css_from_url: u8,        // ok

    // General url
    pub is_image_from_url: u8,        // ok
    pub is_video_from_url: u8,        // ok
    pub is_audio_from_url: u8,        // ok
    pub is_document_from_url: u8,     // ok
    pub cannot_identify_from_url: u8, // ok

    // Explict header
    pub is_utf8_from_header: u8,               // ok
    pub is_html_from_content_header: u8,       // ok
    pub is_javascript_from_content_header: u8, // ok
    pub is_json_from_content_header: u8,       // ok
    pub is_css_from_content_header: u8,        // ok
    pub is_xml_from_content_header: u8,        // ok
    pub is_csv_from_content_header: u8,        // ok
    pub is_plain_from_content_header: u8,      // ok

    // General header
    pub is_image_from_content_header: u8,            // ok
    pub is_video_from_content_header: u8,            // ok
    pub is_audio_from_content_header: u8,            // ok
    pub is_xtoken_from_content_header: u8,           // ok
    pub is_message_from_content_header: u8,          // ok
    pub is_multipart_from_content_header: u8,        // ok
    pub is_not_usual_format_from_content_header: u8, // ok

    // Content length
    pub content_length: usize, // ok
}

impl Hyprlink {
    pub fn new(url: String) -> Self {
        let is_html_from_url = format_bool(url.ends_with(".htm") || url.ends_with(".html"));
        let is_javascript_from_url = format_bool(url.ends_with(".js"));
        let is_json_from_url = format_bool(url.ends_with(".json"));
        let is_css_from_url = format_bool(url.ends_with(".css"));
        let is_image_from_url = format_bool(IMAGE_EXTENSION.is_match(url.as_bytes()));
        let is_video_from_url = format_bool(VIDEO_EXTENSION.is_match(url.as_bytes()));
        let is_audio_from_url = format_bool(AUDIO_EXTENSION.is_match(url.as_bytes()));
        let is_document_from_url = format_bool(DOCUMENT_EXTENSION.is_match(url.as_bytes()));

        Hyprlink {
            url,
            is_ssl_https: 0,
            url_entropy: 0f32,
            is_samesite: 0,
            is_external: 0,
            is_successful_response: 0,
            request_timed_out: 0,
            is_html: 0,
            is_javascript: 0,
            is_json: 0,
            is_css: 0,
            is_image: 0,
            is_video: 0,
            is_audio: 0,
            url_type_is_known: 0,
            is_html_from_url,
            is_javascript_from_url,
            is_json_from_url,
            is_css_from_url,
            is_image_from_url,
            is_video_from_url,
            is_audio_from_url,
            is_document_from_url,
            cannot_identify_from_url: format_bool(
                is_html_from_url
                    + is_javascript_from_url
                    + is_json_from_url
                    + is_css_from_url
                    + is_image_from_url
                    + is_video_from_url
                    + is_audio_from_url
                    + is_document_from_url
                    == 0,
            ),
            is_utf8_from_header: 0,
            is_html_from_content_header: 0,
            is_javascript_from_content_header: 0,
            is_json_from_content_header: 0,
            is_css_from_content_header: 0,
            is_xml_from_content_header: 0,
            is_csv_from_content_header: 0,
            is_plain_from_content_header: 0,
            is_image_from_content_header: 0,
            is_video_from_content_header: 0,
            is_audio_from_content_header: 0,
            is_xtoken_from_content_header: 0,
            is_message_from_content_header: 0,
            is_multipart_from_content_header: 0,
            is_not_usual_format_from_content_header: 0,
            content_length: 0,
        }
    }

    /// Handles generics based on the current values
    /// Example: if is_javascript_from_content_header or is_javascript_from_url are true,
    /// then we set is_javascript to true
    pub fn resolve_generics(&mut self) {
        self.is_html = format_bool(self.is_html_from_content_header + self.is_html_from_url > 0);
        self.is_javascript =
            format_bool(self.is_javascript_from_content_header + self.is_javascript_from_url > 0);
        self.is_json = format_bool(self.is_json_from_content_header + self.is_json_from_url > 0);
        self.is_css = format_bool(self.is_css_from_content_header + self.is_css_from_url > 0);
        self.is_image = format_bool(self.is_image_from_content_header + self.is_image_from_url > 0);
        self.is_video = format_bool(self.is_video_from_content_header + self.is_video_from_url > 0);
        self.is_audio = format_bool(self.is_audio_from_content_header + self.is_audio_from_url > 0);
    }
}

/// Vector structure that is generated
#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    // Url
    pub url: String,                               // ok
    pub is_ssl_https: u8,                          // ok
    pub url_entropy: f32,                          // ok
    pub is_utf8_from_header: u8,                   // ok
    pub contenttype_header_contains_text_html: u8, // ok

    // Links
    pub hyprlinks: Vec<Hyprlink>,          // ok
    pub hyprlinks_count: usize,            // ok
    pub external_link_count: usize,        // ok
    pub samesite_link_count: usize,        // ok
    pub external_samesite_link_ratio: f32, // ok
    /// <a href="" /> or <a href="#" />
    pub null_hyprlinks_count: usize, // ok
    /// <link />
    pub link_tag_count: usize, // ok

    // HTML
    pub html_length: usize,            // ok
    pub html_comments_count: usize,    // ok
    pub title_tag_in_head_section: u8, // ok
    /// ratio
    pub title_tag_and_url_overlap: f32, // ok
    pub navbar_present: u8,            // ok
    pub footer_present: u8,            // ok

    // JavaScript
    pub javascript_count: usize,                 // ok
    pub samesite_javascript_count: usize,        // ok
    pub external_javascript_count: usize,        // ok
    pub external_samesite_javascript_ratio: f32, // ok
    /// 200 OK response?
    pub javascript_reachable_count: usize, // ok
    /// Not 200 OK response?
    pub javascript_unreachable_count: usize, // ok
    pub javascript_reachable_ratio: f32,         // ok
}

impl Vector {
    /// Initializes a new Vector with default values
    pub fn new(url: String) -> Self {
        Vector {
            url,
            is_ssl_https: 0,
            url_entropy: 0f32,
            is_utf8_from_header: 0,
            contenttype_header_contains_text_html: 0,
            hyprlinks: vec![],
            hyprlinks_count: 0,
            external_link_count: 0,
            samesite_link_count: 0,
            external_samesite_link_ratio: 0f32,
            null_hyprlinks_count: 0,
            link_tag_count: 0,
            html_length: 0,
            html_comments_count: 0,
            title_tag_in_head_section: 0,
            title_tag_and_url_overlap: 0f32,
            navbar_present: 0,
            footer_present: 0,
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

pub fn format_bool(b: bool) -> u8 {
    b as u8
}

pub fn format_u8(n: u8) -> bool {
    n == 1
}
