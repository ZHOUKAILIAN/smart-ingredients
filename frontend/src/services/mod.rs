//! API services

use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{FormData, Headers, Request, RequestInit, RequestMode, Response};

use crate::stores::ToastLevel;
use crate::utils::emit_toast;
use crate::utils::auth_storage;
use crate::utils::error_messages::{map_api_error, map_client_error};

const API_BASE: &str = env!("API_BASE");

pub fn resolve_media_url(value: &str) -> String {
    if value.trim().is_empty() {
        return String::new();
    }
    if value.starts_with("http://") || value.starts_with("https://") {
        return value.to_string();
    }
    if value.starts_with('/') {
        return format!("{API_BASE}{value}");
    }
    format!("{API_BASE}/{value}")
}

pub async fn upload_image(file: web_sys::File) -> Result<shared::UploadResponse, String> {
    let form = FormData::new().map_err(|_| map_client_error("form_data"))?;
    form.append_with_blob_and_filename("file", &file, &file.name())
        .map_err(|_| map_client_error("append_file"))?;

    let mut opts = web_sys::RequestInit::new();
    opts.set_method("POST");
    opts.set_mode(web_sys::RequestMode::Cors);
    opts.set_body(&form);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    opts.set_headers(&headers);
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
    apply_auth_header(&headers)?;
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
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

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
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

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
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

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

pub async fn send_sms(phone: String) -> Result<shared::SendSmsResponse, String> {
    let payload = shared::SendSmsRequest { phone };
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
        &format!("{}/api/v1/auth/sms/send", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn verify_sms(phone: String, code: String) -> Result<shared::AuthResponse, String> {
    let payload = shared::VerifySmsRequest { phone, code };
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
        &format!("{}/api/v1/auth/sms/verify", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    let auth: shared::AuthResponse =
        serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))?;
    auth_storage::save_tokens(&auth.access_token, &auth.refresh_token);
    Ok(auth)
}

pub async fn refresh_session() -> Result<shared::AuthResponse, String> {
    let Some(refresh_token) = auth_storage::load_refresh_token() else {
        return Err(map_client_error("missing_refresh_token"));
    };
    let payload = shared::RefreshRequest { refresh_token };
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
        &format!("{}/api/v1/auth/refresh", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    let auth: shared::AuthResponse =
        serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))?;
    auth_storage::save_tokens(&auth.access_token, &auth.refresh_token);
    Ok(auth)
}

pub async fn ensure_session() -> Result<Option<shared::UserProfile>, String> {
    if auth_storage::load_access_token().is_some() {
        if let Ok(profile) = fetch_profile().await {
            return Ok(Some(profile));
        }
    }

    if auth_storage::load_refresh_token().is_some() {
        if let Ok(auth) = refresh_session().await {
            return Ok(Some(auth.user));
        }
    }

    Ok(None)
}

pub async fn logout() -> Result<(), String> {
    if let Some(refresh_token) = auth_storage::load_refresh_token() {
        let payload = shared::LogoutRequest { refresh_token };
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
            &format!("{}/api/v1/auth/logout", API_BASE),
            &init,
        )
        .map_err(|_| map_client_error("build_request"))?;
        let _ = send_request(request).await?;
    }
    auth_storage::clear_tokens();
    Ok(())
}

pub async fn fetch_profile() -> Result<shared::UserProfile, String> {
    let mut init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/me", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn delete_account() -> Result<(), String> {
    let mut init = RequestInit::new();
    init.set_method("DELETE");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/me", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let _ = send_request(request).await?;
    auth_storage::clear_tokens();
    Ok(())
}

pub async fn fetch_preferences() -> Result<shared::UserPreferences, String> {
    let mut init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/preferences", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn update_preferences(
    preferences: serde_json::Value,
) -> Result<shared::UserPreferences, String> {
    let payload = shared::UpdatePreferencesRequest { preferences };
    let body =
        serde_json::to_string(&payload).map_err(|_| map_client_error("serialize_request"))?;

    let mut init = RequestInit::new();
    init.set_method("PUT");
    init.set_mode(RequestMode::Cors);

    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| map_client_error("content_type"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/preferences", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn fetch_user_history(
    page: i64,
    limit: i64,
) -> Result<shared::HistoryResponse, String> {
    let mut init = RequestInit::new();
    init.set_method("GET");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!(
            "{}/api/v1/users/history?page={}&limit={}",
            API_BASE, page, limit
        ),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn delete_history(id: uuid::Uuid) -> Result<(), String> {
    let mut init = RequestInit::new();
    init.set_method("DELETE");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/history/{}", API_BASE, id),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let _ = send_request(request).await?;
    Ok(())
}

pub async fn delete_history_batch(ids: Vec<uuid::Uuid>) -> Result<(), String> {
    let payload = shared::BatchDeleteRequest { ids };
    let body =
        serde_json::to_string(&payload).map_err(|_| map_client_error("serialize_request"))?;

    let mut init = RequestInit::new();
    init.set_method("DELETE");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| map_client_error("content_type"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/history", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let _ = send_request(request).await?;
    Ok(())
}

pub async fn migrate_local_history(
    ids: Vec<uuid::Uuid>,
) -> Result<shared::LocalHistoryMigrateResponse, String> {
    let payload = shared::LocalHistoryMigrateRequest { ids };
    let body =
        serde_json::to_string(&payload).map_err(|_| map_client_error("serialize_request"))?;

    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| map_client_error("content_type"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/history/batch", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
}

pub async fn prune_history(delete_count: i64) -> Result<shared::HistoryPruneResponse, String> {
    let payload = shared::HistoryPruneRequest { delete_count };
    let body =
        serde_json::to_string(&payload).map_err(|_| map_client_error("serialize_request"))?;

    let mut init = RequestInit::new();
    init.set_method("POST");
    init.set_mode(RequestMode::Cors);
    let headers = Headers::new().map_err(|_| map_client_error("build_headers"))?;
    headers
        .set("Content-Type", "application/json")
        .map_err(|_| map_client_error("content_type"))?;
    apply_auth_header(&headers)?;
    init.set_headers(&headers);
    init.set_body(&JsValue::from_str(&body));

    let request = Request::new_with_str_and_init(
        &format!("{}/api/v1/users/history/prune", API_BASE),
        &init,
    )
    .map_err(|_| map_client_error("build_request"))?;

    let response = send_request(request).await?;
    let body = read_response_text(&response).await?;
    serde_json::from_str(&body).map_err(|_| map_client_error("invalid_response"))
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

fn apply_auth_header(headers: &Headers) -> Result<(), String> {
    if let Some(token) = auth_storage::load_access_token() {
        headers
            .set("Authorization", &format!("Bearer {}", token))
            .map_err(|_| map_client_error("build_headers"))?;
    }
    Ok(())
}
