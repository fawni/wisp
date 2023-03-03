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
