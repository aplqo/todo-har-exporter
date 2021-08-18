extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use serde_json::from_reader;
use std::io::Read;

#[derive(Deserialize)]
struct Request {
    url: String,
}

#[derive(Deserialize)]
struct Content {
    #[serde(rename = "mimeType")]
    mime_type: String,
    text: Option<String>,
}
#[derive(Deserialize)]
struct Response {
    content: Content,
}

#[derive(Deserialize)]
struct Entry {
    request: Request,
    response: Response,
}
#[derive(Deserialize)]
struct Log {
    entries: Vec<Entry>,
}
#[derive(Deserialize)]
struct Har {
    log: Log,
}

pub struct APIResult {
    pub url: String,
    pub result: String,
}
pub fn load_har<T: Read>(har: T) -> Vec<APIResult> {
    from_reader::<T, Har>(har)
        .unwrap()
        .log
        .entries
        .into_iter()
        .filter_map(|x| {
            if x.response.content.mime_type == "application/json"
                && x.response.content.text.is_some()
            {
                Some(APIResult {
                    url: x.request.url,
                    result: x.response.content.text.unwrap(),
                })
            } else {
                None
            }
        })
        .collect()
}
