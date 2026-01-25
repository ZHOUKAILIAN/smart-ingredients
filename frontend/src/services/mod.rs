//! API services

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FormData, Headers, Request, RequestInit, RequestMode, Response};

use crate::stores::ToastLevel;
use crate::utils::emit_toast;
use crate::utils::error_messages::{map_api_error, map_client_error};

const DEFAULT_API_BASE: &str = "http://127.0.0.1:3000";
const API_BASE: &str = match option_env!("API_BASE") {
    Some(value) if !value.is_empty() => value,
    _ => DEFAULT_API_BASE,
};

pub async fn upload_image(file: web_sys::File) -> Result<shared::UploadResponse, String> {
    let form = FormData::new().map_err(|_| map_client_error("form_data"))?;
    form.append_with_blob_and_filename("file", &file, &file.name())
        .map_err(|_| map_client_error("append_file"))?;

    let mut opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(web_sys::RequestMode::Cors);
    opts.set_body(&form);
    // 不要手动设置 Content-Type！让浏览器自动设置 multipart/form-data 的 boundary

    let request = web_sys::Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/upload", API_BASE),
        &opts,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn confirm_and_analyze(
    id: uuid::Uuid,
    confirmed_text: String,
    preference: Option<String>,
) -> Result<shared::AnalysisResponse, String> {
    let payload = shared::ConfirmRequest {
        confirmed_text,
        preference,
    };
    let body =
        serde_json::to_string(&payload).map_err(|_| map_client_error("serialize_request"))?;

    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);

    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| map_client_error("content_type"))?;
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}/confirm", API_BASE, id),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn retry_ocr(id: uuid::Uuid) -> Result<shared::AnalysisResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}/retry-ocr", API_BASE, id),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn retry_llm(id: uuid::Uuid) -> Result<shared::AnalysisResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}/retry-llm", API_BASE, id),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn fetch_analysis(id: uuid::Uuid) -> Result<shared::AnalysisResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/analysis/{}", API_BASE, id),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

async fn send_request(request: Request) -> Result<Response, String> {
    let window = web_sys::window().ok_or_else(|| map_client_error("missing_window"))?;
    let response_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| {
            emit_toast(
                ToastLevel::Error,
                "网络错误",
                "请求失败，请检查网络连接后重试",
            );
            map_client_error("network")
        })?;
    let response: Response = response_value
        .dyn_into()
        .map_err(|_| map_client_error("invalid_response"))?;

    if !response.ok() {
        let status = response.status();
        let body = read_response_text(&response).await.unwrap_or_default();
        let (title, message) = map_api_error(status, &body);
        emit_toast(ToastLevel::Error, &title, &message);
        return Err(message);
    }

    Ok(response)
}

async fn read_response_text(response: &Response) -> Result<String, String> {
    let text_promise = response
        .text()
        .map_err(|_| map_client_error("read_response"))?;
    let text_value = JsFuture::from(text_promise)
        .await
        .map_err(|_| map_client_error("read_response"))?;
    text_value
        .as_string()
        .ok_or_else(|| map_client_error("invalid_response"))
}
