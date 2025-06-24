use serde_yaml::Value;

pub fn value_to_string(s: serde_yaml::Value) -> String {
  match s {
    Value::Null => String::new(),
    Value::Bool(bool) => bool.to_string(),
    Value::Number(num) => num.to_string(),
    Value::String(string) => string,
    Value::Sequence(_) => String::new(),
    Value::Mapping(_) => String::new(),
    Value::Tagged(_) => String::new(),
  }
}
