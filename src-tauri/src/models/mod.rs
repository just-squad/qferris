use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TabData {
    pub tabs: Vec<String>, // пока просто список, потом сделаем структуру
}

pub struct AppState {
    pub tabs_file: PathBuf,
}
