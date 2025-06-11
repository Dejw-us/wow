use wow::state::StateValue;

#[derive(Debug)]
pub enum Message {
  SetState(String, StateValue),
  OpenWindow(String),
}

impl Message {
  pub fn parse(raw_message: &str) -> Self {
    match raw_message {
      s if s.starts_with("open") => Self::OpenWindow(s[5..].trim().to_string()),
      s if s.starts_with("set-state") => {
        let parts: Vec<&str> = s.split(" ").collect();
        Self::SetState(parts[1].into(), StateValue::String(parts[2].into()))
      }
      _ => panic!("unable to parse message"),
    }
  }
}
