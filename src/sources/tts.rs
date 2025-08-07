#![allow(dead_code)]

use serde::Deserialize;
use serde_json::json;

// turn this into an enum
pub enum Voice {
    EnglishUSFemale,
    EnglishUSMale1,
    EnglishUSMale2,
    EnglishUSMale3,
    EnglishUSMale4,
    EnglishUKMale1,
    EnglishUKMale2,
    EnglishAUFemale,
    EnglishAUMale,
    FrenchMale1,
    FrenchMale2,
    GermanFemale,
    GermanMale,
    SpanishMale,
    SpanishMXMale1,
    SpanishMXMale2,
    SpanishMXFemale1,
    SpanishMXFemale2,
    SpanishMXFemale3,
    SpanishMXOptimusPrime,
    PortugueseBRFemale1,
    PortugueseBRFemale2,
    PortugueseBRFemale3,
    PortugueseBRMale,
    IndonesianFemale,
    JapaneseFemale1,
    JapaneseFemale2,
    JapaneseFemale3,
    JapaneseMale,
    KoreanMale1,
    KoreanMale2,
    KoreanFemale,
    CharactersGhostface,
    CharactersChewbacca,
    CharactersC3PO,
    CharactersStitch,
    CharactersStormtrooper,
    CharactersRocket,
    SingingAlto,
    SingingTenor,
    SingingSunshineSoon,
    SingingWarmyBreeze,
    SingingGlorious,
    SingingItGoesUp,
    SingingChipmunk,
    SingingDramatic,
}

impl Voice {
    pub fn list() -> Vec<(String, String)> {
        vec![
            ("English US - Female".to_string(), "en_us_001".to_string()),
            ("English US - Male 1".to_string(), "en_us_006".to_string()),
            ("English US - Male 2".to_string(), "en_us_007".to_string()),
            ("English US - Male 3".to_string(), "en_us_009".to_string()),
            ("English US - Male 4".to_string(), "en_us_010".to_string()),
            ("English UK - Male 1".to_string(), "en_uk_001".to_string()),
            ("English UK - Male 2".to_string(), "en_uk_003".to_string()),
            ("English AU - Female".to_string(), "en_au_001".to_string()),
            ("English AU - Male".to_string(), "en_au_002".to_string()),
            ("French - Male 1".to_string(), "fr_001".to_string()),
            ("French - Male 2".to_string(), "fr_002".to_string()),
            ("German - Female".to_string(), "de_001".to_string()),
            ("German - Male".to_string(), "de_002".to_string()),
            ("Spanish - Male".to_string(), "es_002".to_string()),
            ("Spanish MX - Male 1".to_string(), "es_mx_002".to_string()),
            ("Spanish MX - Male 2".to_string(), "es_male_m3".to_string()),
            (
                "Spanish MX - Female 1".to_string(),
                "es_female_f6".to_string(),
            ),
            (
                "Spanish MX - Female 2".to_string(),
                "es_female_fp1".to_string(),
            ),
            (
                "Spanish MX - Female 3".to_string(),
                "es_mx_female_supermom".to_string(),
            ),
            (
                "Spanish MX - Optimus Prime (Transformers)".to_string(),
                "es_mx_male_transformer".to_string(),
            ),
            ("Portuguese BR - Female 1".to_string(), "br_001".to_string()),
            ("Portuguese BR - Female 2".to_string(), "br_003".to_string()),
            ("Portuguese BR - Female 3".to_string(), "br_004".to_string()),
            ("Portuguese BR - Male".to_string(), "br_005".to_string()),
            ("Indonesian - Female".to_string(), "id_001".to_string()),
            ("Japanese - Female 1".to_string(), "jp_001".to_string()),
            ("Japanese - Female 2".to_string(), "jp_003".to_string()),
            ("Japanese - Female 3".to_string(), "jp_005".to_string()),
            ("Japanese - Male".to_string(), "jp_006".to_string()),
            ("Korean - Male 1".to_string(), "kr_002".to_string()),
            ("Korean - Male 2".to_string(), "kr_004".to_string()),
            ("Korean - Female".to_string(), "kr_003".to_string()),
            (
                "Characters - Ghostface (Scream)".to_string(),
                "en_us_ghostface".to_string(),
            ),
            (
                "Characters - Chewbacca (Star Wars)".to_string(),
                "en_us_chewbacca".to_string(),
            ),
            (
                "Characters - C3PO (Star Wars)".to_string(),
                "en_us_c3po".to_string(),
            ),
            (
                "Characters - Stitch (Lilo &amp; Stitch)".to_string(),
                "en_us_stitch".to_string(),
            ),
            (
                "Characters - Stormtrooper (Star Wars)".to_string(),
                "en_us_stormtrooper".to_string(),
            ),
            (
                "Characters - Rocket (Guardians of the Galaxy)".to_string(),
                "en_us_rocket".to_string(),
            ),
            (
                "Singing - Alto".to_string(),
                "en_female_f08_salut_damour".to_string(),
            ),
            (
                "Singing - Tenor".to_string(),
                "en_male_m03_lobby".to_string(),
            ),
            (
                "Singing - Sunshine Soon".to_string(),
                "en_male_m03_sunshine_soon".to_string(),
            ),
            (
                "Singing - Warmy Breeze".to_string(),
                "en_female_f08_warmy_breeze".to_string(),
            ),
            (
                "Singing - Glorious".to_string(),
                "en_female_ht_f08_glorious".to_string(),
            ),
            (
                "Singing - It Goes Up".to_string(),
                "en_male_sing_funny_it_goes_up".to_string(),
            ),
            (
                "Singing - Chipmunk".to_string(),
                "en_male_m2_xhxs_m03_silly".to_string(),
            ),
            (
                "Singing - Dramatic".to_string(),
                "en_female_ht_f08_wonderful_world".to_string(),
            ),
        ]
    }

