use std::fs;

use tauri::State;

use crate::models::{AppState, HttpRequest, HttpResponse, TabData};

#[tauri::command]
pub async fn send_request(req: HttpRequest) -> Result<HttpResponse, String> {
    let client = reqwest::Client::new();
    let mut builder = match req.method.to_uppercase().as_str() {
        "GET" => client.get(&req.url),
        "POST" => client.post(&req.url),
        "PUT" => client.put(&req.url),
        "DELETE" => client.delete(&req.url),
        _ => return Err("Unsupported method".into()),
    };

    // Headers
    for (k, v) in req.headers {
        builder = builder.header(&k, &v);
    }

    // Body
    if let Some(b) = req.body {
        builder = builder.body(b);
    }

    let resp = builder.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let headers = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect::<Vec<_>>();
    let body = resp.text().await.unwrap_or_default();

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[tauri::command]
pub fn save_tabs(state: State<'_, AppState>, data: TabData) -> Result<(), String> {
    fs::write(&state.tabs_file, serde_json::to_string(&data).unwrap()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_tabs(state: State<'_, AppState>) -> Result<TabData, String> {
    if let Ok(content) = fs::read_to_string(&state.tabs_file) {
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else {
        Ok(TabData { tabs: vec![] })
    }
}
