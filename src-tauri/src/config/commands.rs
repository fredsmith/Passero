use super::model::AppConfig;
use crate::error::{PasseroError, Result};
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn get_config(app: tauri::AppHandle) -> Result<AppConfig> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    let config = AppConfig {
        pass_binary: store
            .get("pass_binary")
            .and_then(|v: serde_json::Value| v.as_str().map(String::from)),
        gpg_binary: store
            .get("gpg_binary")
            .and_then(|v: serde_json::Value| v.as_str().map(String::from)),
        git_binary: store
            .get("git_binary")
            .and_then(|v: serde_json::Value| v.as_str().map(String::from)),
        password_store_dir: store
            .get("password_store_dir")
            .and_then(|v: serde_json::Value| v.as_str().map(String::from)),
        clipboard_timeout: store
            .get("clipboard_timeout")
            .and_then(|v: serde_json::Value| v.as_u64())
            .unwrap_or(45) as u32,
    };

    Ok(config)
}

#[tauri::command]
pub async fn set_config(app: tauri::AppHandle, config: AppConfig) -> Result<()> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    if let Some(ref v) = config.pass_binary {
        store.set("pass_binary", serde_json::json!(v));
    } else {
        store.delete("pass_binary");
    }
    if let Some(ref v) = config.gpg_binary {
        store.set("gpg_binary", serde_json::json!(v));
    } else {
        store.delete("gpg_binary");
    }
    if let Some(ref v) = config.git_binary {
        store.set("git_binary", serde_json::json!(v));
    } else {
        store.delete("git_binary");
    }
    if let Some(ref v) = config.password_store_dir {
        store.set("password_store_dir", serde_json::json!(v));
    } else {
        store.delete("password_store_dir");
    }
    store.set("clipboard_timeout", serde_json::json!(config.clipboard_timeout));

    store
        .save()
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    Ok(())
}

#[tauri::command]
pub async fn get_password_store_path(app: tauri::AppHandle) -> Result<String> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    let path = store
        .get("password_store_dir")
        .and_then(|v: serde_json::Value| v.as_str().map(String::from))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".password-store")
                .to_string_lossy()
                .to_string()
        });

    Ok(path)
}
