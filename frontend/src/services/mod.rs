//! API services

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FormData, Request, RequestInit, RequestMode, Response};

const API_BASE: &str = "http://127.0.0.1:3000";

pub async fn upload_image(file: web_sys::File) -> Result<shared::UploadResponse, String> {
    let form = FormData::new().map_err(|_| "failed to build form data".to_string())?;
    form.append_with_blob_and_filename("file", &file, &file.name())
        .map_err(|_| "failed to append file".to_string())?;

    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);
    let body = JsValue::from(form);
    init.set_body(&body);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/upload", API_BASE),
        &init,
    )
    .map_err(|_| "failed to build request".to_string())?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|err| format!("invalid response: {}", err))
}

pub async fn analyze_image(id: uuid::Uuid) -> Result<shared::AnalysisResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}/analyze", API_BASE, id),
        &init,
    )
    .map_err(|_| "failed to build request".to_string())?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|err| format!("invalid response: {}", err))
}

pub async fn fetch_analysis(id: uuid::Uuid) -> Result<shared::AnalysisResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}", API_BASE, id),
        &init,
    )
    .map_err(|_| "failed to build request".to_string())?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|err| format!("invalid response: {}", err))
}

async fn send_request(request: Request) -> Result<Response, String> {
    let window = web_sys::window().ok_or_else(|| "missing window".to_string())?;
    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| "request failed".to_string())?;
    let response: Response = response_value
        .dyn_into()
        .map_err(|_| "invalid response".to_string())?;

    if !response.ok() {
        let status = response.status();
        let body = read_response_text(&response).await.unwrap_or_default();
        let message = if body.is_empty() {
            "request failed".to_string()
        } else {
            body
        };
        return Err(format!("HTTP {}: {}", status, message));
    }

    Ok(response)
}

async fn read_response_text(response: &Response) -> Result<String, String> {
    let text_promise = response
        .text()
        .map_err(|_| "failed to read response".to_string())?;
    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|_| "failed to read response".to_string())?;
    text_value
        .as_string()
        .ok_or_else(|| "response is not text".to_string())
}