    pub fn value(&self) -> &str {
        match self {
            Self::EnglishUSFemale => "en_us_001",
            Self::EnglishUSMale1 => "en_us_006",
            Self::EnglishUSMale2 => "en_us_007",
            Self::EnglishUSMale3 => "en_us_009",
            Self::EnglishUSMale4 => "en_us_010",
            Self::EnglishUKMale1 => "en_uk_001",
            Self::EnglishUKMale2 => "en_uk_003",
            Self::EnglishAUFemale => "en_au_001",
            Self::EnglishAUMale => "en_au_002",
            Self::FrenchMale1 => "fr_001",
            Self::FrenchMale2 => "fr_002",
            Self::GermanFemale => "de_001",
            Self::GermanMale => "de_002",
            Self::SpanishMale => "es_002",
            Self::SpanishMXMale1 => "es_mx_002",
            Self::SpanishMXMale2 => "es_male_m3",
            Self::SpanishMXFemale1 => "es_female_f6",
            Self::SpanishMXFemale2 => "es_female_fp1",
            Self::SpanishMXFemale3 => "es_mx_female_supermom",
            Self::SpanishMXOptimusPrime => "es_mx_male_transformer",
            Self::PortugueseBRFemale1 => "br_001",
            Self::PortugueseBRFemale2 => "br_003",
            Self::PortugueseBRFemale3 => "br_004",
            Self::PortugueseBRMale => "br_005",
            Self::IndonesianFemale => "id_001",
            Self::JapaneseFemale1 => "jp_001",
            Self::JapaneseFemale2 => "jp_003",
            Self::JapaneseFemale3 => "jp_005",
            Self::JapaneseMale => "jp_006",
            Self::KoreanMale1 => "kr_002",
            Self::KoreanMale2 => "kr_004",
            Self::KoreanFemale => "kr_003",
            Self::CharactersGhostface => "en_us_ghostface",
            Self::CharactersChewbacca => "en_us_chewbacca",
            Self::CharactersC3PO => "en_us_c3po",
            Self::CharactersStitch => "en_us_stitch",
            Self::CharactersStormtrooper => "en_us_stormtrooper",
            Self::CharactersRocket => "en_us_rocket",
            Self::SingingAlto => "en_female_f08_salut_damour",
            Self::SingingTenor => "en_male_m03_lobby",
            Self::SingingSunshineSoon => "en_male_m03_sunshine_soon",
            Self::SingingWarmyBreeze => "en_female_f08_warmy_breeze",
            Self::SingingGlorious => "en_female_ht_f08_glorious",
            Self::SingingItGoesUp => "en_male_sing_funny_it_goes_up",
            Self::SingingChipmunk => "en_male_m2_xhxs_m03_silly",
            Self::SingingDramatic => "en_female_ht_f08_wonderful_world",
        }
    }
}

#[derive(Deserialize)]
struct TTSResponse {
    success: bool,
    data: String,
}

pub async fn generate(text: &str, voice: &str) -> Result<String, reqwest::Error> {
    let json = reqwest::Client::new()
        .post("https://tiktok-tts.weilnet.workers.dev/api/generation")
        .json(&json!({
            "text": text,
            "voice": voice,
        }))
        .send()
        .await?
        .json::<TTSResponse>()
        .await?;

    Ok(json.data)
}
