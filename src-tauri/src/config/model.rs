use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vault {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub pass_binary: Option<String>,
    pub gpg_binary: Option<String>,
    pub git_binary: Option<String>,
    pub password_store_dir: Option<String>,
    pub clipboard_timeout: u32,
    #[serde(default)]
    pub vaults: Vec<Vault>,
    pub active_vault_id: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            pass_binary: None,
            gpg_binary: None,
            git_binary: None,
            password_store_dir: None,
            clipboard_timeout: 45,
            vaults: Vec::new(),
            active_vault_id: None,
        }
    }
}
