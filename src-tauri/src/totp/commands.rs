use crate::error::{PasseroError, Result};
use serde::Serialize;
use std::time::SystemTime;
use totp_rs::TOTP;
use url::Url;

#[derive(Debug, Clone, Serialize)]
pub struct TotpCode {
    pub code: String,
    pub remaining_seconds: u64,
    pub period: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TotpInfo {
    pub issuer: Option<String>,
    pub account: Option<String>,
    pub uri: String,
}

/// Parse an otpauth:// URI and generate the current TOTP code.
fn generate_from_uri(uri: &str) -> Result<(TotpCode, TotpInfo)> {
    let totp = TOTP::from_url(uri)
        .map_err(|e| PasseroError::TotpError(format!("Invalid otpauth URI: {e}")))?;

    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| PasseroError::TotpError(e.to_string()))?
        .as_secs();

    let code = totp
        .generate(now)
        .to_string();

    let period = totp.step;
    let remaining = period - (now % period);

    // Extract account from the URI path
    let parsed = Url::parse(uri).ok();
    let account = parsed.as_ref().and_then(|u| {
        let path = u.path().trim_start_matches('/');
        if path.is_empty() {
            None
        } else {
            // Format is usually "issuer:account" or just "account"
            let account = if let Some((_issuer, acct)) = path.split_once(':') {
                acct
            } else {
                path
            };
            let decoded = urlencoding::decode(account).unwrap_or(account.into());
            Some(decoded.into_owned())
        }
    });

    let info = TotpInfo {
        issuer: totp.issuer.clone(),
        account,
        uri: uri.to_string(),
    };

    Ok((
        TotpCode {
            code,
            remaining_seconds: remaining,
            period,
        },
        info,
    ))
}

/// Find otpauth:// URI in password entry content.
fn find_otp_uri(content: &str) -> Option<String> {
    content
        .lines()
        .find(|line| line.trim().starts_with("otpauth://"))
        .map(|line| line.trim().to_string())
}

/// Get TOTP code for a password entry (reads the entry via pass, finds otpauth:// URI).
#[tauri::command]
pub async fn get_totp(app: tauri::AppHandle, path: String) -> Result<TotpCode> {
    let store = crate::pass::commands::make_store(&app)?;
    let content = store.show(&path).await?;

    let uri = find_otp_uri(&content)
        .ok_or_else(|| PasseroError::TotpError("No otpauth:// URI found in entry".into()))?;

    let (code, _info) = generate_from_uri(&uri)?;
    Ok(code)
}

/// Get TOTP info (issuer, account) for a password entry without generating a code.
#[tauri::command]
pub async fn get_totp_info(app: tauri::AppHandle, path: String) -> Result<Option<TotpInfo>> {
    let store = crate::pass::commands::make_store(&app)?;
    let content = store.show(&path).await?;

    let Some(uri) = find_otp_uri(&content) else {
        return Ok(None);
    };

    let (_code, info) = generate_from_uri(&uri)?;
    Ok(Some(info))
}

/// Build an otpauth:// URI from a manual setup code and add it to an existing password entry.
#[tauri::command]
pub async fn insert_totp(
    app: tauri::AppHandle,
    path: String,
    secret: String,
    issuer: Option<String>,
    account: Option<String>,
) -> Result<()> {
    let store = crate::pass::commands::make_store(&app)?;

    // Build the otpauth URI
    let account_name = account.as_deref().unwrap_or("unknown");
    let label = match &issuer {
        Some(iss) => format!("{iss}:{account_name}"),
        None => account_name.to_string(),
    };
    let mut uri = format!(
        "otpauth://totp/{}?secret={}",
        urlencoding::encode(&label),
        secret.replace(' ', "").to_uppercase()
    );
    if let Some(iss) = &issuer {
        uri.push_str(&format!("&issuer={}", urlencoding::encode(iss)));
    }

    // Validate by trying to parse it
    TOTP::from_url(&uri)
        .map_err(|e| PasseroError::TotpError(format!("Invalid TOTP parameters: {e}")))?;

    // Read existing content and append the URI
    let existing = match store.show(&path).await {
        Ok(content) => content,
        Err(_) => String::new(),
    };

    // Check if there's already an otpauth URI
    let new_content = if find_otp_uri(&existing).is_some() {
        // Replace the existing otpauth line
        existing
            .lines()
            .map(|line| {
                if line.trim().starts_with("otpauth://") {
                    uri.as_str()
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        // Append the URI
        let trimmed = existing.trim_end();
        if trimmed.is_empty() {
            uri
        } else {
            format!("{trimmed}\n{uri}")
        }
    };

    store.insert(&path, &new_content).await
}

/// Decode a QR code from an image file and extract the otpauth:// URI.
#[tauri::command]
pub async fn decode_qr_image(image_path: String) -> Result<String> {
    // Run image decoding on a blocking thread since it's CPU-bound
    tokio::task::spawn_blocking(move || {
        let img = image::open(&image_path)
            .map_err(|e| PasseroError::TotpError(format!("Failed to open image: {e}")))?;

        let gray = img.to_luma8();

        let mut prepared = rqrr::PreparedImage::prepare(gray);
        let grids = prepared.detect_grids();

        if grids.is_empty() {
            return Err(PasseroError::TotpError("No QR code found in image".into()));
        }

        let (_meta, content) = grids[0]
            .decode()
            .map_err(|e| PasseroError::TotpError(format!("Failed to decode QR: {e}")))?;

        if !content.starts_with("otpauth://") {
            return Err(PasseroError::TotpError(format!(
                "QR code does not contain an otpauth:// URI (got: {}...)",
                &content[..content.len().min(50)]
            )));
        }

        // Validate it's a valid TOTP URI
        TOTP::from_url(&content)
            .map_err(|e| PasseroError::TotpError(format!("Invalid otpauth URI in QR: {e}")))?;

        Ok(content)
    })
    .await
    .map_err(|e| PasseroError::TotpError(format!("Task failed: {e}")))?
}

/// Import TOTP from a QR code image: decode the QR, then store the URI in the password entry.
#[tauri::command]
pub async fn import_totp_from_qr(
    app: tauri::AppHandle,
    path: String,
    image_path: String,
) -> Result<TotpCode> {
    // Decode QR
    let uri = decode_qr_image(image_path).await?;

    let store = crate::pass::commands::make_store(&app)?;

    // Read existing content
    let existing = match store.show(&path).await {
        Ok(content) => content,
        Err(_) => String::new(),
    };

    // Insert/replace the otpauth URI
    let new_content = if find_otp_uri(&existing).is_some() {
        existing
            .lines()
            .map(|line| {
                if line.trim().starts_with("otpauth://") {
                    uri.as_str()
                } else {
                    line
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    } else {
        let trimmed = existing.trim_end();
        if trimmed.is_empty() {
            uri.clone()
        } else {
            format!("{trimmed}\n{uri}")
        }
    };

    store.insert(&path, &new_content).await?;

    // Generate and return the current code
    let (code, _info) = generate_from_uri(&uri)?;
    Ok(code)
}
