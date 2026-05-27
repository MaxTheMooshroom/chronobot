
use anyhow::{anyhow, Result};

use std::path::{PathBuf, Path};

pub fn exe_dir() -> Result<PathBuf> {
    std::env::current_exe()
        .map(|mut exe| {
            exe.pop();
            exe
        })
        .map_err(|e| anyhow!("Failed to find current executable's path"))
}

pub fn cwd() -> Result<PathBuf> {
    std::env::current_dir()
        .map_err(|e| anyhow::anyhow!("Failed to find current working directory"))
}

pub fn path_in_cwd_or_exedir<P: AsRef<Path>>(path: P) -> Result<PathBuf> {
    let path: &Path = path.as_ref();

    if let Ok(exedir) = exe_dir() {
        let exe_sibling = exedir.join(path);
        if exe_sibling.exists() {
            return Ok(exe_sibling);
        }
    }

    if let Ok(cwd_) = cwd() {
        let cwd_child = cwd_.join(path);
        if cwd_child.exists() {
            return Ok(cwd_child);
        }
    }

    Err(anyhow!(
        "Failed to find '{}' relative to chronobot's exe or the current working directory",
        path.display()
    ))
}

