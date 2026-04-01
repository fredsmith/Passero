use crate::error::{PasseroError, Result};
use serde::Serialize;
use std::path::{Path, PathBuf};
use tokio::process::Command;

#[derive(Debug, Clone, Serialize)]
pub struct PasswordEntry {
    pub path: String,
    pub name: String,
    pub is_dir: bool,
    pub children: Vec<PasswordEntry>,
}

pub struct PassStore {
    pass_binary: PathBuf,
    store_dir: PathBuf,
}

impl PassStore {
    pub fn new(pass_binary: Option<String>, store_dir: Option<String>) -> Self {
        let pass_binary = pass_binary
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("pass"));
        let store_dir = store_dir.map(PathBuf::from).unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_default()
                .join(".password-store")
        });
        Self {
            pass_binary,
            store_dir,
        }
    }

    fn command(&self) -> Command {
        let mut cmd = Command::new(&self.pass_binary);
        cmd.env("PASSWORD_STORE_DIR", &self.store_dir);
        cmd
    }

    pub fn build_tree(&self) -> Result<Vec<PasswordEntry>> {
        let store_dir = &self.store_dir;
        if !store_dir.exists() {
            return Ok(vec![]);
        }
        let mut entries = Vec::new();
        Self::walk_dir(store_dir, store_dir, &mut entries)?;
        entries.sort_by(|a, b| {
            b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
        });
        Ok(entries)
    }

    fn walk_dir(
        base: &Path,
        dir: &Path,
        entries: &mut Vec<PasswordEntry>,
    ) -> Result<()> {
        let read_dir = std::fs::read_dir(dir)?;
        for entry in read_dir {
            let entry = entry?;
            let file_name = entry.file_name().to_string_lossy().to_string();

            // Skip hidden files/dirs (.git, .gpg-id, etc.)
            if file_name.starts_with('.') {
                continue;
            }

            let file_type = entry.file_type()?;
            let rel_path = entry
                .path()
                .strip_prefix(base)
                .unwrap_or(entry.path().as_path())
                .to_string_lossy()
                .to_string();

            if file_type.is_dir() {
                let mut children = Vec::new();
                Self::walk_dir(base, &entry.path(), &mut children)?;
                children.sort_by(|a, b| {
                    b.is_dir.cmp(&a.is_dir).then(a.name.cmp(&b.name))
                });
                entries.push(PasswordEntry {
                    path: rel_path,
                    name: file_name,
                    is_dir: true,
                    children,
                });
            } else if file_name.ends_with(".gpg") {
                let name = file_name.trim_end_matches(".gpg").to_string();
                let path = rel_path.trim_end_matches(".gpg").to_string();
                entries.push(PasswordEntry {
                    path,
                    name,
                    is_dir: false,
                    children: vec![],
                });
            }
        }
        Ok(())
    }

    pub async fn show(&self, path: &str) -> Result<String> {
        let output = self.command().args(["show", path]).output().await?;
        if !output.status.success() {
            return Err(PasseroError::PassError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    pub async fn insert(&self, path: &str, content: &str) -> Result<()> {
        let mut child = self
            .command()
            .args(["insert", "--multiline", "--force", path])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin.write_all(content.as_bytes()).await?;
            // Close stdin to signal EOF
            drop(stdin);
        }

        let output = child.wait_with_output().await?;
        if !output.status.success() {
            return Err(PasseroError::PassError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub async fn delete(&self, path: &str) -> Result<()> {
        let output = self
            .command()
            .args(["rm", "--force", path])
            .output()
            .await?;
        if !output.status.success() {
            return Err(PasseroError::PassError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        Ok(())
    }

    pub async fn generate(
        &self,
        path: &str,
        length: u32,
        no_symbols: bool,
    ) -> Result<String> {
        let mut args = vec![
            "generate".to_string(),
            "--force".to_string(),
            path.to_string(),
            length.to_string(),
        ];
        if no_symbols {
            args.push("--no-symbols".to_string());
        }

        let output = self
            .command()
            .args(&args)
            .output()
            .await?;
        if !output.status.success() {
            return Err(PasseroError::PassError(
                String::from_utf8_lossy(&output.stderr).to_string(),
            ));
        }
        // `pass generate` outputs the generated password; show the entry to get it clean
        self.show(path).await
    }
}
