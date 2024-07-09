use serde::{Deserialize, Deserializer};
use std::{fs, io::BufReader, path::Path};
use url::Url;

impl Har {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let file = fs::File::open(path)?;
        let reader = BufReader::new(file);
        let har = serde_json::from_reader(reader)?;

        Ok(har)
    }
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    Url::parse(&s).map_err(serde::de::Error::custom)
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Har {
    pub log: Log,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub version: Option<String>,
    pub creator: Option<Creator>,
    pub browser: Option<Browser>,
    pub pages: Option<Vec<Page>>,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub name: Option<String>,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub id: String,
    pub page_timings: PageTimings,
    pub started_date_time: String,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageTimings {
    pub on_content_load: i64,
    pub on_load: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub started_date_time: String,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    pub time: f64,
    #[serde(rename = "_securityState")]
    pub security_state: Option<String>,
    pub pageref: Option<String>,
    #[serde(rename = "serverIPAddress")]
    pub server_ipaddress: Option<String>,
    pub connection: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub body_size: i64,
    pub method: String,
    #[serde(deserialize_with = "deserialize_url")]
    pub url: url::Url,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub query_string: Vec<QueryString>,
    pub headers_size: i64,
    pub post_data: Option<PostData>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cookie {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryString {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub mime_type: String,
    pub params: Option<Vec<Param>>,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Param {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: i64,
    pub status_text: String,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Cookie>,
    pub content: Content,
    #[serde(rename = "redirectURL")]
    pub redirect_url: String,
    pub headers_size: i64,
    pub body_size: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header2 {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub mime_type: Option<String>,
    pub size: Option<i64>,
    pub text: Option<String>,
    pub encoding: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub blocked: Option<f64>,
    pub dns: Option<f64>,
    pub ssl: Option<f64>,
    pub connect: Option<f64>,
    pub send: Option<f64>,
    pub wait: Option<f64>,
    pub receive: Option<f64>,
}
