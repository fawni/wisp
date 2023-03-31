use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Catalog {
    pub page: u8,
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
    pub fn is_sticky(&self) -> bool {
        self.sticky.is_some()
    }

    pub fn is_webm(&self) -> bool {
        self.ext.is_some() && self.ext.as_ref().unwrap() == ".webm"
    }

    pub fn is_image(&self) -> bool {
        self.ext.is_some() && !self.is_webm()
    }
}
