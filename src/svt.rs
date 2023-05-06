use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SvtResponse {
    pub status: String,
    pub data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    #[serde(rename = "pageNumber")]
    pub page_number: String,
    #[serde(rename = "prevPage")]
    pub prev_page: String,
    #[serde(rename = "nextPage")]
    pub next_page: String,
    #[serde(rename = "subPages")]
    pub sub_pages: Vec<SubPage>,
    pub meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    pub updated: String,
}

#[derive(Serialize, Deserialize)]
pub struct SubPage {
    #[serde(rename = "subPageNumber")]
    pub sub_page_number: String,
    #[serde(rename = "gifAsBase64")]
    pub gif_as_base64: String,
    #[serde(rename = "imageMap")]
    pub image_map: String,
    #[serde(rename = "altText")]
    pub alt_text: String,
}

pub struct SvtClient {
    base_url: String,
    http_client: Client,
}

impl SvtClient {
    pub fn new(base_url: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
        }
    }
    pub async fn get_page(&self, page: &String) -> Result<SvtResponse, Error> {
        let response = self
            .http_client
            .get(format!("{}/{}", self.base_url, page))
            .send()
            .await?;

        // TODO check status codes. Examples here: https://blog.logrocket.com/making-http-requests-rust-reqwest/
        let response: SvtResponse = response.json().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn get_page() {
        let client = SvtClient::new(String::from("https://www.svt.se/text-tv/api"));
        let page = String::from("100");
        let result = client.get_page(&page).await;
        let response = match result {
            Ok(result) => result,
            Err(error) => panic!("Did not expect error: {:?}", error),
        };

        assert_eq!(response.data.page_number, "100")
    }
}
