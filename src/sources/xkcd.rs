use serde::Deserialize;

pub const BASE_URL: &str = "https://xkcd.com";
pub const LATEST_URL: &str = "https://xkcd.com/info.0.json";

#[derive(Deserialize)]
pub struct Comic {
    pub month: String,
    pub num: u32,
    // pub link: String,
    pub year: String,
    // pub news: String,
    pub safe_title: String,
    // pub transcript: String,
    pub alt: String,
    pub img: String,
    // pub title: String,
    pub day: String,
}

impl Comic {
    pub async fn from(id: u32) -> Result<Self, reqwest::Error> {
        reqwest::get(&format!("{BASE_URL}/{id}/info.0.json"))
            .await?
            .json::<Self>()
            .await
    }

    pub async fn latest() -> Result<Self, reqwest::Error> {
        reqwest::get(LATEST_URL).await?.json::<Self>().await
    }
}
