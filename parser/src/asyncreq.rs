use anyhow::Result;

use reqwest::{RequestBuilder, Response};
use tokio::sync::Semaphore;

static PERMITS: Semaphore = Semaphore::const_new(500);

/// Make a request in async.
/// This will acquire a permit and release it after the request is done.
pub async fn make_req(req: RequestBuilder) -> Result<Response, reqwest::Error> {
    let _permit = PERMITS.acquire().await.unwrap();
    req.send().await
}
