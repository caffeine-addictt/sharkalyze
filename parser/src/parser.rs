use std::{sync::Arc, time::Duration};

use anyhow::Result;
use futures_util::future::join_all;
use tokio::sync::Semaphore;
use url::Url;

use crate::{
    asyncreq,
    parser::vector::{format_bool, format_u8},
    weburl,
};

pub mod crawl;
pub mod overlap;
pub mod vector;

/// Entrypoint for parallel processing
pub async fn generate_vector(client: reqwest::Client, url: String) -> Result<vector::Vector> {
    let mut vector = vector::Vector::new(url.as_str().to_string());

    // SSL
    if vector.url.starts_with("https://") {
        vector.is_ssl_https = 1;
    }

    // Calculating entropy
    vector.url_entropy = weburl::calculate_entropy(&vector.url);

    // Resolve url
    let discovered_urls = crawl::crawl_page(&client, &mut vector).await?;
    let root_url = Url::parse(url.as_str())?;

    // Run concurrently
    let semaphore = Arc::new(Semaphore::new(20));
    let mut futures = vec![];

    for to_fetch in &discovered_urls {
        let semaphore = Arc::clone(&semaphore);
        let client = client.clone();
        let root_url = root_url.clone();

        futures.push(async move {
            let _permit = semaphore.acquire().await?;
            if let Ok(hyprlink_vector) =
                generate_hyprlink_vector(&client, to_fetch, &root_url).await
            {
                if format_u8(hyprlink_vector.is_external) {
                    vector.external_link_count += 1;
                }
                if format_u8(hyprlink_vector.is_samesite) {
                    vector.samesite_link_count += 1;
                }

                // js
                if format_u8(hyprlink_vector.is_javascript) {
                    vector.javascript_count += 1;

                    if format_u8(hyprlink_vector.is_external) {
                        vector.external_javascript_count += 1;
                    }
                    if format_u8(hyprlink_vector.is_samesite) {
                        vector.samesite_javascript_count += 1;
                    }

                    if format_u8(hyprlink_vector.is_successful_response) {
                        vector.javascript_reachable_count += 1;
                    } else {
                        vector.javascript_unreachable_count += 1;
                    }
                }

                return Ok(hyprlink_vector);
            }

            Err(anyhow::anyhow!("Failed to fetch {}", to_fetch))
        })
    }

    // resolve futures
    let binding = join_all(futures).await;
    vector.hyprlinks.extend(binding.into_iter().flatten());

    vector.external_samesite_link_ratio =
        vector.external_link_count as f32 / vector.samesite_link_count as f32;
    vector.javascript_reachable_ratio =
        vector.javascript_reachable_count as f32 / vector.javascript_unreachable_count as f32;
    vector.external_samesite_javascript_ratio =
        vector.external_javascript_count as f32 / vector.samesite_javascript_count as f32;

    Ok(vector)
}

/// To generate a hyprlink vector
async fn generate_hyprlink_vector(
    client: &reqwest::Client,
    url_str: &str,
    root_url: &Url,
) -> Result<vector::Hyprlink> {
    let mut hyprlink = vector::Hyprlink::new(url_str.to_string());

    // Account for when url is relative
    let url = if weburl::SAMESITE_URL_REGEXP.is_match(url_str) {
        hyprlink.is_samesite = 1;

        // handle root level /
        let new_base = if url_str.starts_with('/') {
            let mut new_to_fetch = root_url.clone();
            new_to_fetch.set_path(url_str);

            new_to_fetch
        } else {
            root_url.join(url_str)?
        };

        Url::parse(new_base.as_str())?.to_string()
    } else {
        url_str.to_string()
    };

    hyprlink.is_external = format_bool(url == root_url.to_string());

    if url.starts_with("https://") {
        hyprlink.is_ssl_https = 1;
    }

    hyprlink.url_entropy = weburl::calculate_entropy(&url);

    let req = match asyncreq::make_req(client.get(&url).timeout(Duration::from_secs(30))).await {
        Ok(req) => req,
        Err(e) => {
            if e.is_timeout() {
                hyprlink.request_timed_out = 1;
                return Ok(hyprlink);
            }

            anyhow::bail!(e);
        }
    };

    if !req.status().is_success() {
        hyprlink.is_successful_response = 0;
        return Ok(hyprlink);
    }
    hyprlink.is_successful_response = 1;

    if let Some(val) = req.headers().get("content-type") {
        if let Ok(header) = val.to_str() {
            // Explicit headers
            hyprlink.is_utf8_from_header = format_bool(header.contains("utf-8"));
            hyprlink.is_html_from_content_header = format_bool(header.contains("text/html"));
            hyprlink.is_javascript_from_content_header =
                format_bool(header.contains("text/javascript"));
            hyprlink.is_json_from_content_header = format_bool(header.contains("application/json"));
            hyprlink.is_css_from_content_header = format_bool(header.contains("text/css"));
            hyprlink.is_xml_from_content_header = format_bool(header.contains("text/xml"));
            hyprlink.is_csv_from_content_header = format_bool(header.contains("text/csv"));
            hyprlink.is_plain_from_content_header = format_bool(header.contains("text/plain"));

            // General headers
            hyprlink.is_image_from_content_header = format_bool(header.contains("image/"));
            hyprlink.is_video_from_content_header = format_bool(header.contains("video/"));
            hyprlink.is_audio_from_content_header = format_bool(header.contains("audio/"));
            hyprlink.is_xtoken_from_content_header = format_bool(header.contains("x-token/"));
            hyprlink.is_message_from_content_header = format_bool(header.contains("message/"));
            hyprlink.is_multipart_from_content_header = format_bool(header.contains("multipart/"));
            hyprlink.is_not_usual_format_from_content_header = format_bool(
                hyprlink.is_image_from_content_header
                    + hyprlink.is_video_from_content_header
                    + hyprlink.is_audio_from_content_header
                    + hyprlink.is_xtoken_from_content_header
                    + hyprlink.is_message_from_content_header
                    + hyprlink.is_multipart_from_content_header
                    == 0,
            );

            hyprlink.resolve_generics();
        }
    }

    hyprlink.content_length = usize::max(
        req.content_length().unwrap_or(0) as usize,
        req.text().await.unwrap_or(String::new()).len(),
    );

    Ok(hyprlink)
}
