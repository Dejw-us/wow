use wow_core::config::window::WindowConfigDir;

#[test]
fn test_reading_from_config_dir() {
  let config_dir = WindowConfigDir::read().expect("Failed to read window config");

  assert_eq!(config_dir.windows().len(), 2);
  assert!(config_dir.windows().get("task_bar").is_some());
  assert!(config_dir.windows().get("example").is_some());
  assert!(config_dir.windows().get("none").is_none());
}
