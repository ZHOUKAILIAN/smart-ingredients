//! Centralized API error message mapping.

use shared::error_codes;

pub fn map_client_error(code: &str) -> String {
    match code {
        "form_data" | "append_file" => "图片上传失败，请重新选择后再试".to_string(),
        "serialize_request" | "build_request" | "build_headers" | "content_type" => {
            "请求准备失败，请重试".to_string()
        }
        "missing_window" => "页面初始化失败，请重新打开".to_string(),
        "network" => "网络连接异常，请检查网络后重试".to_string(),
        "read_response" | "invalid_response" => "服务响应异常，请稍后重试".to_string(),
        _ => "请求失败，请稍后重试".to_string(),
    }
}

pub fn map_api_error(status: u16, body: &str) -> (String, String) {
    if let Ok(api_error) = serde_json::from_str::<shared::ApiError>(body) {
        let default_message = match api_error.code.as_str() {
            error_codes::BAD_REQUEST => "请求参数有误，请检查后重试",
            error_codes::UNSUPPORTED_MEDIA_TYPE => {
                "不支持该图片格式，请选择常见图片格式后重试"
            }
            error_codes::PAYLOAD_TOO_LARGE => "图片文件过大，请选择更小的图片",
            error_codes::NOT_FOUND => "未找到对应资源，请返回重试",
            error_codes::OCR_ERROR | error_codes::OCR_TIMEOUT => {
                "OCR 服务暂不可用，请稍后重试"
            }
            error_codes::LLM_ERROR | error_codes::LLM_TIMEOUT => {
                "分析服务暂不可用，请稍后重试"
            }
            error_codes::STORAGE_ERROR => "图片保存失败，请稍后重试",
            error_codes::INTERNAL_ERROR => "服务异常，请稍后重试",
            error_codes::SERVICE_UNAVAILABLE => "服务暂不可用，请稍后重试",
            _ => "请求失败，请稍后重试",
        };

        let message = if api_error.message.trim().is_empty() {
            default_message.to_string()
        } else {
            api_error.message
        };

        let title = match api_error.code.as_str() {
            error_codes::UNSUPPORTED_MEDIA_TYPE => "图片格式不支持",
            error_codes::PAYLOAD_TOO_LARGE => "图片过大",
            error_codes::OCR_ERROR | error_codes::OCR_TIMEOUT => "识别失败",
            error_codes::LLM_ERROR | error_codes::LLM_TIMEOUT => "分析失败",
            error_codes::STORAGE_ERROR => "保存失败",
            error_codes::NOT_FOUND => "资源不存在",
            _ => "请求失败",
        };

        return (title.to_string(), message);
    }

    if body.trim().is_empty() {
        return ("请求失败".to_string(), "服务异常，请稍后重试".to_string());
    }

    ("请求失败".to_string(), "服务异常，请稍后重试".to_string())
}
