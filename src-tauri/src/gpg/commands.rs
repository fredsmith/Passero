use crate::error::{PasseroError, Result};
use serde::Serialize;
use tauri_plugin_store::StoreExt;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct GpgKey {
    pub id: String,
    pub fingerprint: String,
    pub uid: String,
    pub trust: String,
}

#[tauri::command]
pub async fn list_gpg_keys(app: tauri::AppHandle) -> Result<Vec<GpgKey>> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    let gpg_binary = store
        .get("gpg_binary")
        .and_then(|v: serde_json::Value| v.as_str().map(String::from))
        .unwrap_or_else(|| "gpg".to_string());

    let output = Command::new(&gpg_binary)
        .args(["--list-keys", "--with-colons", "--batch", "--no-tty"])
        .output()
        .await?;

    if !output.status.success() {
        return Err(PasseroError::GpgError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut keys = Vec::new();
    let mut current_fpr = String::new();
    let mut current_id = String::new();
    let mut current_trust = String::new();

    for line in stdout.lines() {
        let fields: Vec<&str> = line.split(':').collect();
        if fields.len() < 2 {
            continue;
        }

        match fields[0] {
            "pub" => {
                current_id = fields.get(4).unwrap_or(&"").to_string();
                current_trust = fields.get(1).unwrap_or(&"").to_string();
                current_fpr.clear();
            }
            "fpr" => {
                current_fpr = fields.get(9).unwrap_or(&"").to_string();
            }
            "uid" => {
                let uid = fields.get(9).unwrap_or(&"").to_string();
                if !current_id.is_empty() {
                    keys.push(GpgKey {
                        id: current_id.clone(),
                        fingerprint: current_fpr.clone(),
                        uid,
                        trust: current_trust.clone(),
                    });
                }
            }
            _ => {}
        }
    }

    Ok(keys)
}

#[tauri::command]
pub async fn get_store_gpg_id(app: tauri::AppHandle) -> Result<String> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    let store_dir = store
        .get("password_store_dir")
        .and_then(|v: serde_json::Value| v.as_str().map(String::from))
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".password-store")
                .to_string_lossy()
                .to_string()
        });

    let gpg_id_path = std::path::Path::new(&store_dir).join(".gpg-id");
    let content = tokio::fs::read_to_string(&gpg_id_path)
        .await
        .map_err(|e| PasseroError::GpgError(format!("Failed to read .gpg-id: {}", e)))?;

    Ok(content.trim().to_string())
}
