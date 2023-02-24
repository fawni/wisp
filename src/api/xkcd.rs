use serde::Deserialize;

#[derive(Deserialize)]
pub struct Comic {
    pub month: String,
    pub num: u32,
    pub link: String,
    pub year: String,
    pub news: String,
    pub safe_title: String,
    pub transcript: String,
    pub alt: String,
    pub img: String,
    pub title: String,
    pub day: String,
}
