use crate::utils::{read_files, strip_yml};
use crate::{err, utils};
use getset::Getters;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct WindowConfig {}

impl TryFrom<DirEntry> for WindowConfig {
  type Error = crate::Error;

  fn try_from(value: DirEntry) -> Result<Self, Self::Error> {
    let file_name = value.file_name();
    let file_name = file_name.to_str().unwrap_or("Invalid UTF-8");

    let content =
      fs::read_to_string(value.path()).map_err(|_| err!("Failed to read file {}", file_name))?;
    let config = serde_yaml::from_str(&content)
      .map_err(|e| err!("Failed to parse config file {}, err: {}", file_name, e))?;

    Ok(config)
  }
}

#[derive(Getters)]
#[get = "pub"]
pub struct WindowConfigDir {
  windows: HashMap<String, WindowConfig>,
}

impl WindowConfigDir {
  pub fn read() -> crate::Result<WindowConfigDir> {
    let home = utils::env("HOME")?;
    let path_buf = PathBuf::from(home)
      .join(".config")
      .join("wow")
      .join("windows");

    let windows = read_files(path_buf.as_path())?
      .filter_map(Self::map_windows)
      .collect();

    Ok(WindowConfigDir { windows })
  }

  fn map_windows(e: DirEntry) -> Option<(String, WindowConfig)> {
    let name = e.file_name();
    let name = name.to_str().map(strip_yml)?;
    let config = WindowConfig::try_from(e).ok()?;
    Some((name.to_string(), config))
  }
}
