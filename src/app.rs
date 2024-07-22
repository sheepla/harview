use mime;
use ratatui::{prelude::*, widgets::*};
use url;

#[derive(Debug)]
pub struct App {
    pub running: bool,
    index: usize,
    pub har: Har,
    //pub preview_widget_state: PreviewWidetState,
    pub tabbar_state: TabBarState,
}

impl App {
    pub fn init(har: Har) -> Self {
        Self {
            running: true,
            index: 0,
            tabbar_state: TabBarState::Headers,
            har: har,
        }
    }

    pub fn tick(&self) {}

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn max_index(&self) -> usize {
        self.har.log.entries.len()
    }

    pub fn update_index(&mut self, delta: i32) {
        let max = self.max_index();
        let added = self.index as i32 + delta;
        self.index = if added < 0 {
            0
        } else if added >= max as i32 {
            max - 1
        } else {
            added as usize
        }
    }
    pub fn update_index_first(&mut self) {
        self.index = 0;
    }

    pub fn update_index_last(&mut self) {
        self.index = self.har.log.entries.len() - 1
    }

    //pub fn set_preview_widget_state(&mut self, state: &PreviewWidetState) {
    //    self.preview_widget_state = state.clone();
    //}
    pub fn set_tabbar_state(&mut self, state: &TabBarState) {
        self.tabbar_state = state.clone();
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}

type Har = crate::Har;

impl Har {
    pub fn to_table_items(&self) -> Vec<TableItem> {
        self.log
            .entries
            .iter()
            .map(|entry| {
                let result = url::Url::parse(entry.request.url.as_str());

                TableItem {
                    status: entry.response.status as u16,
                    method: entry.request.method.clone(),
                    domain: {
                        match result {
                            Ok(ref url) => url.domain().unwrap_or("").to_string(),
                            Err(_) => "".to_string(),
                        }
                    },
                    file_name: {
                        match result {
                            Ok(ref url) => url.path().to_string(),
                            Err(_) => "".to_string(),
                        }
                    },
                    mime_type: entry
                        .response
                        .content
                        .mime_type
                        .clone()
                        .unwrap_or("".to_string()),
                    size: entry.response.content.size,
                }
            })
            .collect()
    }

    pub fn to_header_info(&self, index: usize) -> Option<HeaderInfo> {
        if let Some(entry) = self.log.entries.get(index) {
            return Some(HeaderInfo {
                status: entry.response.status,
                method: entry.request.method.clone(),
                http_version: entry.request.http_version.clone(),
                url: entry.request.url.clone(),
                referrer_policy: entry
                    .request
                    .headers
                    .iter()
                    .filter(|header| header.name.eq_ignore_ascii_case("Referrer-Policy"))
                    .map(|header| header.value.clone())
                    .next(),
                query_params: entry
                    .request
                    .query_string
                    .iter()
                    .map(|query| (query.name.clone(), query.value.clone()))
                    .collect(),
                req_headers: entry
                    .request
                    .headers
                    .iter()
                    .map(|header| (header.name.clone(), header.value.clone()))
                    .collect(),
                resp_headers: entry
                    .response
                    .headers
                    .iter()
                    .map(|header| (header.name.clone(), header.value.clone()))
                    .collect(),
            });
        }

        None
    }

    pub fn to_cookie_info(har: &Har, index: usize) -> Option<CookieInfo> {
        if let Some(entry) = har.log.entries.get(index) {
            return Some(CookieInfo {
                req_cookies: entry
                    .request
                    .cookies
                    .iter()
                    .map(|cookie| (cookie.name.clone(), cookie.value.clone()))
                    .collect(),
                resp_cookies: entry
                    .response
                    .cookies
                    .iter()
                    .map(|cookie| (cookie.name.clone(), cookie.value.clone()))
                    .collect(),
            });
        }

        None
    }
}
#[derive(Debug, Clone)]
pub enum TabBarState {
    Headers,
    Cookies,
    Request,
    Response,
}

pub const TABBAR_ITEMS: [TabBarState; 4] = [
    TabBarState::Headers,
    TabBarState::Cookies,
    TabBarState::Request,
    TabBarState::Response,
];

impl TabBarState {
    pub fn from_index(&self, index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Headers),
            1 => Some(Self::Cookies),
            2 => Some(Self::Request),
            3 => Some(Self::Response),
            _ => None,
        }
    }

    pub fn to_index(&self) -> usize {
        match self {
            Self::Headers => 0,
            Self::Cookies => 1,
            Self::Request => 2,
            Self::Response => 3,
        }
    }
}

