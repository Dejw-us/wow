use crate::widget::Widget;
use crate::window::{WindowConfig, WindowConfigStates};
use std::fs;
use std::fs::DirEntry;

pub fn to_window_entry(entry: DirEntry) -> Option<(String, (WindowConfig, WindowConfigStates))> {
  fn inner(entry: DirEntry) -> Result<(String, (WindowConfig, WindowConfigStates)), String> {
    let file_name = entry
      .file_name()
      .to_string_lossy()
      .strip_suffix(".yml")
      .ok_or("Failed to strip .yml suffix")?
      .to_string();

    let content =
      fs::read_to_string(entry.path()).map_err(|_| format!("Failed to read file {}", file_name))?;

    let states = serde_yaml::from_str::<WindowConfigStates>(&content)
      .map_err(|_| format!("Failed to read states from: {}", file_name))?;

    let config = serde_yaml::from_str(&content)
      .map_err(|e| format!("Failed to parse config file {}, err: {:?}", file_name, e))?;

    println!("Adding window : {:?}", file_name);
    Ok((file_name, (config, states)))
  }

  match inner(entry) {
    Ok(result) => Some(result),
    Err(err) => {
      println!("{}", err);
      None
    }
  }
}

pub fn to_widget_entry(f: DirEntry) -> Option<(String, Widget)> {
  let file_name = f
    .file_name()
    .to_string_lossy()
    .strip_suffix(".yml")
    .ok_or("Failed to strip .yml suffix")
    .unwrap()
    .to_string();
  let content = fs::read_to_string(f.path()).unwrap();
  let widget = serde_yaml::from_str::<Widget>(&content).unwrap();
  println!("Adding widget {:?}", &file_name);
  Some((file_name, widget))
}
