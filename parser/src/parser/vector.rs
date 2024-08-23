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
    pub url: String,                  // ok
    pub is_ssl_https: bool,           // ok
    pub url_entropy: f32,             // ok
    pub is_samesite: bool,            // ok
    pub is_external: bool,            // ok
    pub is_successful_response: bool, // ok

    // Generic
    pub is_html: bool,           // ok
    pub is_javascript: bool,     // ok
    pub is_json: bool,           // ok
    pub is_css: bool,            // ok
    pub is_image: bool,          // ok
    pub is_video: bool,          // ok
    pub is_audio: bool,          // ok
    pub url_type_is_known: bool, // ok

    // Explicit url
    pub is_html_from_url: bool,       // ok
    pub is_javascript_from_url: bool, // ok
    pub is_json_from_url: bool,       // ok
    pub is_css_from_url: bool,        // ok

    // General url
    pub is_image_from_url: bool,        // ok
    pub is_video_from_url: bool,        // ok
    pub is_audio_from_url: bool,        // ok
    pub is_document_from_url: bool,     // ok
    pub cannot_identify_from_url: bool, // ok

    // Explict header
    pub is_utf8_from_header: bool,               // ok
    pub is_html_from_content_header: bool,       // ok
    pub is_javascript_from_content_header: bool, // ok
    pub is_json_from_content_header: bool,       // ok
    pub is_css_from_content_header: bool,        // ok
    pub is_xml_from_content_header: bool,        // ok
    pub is_csv_from_content_header: bool,        // ok
    pub is_plain_from_content_header: bool,      // ok

    // General header
    pub is_image_from_content_header: bool,            // ok
    pub is_video_from_content_header: bool,            // ok
    pub is_audio_from_content_header: bool,            // ok
    pub is_xtoken_from_content_header: bool,           // ok
    pub is_message_from_content_header: bool,          // ok
    pub is_multipart_from_content_header: bool,        // ok
    pub is_not_usual_format_from_content_header: bool, // ok

    // Content length
    pub content_length: usize, // ok
}

impl Hyprlink {
    pub fn new(url: String) -> Self {
        let is_html_from_url = url.ends_with(".htm") || url.ends_with(".html");
        let is_javascript_from_url = url.ends_with(".js");
        let is_json_from_url = url.ends_with(".json");
        let is_css_from_url = url.ends_with(".css");
        let is_image_from_url = IMAGE_EXTENSION.is_match(url.as_bytes());
        let is_video_from_url = VIDEO_EXTENSION.is_match(url.as_bytes());
        let is_audio_from_url = AUDIO_EXTENSION.is_match(url.as_bytes());
        let is_document_from_url = DOCUMENT_EXTENSION.is_match(url.as_bytes());

        Hyprlink {
            url,
            is_ssl_https: false,
            url_entropy: 0f32,
            is_samesite: false,
            is_external: false,
            is_successful_response: false,
            is_html: false,
            is_javascript: false,
            is_json: false,
            is_css: false,
            is_image: false,
            is_video: false,
            is_audio: false,
            url_type_is_known: false,
            is_html_from_url,
            is_javascript_from_url,
            is_json_from_url,
            is_css_from_url,
            is_image_from_url,
            is_video_from_url,
            is_audio_from_url,
            is_document_from_url,
            cannot_identify_from_url: !(is_html_from_url
                || is_javascript_from_url
                || is_json_from_url
                || is_css_from_url
                || is_image_from_url
                || is_video_from_url
                || is_audio_from_url
                || is_document_from_url),
            is_utf8_from_header: false,
            is_html_from_content_header: false,
            is_javascript_from_content_header: false,
            is_json_from_content_header: false,
            is_css_from_content_header: false,
            is_xml_from_content_header: false,
            is_csv_from_content_header: false,
            is_plain_from_content_header: false,
            is_image_from_content_header: false,
            is_video_from_content_header: false,
            is_audio_from_content_header: false,
            is_xtoken_from_content_header: false,
            is_message_from_content_header: false,
            is_multipart_from_content_header: false,
            is_not_usual_format_from_content_header: false,
            content_length: 0,
        }
    }

    /// Handles generics based on the current values
    /// Example: if is_javascript_from_content_header or is_javascript_from_url are true,
    /// then we set is_javascript to true
    pub fn resolve_generics(&mut self) {
        self.is_html = self.is_html_from_content_header || self.is_html_from_url;
        self.is_javascript = self.is_javascript_from_content_header || self.is_javascript_from_url;
        self.is_json = self.is_json_from_content_header || self.is_json_from_url;
        self.is_css = self.is_css_from_content_header || self.is_css_from_url;
        self.is_image = self.is_image_from_content_header || self.is_image_from_url;
        self.is_video = self.is_video_from_content_header || self.is_video_from_url;
        self.is_audio = self.is_audio_from_content_header || self.is_audio_from_url;
    }
}

/// Vector structure that is generated
#[derive(Serialize, Deserialize, Debug)]
pub struct Vector {
    // Url
    pub url: String,                                 // ok
    pub is_ssl_https: bool,                          // ok
    pub url_entropy: f32,                            // ok
    pub is_utf8_from_header: bool,                   // ok
    pub contenttype_header_contains_text_html: bool, // ok

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
    pub html_length: usize,              // ok
    pub html_comments_count: usize,      // ok
    pub title_tag_in_head_section: bool, // ok
    /// ratio
    pub title_tag_and_url_overlap: f32, // ok
    pub navbar_present: bool,            // ok
    pub footer_present: bool,            // ok

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
            is_ssl_https: false,
            url_entropy: 0f32,
            is_utf8_from_header: false,
            contenttype_header_contains_text_html: false,
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
            navbar_present: false,
            footer_present: false,
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
