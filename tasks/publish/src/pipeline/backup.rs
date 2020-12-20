use anyhow::Context;
use chrono::Utc;
use std::path::{Path, PathBuf};

pub(crate) fn backup_manifest(manifest: &Path) -> anyhow::Result<()> {
    let folder = manifest
        .parent()
        .with_context(|| "manifest file has no parent")?;
    let backup_folder = make_backup_folder(folder)?;

    let name = backup_filename();
    std::fs::copy(manifest, backup_folder.join(&name))?;

    Ok(())
}

fn backup_filename() -> String {
    let date_time = Utc::now();
    let formatted_date_time = date_time.format("%F_%H-%M-%S_%6f").to_string();

    format!("Cargo.toml.{}", formatted_date_time)
}

fn make_backup_folder(path: &Path) -> anyhow::Result<PathBuf> {
    let backup_dir = path.join("backup");
    let backup_dir = backup_dir.as_path();

    if !backup_dir.exists() {
        std::fs::create_dir_all(backup_dir)?;
    }

    if !backup_dir.is_dir() {
        Err(anyhow::anyhow!(
            "backup path exists but is not a directory..."
        ))
    } else {
        std::fs::write(backup_dir.join(".gitignore"), "*")?;
        Ok(backup_dir.to_path_buf())
    }
}
