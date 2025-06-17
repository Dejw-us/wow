use wow_core::action::raw::RawAction;

#[test]
fn test_raw_action() {
  let text = "~test(12, welcome, true)";
  let raw_action = RawAction::parse(text).expect("Failed to parse action");

  assert_eq!(raw_action.name(), "test");
  assert_eq!(raw_action.param(0).expect("Failed to get param"), "12");
  assert_eq!(raw_action.param(1).expect("Failed to get param"), "welcome");
  assert_eq!(raw_action.param(2).expect("Failed to get param"), "true");
}