impl ToString for TabBarState {
    fn to_string(&self) -> String {
        match self {
            Self::Headers => " [1] Headers ".to_string(),
            Self::Cookies => " [2] Cookies ".to_string(),
            Self::Request => " [3] Request ".to_string(),
            Self::Response => " [4] Response ".to_string(),
        }
    }
}

//#[derive(Debug, Clone)]
//pub enum PreviewWidetState {
//    Hidden,
//    Bottom,
//    Right,
//    Full,
//}

#[derive(Debug, Clone)]
pub struct TableItem {
    status: u16,
    method: String,
    domain: String,
    file_name: String,
    mime_type: String,
    size: Option<i64>,
}

impl TableItem {
    pub fn to_table_row(&self) -> ratatui::widgets::Row<'static> {
        let status_span = match self.status {
            100..=199 => Span::styled(
                self.status.to_string(),
                Style::default().fg(Color::LightBlue).bold(),
            ),
            200..=299 => Span::styled(
                self.status.to_string(),
                Style::default().fg(Color::LightGreen).bold(),
            ),
            300..=399 => Span::styled(
                self.status.to_string(),
                Style::default().fg(Color::LightCyan).bold(),
            ),
            400..=499 => Span::styled(
                self.status.to_string(),
                Style::default().fg(Color::LightYellow).bold(),
            ),
            500..=599 => Span::styled(
                self.status.to_string(),
                Style::default().fg(Color::LightMagenta).bold(),
            ),
            0 => Span::styled("---", Style::default().fg(Color::DarkGray).bold()),
            _ => Span::styled(
                self.status.to_string(),
                Style::default().bg(Color::DarkGray),
            ),
        };

        let mime_type = self.mime_type.clone();
        let shorten_mime = match mime_type.as_str().parse::<mime::Mime>() {
            Ok(m) => m.subtype().to_string(),
            Err(_) => mime_type,
        };

        let size_span = match self.size {
            Some(s) => {
                let b = byte_unit::Byte::from_u64(s as u64);
                Span::styled(
                    format!(
                        "{:>8.2} {:<2}",
                        b.get_appropriate_unit(byte_unit::UnitType::Decimal)
                            .get_value(),
                        b.get_appropriate_unit(byte_unit::UnitType::Decimal)
                            .get_unit()
                    ),
                    Style::default(),
                )
            }
            None => Span::styled("     --- B", Style::default().fg(Color::DarkGray)),
        };

        Row::new([
            Cell::new(status_span),
            Cell::new(Span::styled(
                self.method.clone(),
                Style::default().fg(Color::White).bold(),
            )),
            Cell::new(Span::styled(
                self.domain.clone(),
                Style::default().fg(Color::White),
            )),
            Cell::new(self.file_name.clone()),
            Cell::new(shorten_mime),
            Cell::new(size_span),
        ])
    }
}

pub const TABLES_ROWS_COUNT: usize = 6;

#[derive(Debug)]
pub enum TabItem {
    Headers,
    Cookie,
    Request,
    Response,
    Timing,
    Encryption,
}

#[derive(Debug)]
pub struct HeaderInfo {
    status: i64,
    method: String,
    http_version: String,
    url: url::Url,
    query_params: Vec<(String, String)>,
    referrer_policy: Option<String>,
    req_headers: Vec<(String, String)>,
    resp_headers: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct CookieInfo {
    req_cookies: Vec<(String, String)>,
    resp_cookies: Vec<(String, String)>,
}
