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
        cmd.env("PATH", crate::path::augmented_path());
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

    /// Read the .gpg-id file and return the list of recipient key IDs.
    pub async fn list_recipients(&self) -> Result<Vec<String>> {
        let gpg_id_path = self.store_dir.join(".gpg-id");
        if !gpg_id_path.exists() {
            return Ok(vec![]);
        }
        let content = tokio::fs::read_to_string(&gpg_id_path).await?;
        Ok(content
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect())
    }

    /// Re-initialize the store with the given GPG key IDs (runs `pass init`).
    /// This updates .gpg-id and re-encrypts all entries.
    pub async fn init_store(&self, gpg_ids: &[String]) -> Result<()> {
        if gpg_ids.is_empty() {
            return Err(PasseroError::PassError(
                "At least one GPG key ID is required".into(),
            ));
        }

        let mut args = vec!["init".to_string()];
        for id in gpg_ids {
            args.push(id.clone());
        }

        let output = self.command().args(&args).output().await?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn make_test_store(dir: &Path) -> PassStore {
        PassStore {
            pass_binary: PathBuf::from("pass"),
            store_dir: dir.to_path_buf(),
        }
    }

    #[test]
    fn build_tree_empty_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();
        assert!(tree.is_empty());
    }

    #[test]
    fn build_tree_nonexistent_dir() {
        let store = PassStore {
            pass_binary: PathBuf::from("pass"),
            store_dir: PathBuf::from("/nonexistent/path/that/does/not/exist"),
        };
        let tree = store.build_tree().unwrap();
        assert!(tree.is_empty());
    }

    #[test]
    fn build_tree_finds_gpg_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("email.gpg"), b"encrypted").unwrap();
        fs::write(tmp.path().join("bank.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        assert_eq!(tree.len(), 2);
        // Sorted alphabetically
        assert_eq!(tree[0].name, "bank");
        assert_eq!(tree[0].path, "bank");
        assert!(!tree[0].is_dir);
        assert_eq!(tree[1].name, "email");
        assert_eq!(tree[1].path, "email");
        assert!(!tree[1].is_dir);
    }

    #[test]
    fn build_tree_strips_gpg_extension() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("social/twitter.gpg"), b"").unwrap_err(); // dir doesn't exist yet
        fs::create_dir(tmp.path().join("social")).unwrap();
        fs::write(tmp.path().join("social/twitter.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        assert_eq!(tree.len(), 1);
        assert!(tree[0].is_dir);
        assert_eq!(tree[0].name, "social");
        assert_eq!(tree[0].children.len(), 1);
        assert_eq!(tree[0].children[0].name, "twitter");
        assert_eq!(tree[0].children[0].path, "social/twitter");
    }

    #[test]
    fn build_tree_skips_hidden_files_and_dirs() {
        let tmp = tempfile::tempdir().unwrap();
        // These should be ignored
        fs::create_dir(tmp.path().join(".git")).unwrap();
        fs::write(tmp.path().join(".git/config"), b"").unwrap();
        fs::write(tmp.path().join(".gpg-id"), b"0xABC123").unwrap();
        // This should be found
        fs::write(tmp.path().join("login.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "login");
    }

    #[test]
    fn build_tree_ignores_non_gpg_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("notes.txt"), b"not a password").unwrap();
        fs::write(tmp.path().join("readme.md"), b"info").unwrap();
        fs::write(tmp.path().join("actual.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        assert_eq!(tree.len(), 1);
        assert_eq!(tree[0].name, "actual");
    }

    #[test]
    fn build_tree_dirs_sorted_before_files() {
        let tmp = tempfile::tempdir().unwrap();
        fs::write(tmp.path().join("aaa.gpg"), b"encrypted").unwrap();
        fs::create_dir(tmp.path().join("zzz")).unwrap();
        fs::write(tmp.path().join("zzz/entry.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        assert_eq!(tree.len(), 2);
        // Directories come first regardless of alpha order
        assert!(tree[0].is_dir);
        assert_eq!(tree[0].name, "zzz");
        assert!(!tree[1].is_dir);
        assert_eq!(tree[1].name, "aaa");
    }

    #[test]
    fn build_tree_nested_structure() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("work/servers")).unwrap();
        fs::write(tmp.path().join("work/email.gpg"), b"encrypted").unwrap();
        fs::write(tmp.path().join("work/servers/prod.gpg"), b"encrypted").unwrap();
        fs::write(tmp.path().join("personal.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        // Top level: work/ dir first, then personal file
        assert_eq!(tree.len(), 2);
        assert!(tree[0].is_dir);
        assert_eq!(tree[0].name, "work");
        assert!(!tree[1].is_dir);
        assert_eq!(tree[1].name, "personal");

        // work/ has servers/ dir then email file
        let work = &tree[0];
        assert_eq!(work.children.len(), 2);
        assert!(work.children[0].is_dir);
        assert_eq!(work.children[0].name, "servers");
        assert!(!work.children[1].is_dir);
        assert_eq!(work.children[1].name, "email");
        assert_eq!(work.children[1].path, "work/email");

        // work/servers/ has prod
        let servers = &work.children[0];
        assert_eq!(servers.children.len(), 1);
        assert_eq!(servers.children[0].name, "prod");
        assert_eq!(servers.children[0].path, "work/servers/prod");
    }

    #[test]
    fn build_tree_empty_subdirs_not_shown() {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir(tmp.path().join("empty_folder")).unwrap();
        fs::write(tmp.path().join("login.gpg"), b"encrypted").unwrap();

        let store = make_test_store(tmp.path());
        let tree = store.build_tree().unwrap();

        // Empty dir still shows up (it's a valid folder in the store)
        assert_eq!(tree.len(), 2);
        assert!(tree[0].is_dir);
        assert_eq!(tree[0].name, "empty_folder");
        assert!(tree[0].children.is_empty());
    }
}
