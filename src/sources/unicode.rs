use std::collections::HashMap;

async fn get_unicode_map() -> HashMap<String, String> {
    let data = reqwest::get("https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let mut map = HashMap::new();

    for line in data.lines() {
        let mut parts = line.split(";");
        let code = parts.next().unwrap();
        let name = parts.next().unwrap();
        map.insert(code.to_owned(), name.to_owned());
    }

    map
}

static MAP: tokio::sync::OnceCell<HashMap<String, String>> = tokio::sync::OnceCell::const_new();

async fn unicode() -> &'static HashMap<String, String> {
    MAP.get_or_init(get_unicode_map).await
}

pub async fn get_char_unicode_name(character: char) -> Option<String> {
    unicode()
        .await
        .get(&format!("{:04X}", character as u32))
        .cloned()
}

pub async fn get_unicode_name(text: &str) -> Vec<(char, String)> {
    let mut result = Vec::new();
    for c in text.chars() {
        if let Some(name) = get_char_unicode_name(c).await {
            result.push((c, name));
        } else {
            result.push((c, "???".to_owned()));
        }
    }
    result
}
