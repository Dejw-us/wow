use crate::functions::Errors;
use std::fs::DirEntry;
use std::path::Path;
use std::{env, fs};

pub fn env(var: &str) -> crate::Result<String> {
  env::var(var).map_err(|_| crate::Error::EnvVarNotFound(var.to_string()))
}

pub fn read_files(path: &Path) -> crate::Result<impl Iterator<Item = DirEntry>> {
  Ok(
    fs::read_dir(path)
      .map_err(Errors::unknown)?
      .filter_map(Result::ok)
      .filter(|e| e.path().is_file()),
  )
}

pub fn strip_yml(s: &str) -> &str {
  s.strip_suffix(".yml")
    .unwrap_or_else(|| s.strip_suffix(".yaml").unwrap_or_else(|| s))
}
