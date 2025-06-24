use std::fs;
use std::path::{Path, PathBuf};

/// Returns config directories - (config dir, windows dir, widgets dir)
pub fn get_config_paths() -> crate::Result<(PathBuf, PathBuf, PathBuf)> {
  let config_dir = dirs::config_dir()
    .and_then(|path| Some(path.join("wow")))
    .ok_or_else(|| crate::Error::ConfigDirNotFound)?;
  let windows_dir = PathBuf::from(&config_dir).join("windows");
  let widgets_dir = PathBuf::from(&config_dir).join("widgets");

  create_dir_if_not_exists(&config_dir)?;
  create_dir_if_not_exists(&windows_dir)?;
  create_dir_if_not_exists(&widgets_dir)?;

  Ok((config_dir, windows_dir, widgets_dir))
}

fn create_dir_if_not_exists(path: &Path) -> crate::Result<()> {
  if path.exists() {
    fs::create_dir(&path).map_err(|_| {
      crate::Error::FailedToCreateConfigDir(
        path
          .as_os_str()
          .to_str()
          .unwrap_or("Invalid UTF-8")
          .to_string(),
      )
    })?;
  }
  Ok(())
}
