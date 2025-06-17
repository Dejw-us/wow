use crate::action::bool::BoolAction;
use crate::action::date::DateAction;
use crate::action::float::FloatAction;
use crate::action::int::IntAction;
use crate::action::log::LogAction;
use crate::action::none::NoneAction;
use crate::action::set_state::SetStateAction;
use crate::action::string::StringAction;
use crate::action::task::RepeatAction;
use crate::action::traits::TryFromRawAction;
use crate::action::{utils, Action};
use crate::functions::Mappings;
use getset::Getters;
use serde::de;

#[derive(Getters)]
pub struct RawAction {
  #[get = "pub"]
  name: String,
  params: Vec<String>,
}

impl TryInto<Action> for RawAction {
  type Error = serde_yaml::Error;

  fn try_into(self) -> Result<Action, Self::Error> {
    match self.name().as_str() {
      "log" => Ok(Action::new(LogAction::try_from_raw_action::<Self::Error>(
        self,
      )?)),
      "set" => Ok(Action::new(SetStateAction::try_from_raw_action::<
        Self::Error,
      >(self)?)),
      "str" => Ok(Action::new(
        StringAction::try_from_raw_action::<Self::Error>(self)?,
      )),
      "int" => Ok(Action::new(IntAction::try_from_raw_action::<Self::Error>(
        self,
      )?)),
      "float" => Ok(Action::new(
        FloatAction::try_from_raw_action::<Self::Error>(self)?,
      )),
      "bool" => Ok(Action::new(BoolAction::try_from_raw_action::<Self::Error>(
        self,
      )?)),
      "none" => Ok(Action::new(NoneAction::try_from_raw_action::<Self::Error>(
        self,
      )?)),
      "date" => Ok(Action::new(DateAction::try_from_raw_action::<Self::Error>(
        self,
      )?)),
      "repeat" => Ok(Action::new(
        RepeatAction::try_from_raw_action::<Self::Error>(self)?,
      )),
      _ => Err(serde::de::Error::custom(format!(
        "Invalid action name: {}",
        self.name()
      ))),
    }
  }
}

impl RawAction {
  pub fn param(&self, i: usize) -> Option<&String> {
    self.params.get(i)
  }

  pub fn de_param<E: de::Error>(&self, i: usize) -> Result<String, E> {
    match self.param(i) {
      None => Err(E::custom("param not found")),
      Some(p) => Ok(p.to_owned()),
    }
  }

  pub fn parse(s: &str) -> Result<Self, String> {
    let s = s.strip_prefix('~').ok_or("Actions need to start with ~")?;
    let open_paren = s.find('(').ok_or("Invalid syntax. Expected '('")?;
    let name = &s[..open_paren];
    let params_str = &s[(open_paren + 1)..];

    if !params_str.ends_with(')') {
      return Err("Invalid syntax. Expected ')' at the end".into());
    }

    let params_str = &params_str[..params_str.len() - 1];
    let params = utils::split_params(params_str)
      .into_iter()
      .map(Mappings::into)
      .collect();

    Ok(RawAction {
      name: name.to_string(),
      params,
    })
  }
}
