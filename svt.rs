use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SvtResponse {
    status: String,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "pageNumber")]
    page_number: String,
    #[serde(rename = "prevPage")]
    prev_page: String,
    #[serde(rename = "nextPage")]
    next_page: String,
    #[serde(rename = "subPages")]
    sub_pages: Vec<SubPage>,
    meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubPage {
    #[serde(rename = "subPageNumber")]
    sub_page_number: String,
    #[serde(rename = "gifAsBase64")]
    gif_as_base64: String,
    #[serde(rename = "imageMap")]
    image_map: String,
    #[serde(rename = "altText")]
    alt_text: String,
}

async fn make_async_http_request() -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .get("https://www.svt.se/text-tv/api/100")
        .send()
        .await?;
    // let model: Welcome = serde_json::from_str(&json).unwrap();
    let response: SvtResponse = response.json().await?;
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        make_async_http_
        // assert_eq!(4, internal_adder(2, 2));
    }
}
