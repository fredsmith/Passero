use crate::error::{PasseroError, Result};
use serde::Serialize;
use tauri_plugin_store::StoreExt;
use tokio::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct GitLogEntry {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

fn store_dir(app: &tauri::AppHandle) -> Result<String> {
    crate::config::commands::get_effective_store_dir(app).map(|opt| {
        opt.unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".password-store")
                .to_string_lossy()
                .to_string()
        })
    })
}

fn git_cmd(binary: &str) -> Command {
    let mut cmd = Command::new(binary);
    cmd.env("PATH", crate::path::augmented_path());
    cmd
}

fn git_binary(app: &tauri::AppHandle) -> Result<String> {
    let store = app
        .store("config.json")
        .map_err(|e: tauri_plugin_store::Error| PasseroError::ConfigError(e.to_string()))?;

    Ok(store
        .get("git_binary")
        .and_then(|v: serde_json::Value| v.as_str().map(String::from))
        .unwrap_or_else(|| "git".to_string()))
}

#[tauri::command]
pub async fn git_pull(app: tauri::AppHandle) -> Result<String> {
    let dir = store_dir(&app)?;
    let git = git_binary(&app)?;

    let output = git_cmd(&git)
        .args(["pull"])
        .current_dir(&dir)
        .output()
        .await?;

    if !output.status.success() {
        return Err(PasseroError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub async fn git_push(app: tauri::AppHandle) -> Result<String> {
    let dir = store_dir(&app)?;
    let git = git_binary(&app)?;

    let output = git_cmd(&git)
        .args(["push"])
        .current_dir(&dir)
        .output()
        .await?;

    if !output.status.success() {
        return Err(PasseroError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub async fn git_log(app: tauri::AppHandle, count: Option<u32>) -> Result<Vec<GitLogEntry>> {
    let dir = store_dir(&app)?;
    let git = git_binary(&app)?;
    let count = count.unwrap_or(20);

    let output = git_cmd(&git)
        .args([
            "log",
            &format!("-{}", count),
            "--pretty=format:%H%n%s%n%an%n%ai%n---",
        ])
        .current_dir(&dir)
        .output()
        .await?;

    if !output.status.success() {
        return Err(PasseroError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut entries = Vec::new();

    for chunk in stdout.split("---\n") {
        let lines: Vec<&str> = chunk.trim().lines().collect();
        if lines.len() >= 4 {
            entries.push(GitLogEntry {
                hash: lines[0].to_string(),
                message: lines[1].to_string(),
                author: lines[2].to_string(),
                date: lines[3].to_string(),
            });
        }
    }

    Ok(entries)
}

#[tauri::command]
pub async fn git_clone(app: tauri::AppHandle, url: String, path: Option<String>) -> Result<()> {
    let git = git_binary(&app)?;
    let target = path.unwrap_or_else(|| {
        dirs::home_dir()
            .unwrap_or_default()
            .join(".password-store")
            .to_string_lossy()
            .to_string()
    });

    let output = git_cmd(&git)
        .args(["clone", &url, &target])
        .output()
        .await?;

    if !output.status.success() {
        return Err(PasseroError::GitError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    Ok(())
}
