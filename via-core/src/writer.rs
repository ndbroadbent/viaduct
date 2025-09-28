use std::{fs, path::Path};

use anyhow::{Context, Result};

use crate::codegen::GeneratedFile;

pub fn clean_output_root(out_dir: &Path) -> Result<()> {
    if out_dir.exists() {
        let src_dir = out_dir.join("src");
        if src_dir.exists() {
            fs::remove_dir_all(&src_dir).with_context(|| {
                format!("Failed to clear generated directory {}", src_dir.display())
            })?;
        }
    }
    Ok(())
}

pub fn write_files(out_dir: &Path, files: &[GeneratedFile]) -> Result<()> {
    for file in files {
        let path = out_dir.join(&file.relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {}", parent.display()))?;
        }
        fs::write(&path, file.contents.as_bytes())
            .with_context(|| format!("Failed to write {}", path.display()))?;
    }
    Ok(())
}

pub fn write_ir_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory {}", parent.display()))?;
    }
    fs::write(&path, contents).with_context(|| format!("Failed to write {}", path.display()))?;
    Ok(())
}
