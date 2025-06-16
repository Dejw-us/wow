use wow_core::action::log::LogAction;
use wow_core::action::Action;

#[test]
fn deserialize_should_deserialize() {
  let raw = "~log(hello)";
  let deserialized: Action = serde_yaml::from_str(&raw).expect("Failed to deserialize");

  let inner = deserialized.clone_inner();
  let action = inner
    .as_any()
    .downcast_ref::<LogAction>()
    .expect("Expected LogAction");

  assert_eq!(action.message(), "hello");
}
