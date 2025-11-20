use crate::path_safety::validate_path;
use crate::protocol::ResponsePayload;
use crate::state::State;
use anyhow::{anyhow, Context, Result};
use base64::{engine::general_purpose, Engine as _};
use std::path::PathBuf;
use tokio::fs::{self, File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, SeekFrom};

pub async fn set_download_root(state: &mut State, path: String) -> Result<ResponsePayload> {
    let path_buf = PathBuf::from(path);
    if !path_buf.exists() {
        return Err(anyhow!("Download root does not exist"));
    }
    let canonical = path_buf
        .canonicalize()
        .context("Failed to canonicalize download root")?;
    
    state.download_root = Some(canonical);
    Ok(ResponsePayload::Empty)
}

pub async fn ensure_dir(state: &mut State, path: String) -> Result<ResponsePayload> {
    let root = state
        .download_root
        .as_ref()
        .ok_or_else(|| anyhow!("Download root not set"))?;
    
    // For ensure_dir, the path might not exist, so validate_path might need to handle that.
    // Our validate_path implementation handles non-existing paths by checking the parent.
    // But ensure_dir might create multiple levels.
    // We should validate that the *target* path is inside the root.
    // Since validate_path returns a safe absolute path if it's inside root...
    
    let safe_path = validate_path(&path, root)?;
    
    fs::create_dir_all(&safe_path)
        .await
        .context("Failed to create directory")?;

    Ok(ResponsePayload::Empty)
}

pub async fn read_file(
    state: &mut State,
    path: String,
    offset: u64,
    length: usize,
) -> Result<ResponsePayload> {
    let root = state
        .download_root
        .as_ref()
        .ok_or_else(|| anyhow!("Download root not set"))?;
    
    let safe_path = validate_path(&path, root)?;
    
    let mut file = File::open(&safe_path).await.context("Failed to open file")?;
    
    file.seek(SeekFrom::Start(offset))
        .await
        .context("Failed to seek file")?;
    
    let mut buf = vec![0u8; length];
    let n = file.read(&mut buf).await.context("Failed to read file")?;
    
    // Resize buffer to actual read amount
    buf.truncate(n);
    
    let data = general_purpose::STANDARD.encode(&buf);
    
    Ok(ResponsePayload::Data { data })
}

pub async fn write_file(
    state: &mut State,
    path: String,
    offset: u64,
    data_b64: String,
) -> Result<ResponsePayload> {
    let root = state
        .download_root
        .as_ref()
        .ok_or_else(|| anyhow!("Download root not set"))?;
    
    let safe_path = validate_path(&path, root)?;
    
    let data = general_purpose::STANDARD
        .decode(data_b64)
        .context("Invalid base64 data")?;
    
    // Open for writing, create if not exists.
    // We don't truncate because we might be writing a chunk.
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&safe_path)
        .await
        .context("Failed to open file for writing")?;
    
    file.seek(SeekFrom::Start(offset))
        .await
        .context("Failed to seek file")?;
    
    file.write_all(&data)
        .await
        .context("Failed to write to file")?;
    
    Ok(ResponsePayload::Empty)
}

pub async fn stat_file(state: &mut State, path: String) -> Result<ResponsePayload> {
    let root = state
        .download_root
        .as_ref()
        .ok_or_else(|| anyhow!("Download root not set"))?;
    
    let safe_path = validate_path(&path, root)?;
    
    let metadata = fs::metadata(&safe_path)
        .await
        .context("Failed to get file metadata")?;
    
    let mtime = metadata
        .modified()
        .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    
    Ok(ResponsePayload::Stat {
        size: metadata.len(),
        mtime,
        is_dir: metadata.is_dir(),
    })
}
