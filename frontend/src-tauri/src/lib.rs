// Library entry point for Tauri mobile apps

use base64::Engine;
use tauri::Manager;

/// Save a base64-encoded PNG image to the device.
/// On Android, saves to app cache dir (no permissions required).
/// Returns the file path on success.
#[tauri::command]
fn save_image_file(app: tauri::AppHandle, base64_data: String, filename: String) -> Result<String, String> {
    println!("[SI-NATIVE] save_image_file called, filename={}, data_len={}", filename, base64_data.len());

    let bytes = base64::engine::general_purpose::STANDARD
        .decode(&base64_data)
        .map_err(|e| {
            let msg = format!("Base64 解码失败: {}", e);
            eprintln!("[SI-NATIVE] {}", msg);
            msg
        })?;

    println!("[SI-NATIVE] decoded {} bytes", bytes.len());

    // Try download dir first, fall back to app cache dir
    let dir = app
        .path()
        .download_dir()
        .inspect(|p| println!("[SI-NATIVE] using download_dir: {:?}", p))
        .or_else(|e| {
            println!("[SI-NATIVE] download_dir failed: {}, trying app_cache_dir", e);
            app.path().app_cache_dir()
        })
        .map_err(|e| {
            let msg = format!("获取目录失败: {}", e);
            eprintln!("[SI-NATIVE] {}", msg);
            msg
        })?;

    std::fs::create_dir_all(&dir).map_err(|e| {
        let msg = format!("创建目录失败: {}", e);
        eprintln!("[SI-NATIVE] {}", msg);
        msg
    })?;

    let path = dir.join(&filename);
    std::fs::write(&path, &bytes).map_err(|e| {
        let msg = format!("写入文件失败: {}", e);
        eprintln!("[SI-NATIVE] {}", msg);
        msg
    })?;

    println!("[SI-NATIVE] image saved to: {}", path.display());
    Ok(path.display().to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_image_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
