use serde::Deserialize;

pub async fn get_catalog(board: &str) -> Result<Catalog, reqwest::Error> {
    reqwest::Client::new()
        .get(format!("https://a.4cdn.org/{board}/catalog.json"))
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0",
        )
        .send()
        .await?
        .error_for_status()?
        .json::<Catalog>()
        .await
}

pub type Catalog = Vec<Page>;

#[derive(Debug, Deserialize, Clone)]
pub struct Page {
    pub threads: Vec<CatalogThread>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CatalogThread {
    pub no: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Thread {
    pub posts: Vec<Post>,
}

impl Thread {
    pub async fn from(board: &str, thread_no: u32) -> Result<Self, reqwest::Error> {
        reqwest::Client::new()
            .get(format!(
                "https://a.4cdn.org/{board}/thread/{thread_no}.json"
            ))
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0",
            )
            .send()
            .await?
            .error_for_status()?
            .json::<Self>()
            .await
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Post {
    pub no: u32,
    pub time: i64,
    pub ext: Option<String>,
    pub tim: Option<u64>,
    pub filename: Option<String>,
    pub sticky: Option<u8>,
}

impl Post {
    pub const fn is_sticky(&self) -> bool {
        self.sticky.is_some()
    }

    pub fn is_webm(&self) -> bool {
        self.ext.is_some() && self.ext.as_ref().unwrap() == ".webm"
    }

    pub fn is_image(&self) -> bool {
        self.ext.is_some() && !self.is_webm()
    }
}
