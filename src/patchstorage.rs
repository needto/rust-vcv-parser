extern crate serde_json;
use serde_json::{Value};
extern crate reqwest;

static URL: &'static str = "https://patchstorage.com/api/wp/v2/patches?per_page=10&categories=378";

#[derive(Serialize, Deserialize, Debug)]
struct MediaInfo {
    source_url: String
}

pub fn get_patch_list() -> Vec<String> {
    let mut resp = reqwest::get(URL).unwrap();
    assert!(resp.status().is_success());
    return parse_patch_list(resp.text().unwrap())
}

fn parse_patch_list(json: String) -> Vec<String> {
    let array: Vec<Value> = serde_json::from_str(&json).unwrap();
    let urls: Vec<String> = array.into_iter().map(|x| x["_links"]["wp:attachment"][0]["href"].as_str().unwrap().to_string()).collect();
    return urls;
}

pub fn get_patch_contents(media_info_url: String) -> Option<String>{
    let mut resp = reqwest::get(&media_info_url).unwrap();
    assert!(resp.status().is_success());
    let array: Vec<MediaInfo> = serde_json::from_str(&resp.text().unwrap()).unwrap();
    let media_info: Option<MediaInfo> = array.into_iter().find(|mi| mi.source_url.ends_with(".vcv"));

    return match media_info {
        Some(media_info) => {
            let mut resp = reqwest::get(&media_info.source_url).unwrap();
            assert!(resp.status().is_success());
            Some(resp.text().unwrap())
        },
        None => None
    }
}
