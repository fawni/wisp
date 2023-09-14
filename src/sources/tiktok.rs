use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Tiktok {
    pub description: String,
    pub video_url: String,
    pub author: VideoAuthor,
    pub statistics: VideoStatistics,
}

impl Tiktok {
    pub async fn from(id: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let api_url = format!("https://api2.musical.ly/aweme/v1/feed/?aweme_id={id}");
        let res = reqwest::get(api_url).await?.json::<ApiResponse>().await?;
        let aweme = res.aweme_list[0].clone();

        if aweme.id != id {
            return Err("Tiktok not found!".into());
        }

        Ok(Self {
            video_url: aweme.video.play_addr.url_list[0].clone(),
            description: aweme.desc,
            author: aweme.author,
            statistics: aweme.statistics,
        })
    }

    pub fn valid_urls() -> [Regex; 2] {
        const DESKTOP_REGEX: &str =
            r"https?://(?:www\.|m\.)?tiktok\.com/(?:embed|@[\w\.-]+/video|v)/(\d+)";
        const MOBILE_REGEX: &str = r"https?://(?:vm|vt)\.tiktok\.com/(\w+)";

        [
            Regex::new(DESKTOP_REGEX).unwrap(),
            Regex::new(MOBILE_REGEX).unwrap(),
        ]
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct VideoAuthor {
    #[serde(rename = "nickname")]
    pub name: String,
    #[serde(rename = "unique_id")]
    pub username: String,
    pub avatar_uri: String,
}

impl VideoAuthor {
    pub fn avatar_url(&self) -> String {
        format!(
            "https://p16-amd-va.tiktokcdn.com/origin/{}.jpeg",
            &self.avatar_uri
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct VideoStatistics {
    #[serde(rename = "digg_count")]
    likes: u32,
    #[serde(rename = "comment_count")]
    comments: u32,
    #[serde(rename = "play_count")]
    views: u32,
}

impl VideoStatistics {
    pub fn likes(&self) -> String {
        self.likes.format()
    }

    pub fn comments(&self) -> String {
        self.comments.format()
    }

    pub fn views(&self) -> String {
        self.views.format()
    }
}

#[derive(Deserialize, Debug, Clone)]
struct ApiResponse {
    aweme_list: Vec<Aweme>,
}

#[derive(Deserialize, Debug, Clone)]
struct Aweme {
    #[serde(rename = "aweme_id")]
    id: String,
    desc: String,
    author: VideoAuthor,
    video: ApiVideo,
    statistics: VideoStatistics,
}

#[derive(Deserialize, Debug, Clone)]
struct ApiVideo {
    play_addr: PlayAddr,
}

#[derive(Deserialize, Debug, Clone)]
struct PlayAddr {
    url_list: Vec<String>,
}

trait NumberFormat {
    fn format(&self) -> String;
}

impl NumberFormat for u32 {
    fn format(&self) -> String {
        let mut num = self.to_string();
        let len = num.len();

        if len > 3 {
            let mut i = len % 3;
            if i == 0 {
                i = 3;
            }
            while i < num.len() {
                num.insert(i, ',');
                i += 4;
            }
        }

        num
    }
}
