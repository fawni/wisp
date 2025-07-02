use crate::WOLFRAM;

pub const BASE_URL: &str = "http://api.wolframalpha.com/v1/result";

pub struct Wolfram;

impl Wolfram {
    pub async fn query(query: String) -> Result<String, reqwest::Error> {
        reqwest::get(format!(
            "{}?appid={}&i={}",
            BASE_URL,
            *WOLFRAM,
            urlencoding::encode(&query)
        ))
        .await?
        .error_for_status()?
        .text()
        .await
    }
}
