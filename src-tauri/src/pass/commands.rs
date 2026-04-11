use super::store::{PassStore, PasswordEntry};
use crate::error::{PasseroError, Result};
use tauri_plugin_store::StoreExt;

pub(crate) fn make_store(app: &tauri::AppHandle) -> Result<PassStore> {
    let store_dir = crate::config::commands::get_effective_store_dir(app)?;

    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    let pass_binary = store
        .get("pass_binary")
        .and_then(|v: serde_json::Value| v.as_str().map(String::from));

    Ok(PassStore::new(pass_binary, store_dir))
}

#[tauri::command]
pub async fn list_passwords(app: tauri::AppHandle) -> Result<Vec<PasswordEntry>> {
    let store = make_store(&app)?;
    store.build_tree()
}

#[tauri::command]
pub async fn show_password(app: tauri::AppHandle, path: String) -> Result<String> {
    let store = make_store(&app)?;
    store.show(&path).await
}

#[tauri::command]
pub async fn insert_password(
    app: tauri::AppHandle,
    path: String,
    content: String,
) -> Result<()> {
    let store = make_store(&app)?;
    store.insert(&path, &content).await
}

#[tauri::command]
pub async fn edit_password(
    app: tauri::AppHandle,
    path: String,
    content: String,
) -> Result<()> {
    let store = make_store(&app)?;
    store.insert(&path, &content).await
}

#[tauri::command]
pub async fn delete_password(app: tauri::AppHandle, path: String) -> Result<()> {
    let store = make_store(&app)?;
    store.delete(&path).await
}

#[tauri::command]
pub async fn generate_password(
    app: tauri::AppHandle,
    path: String,
    length: u32,
    symbols: bool,
) -> Result<String> {
    let store = make_store(&app)?;
    store.generate(&path, length, !symbols).await
}

#[tauri::command]
pub async fn list_recipients(app: tauri::AppHandle) -> Result<Vec<String>> {
    let store = make_store(&app)?;
    store.list_recipients().await
}

#[tauri::command]
pub async fn add_recipient(app: tauri::AppHandle, gpg_id: String) -> Result<Vec<String>> {
    let store = make_store(&app)?;
    let mut recipients = store.list_recipients().await?;
    if !recipients.contains(&gpg_id) {
        recipients.push(gpg_id);
    }
    store.init_store(&recipients).await?;
    Ok(recipients)
}

#[tauri::command]
pub async fn remove_recipient(app: tauri::AppHandle, gpg_id: String) -> Result<Vec<String>> {
    let store = make_store(&app)?;
    let mut recipients = store.list_recipients().await?;
    recipients.retain(|r| r != &gpg_id);
    store.init_store(&recipients).await?;
    Ok(recipients)
}

#[tauri::command]
pub async fn init_password_store(app: tauri::AppHandle, gpg_ids: Vec<String>) -> Result<()> {
    let store = make_store(&app)?;
    store.init_store(&gpg_ids).await
}

#[tauri::command]
pub async fn copy_password(app: tauri::AppHandle, path: String) -> Result<()> {
    let store = make_store(&app)?;
    let content = store.show(&path).await?;
    let password = content.lines().next().unwrap_or("").to_string();

    #[cfg(target_os = "macos")]
    let mut cmd = tokio::process::Command::new("pbcopy");
    #[cfg(target_os = "linux")]
    let mut cmd = {
        let mut c = tokio::process::Command::new("xclip");
        c.args(["-selection", "clipboard"]);
        c
    };
    #[cfg(target_os = "windows")]
    let mut cmd = tokio::process::Command::new("clip");

    let mut child = cmd.stdin(std::process::Stdio::piped()).spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        use tokio::io::AsyncWriteExt;
        stdin.write_all(password.as_bytes()).await?;
    }

    child.wait().await?;
    Ok(())
}
