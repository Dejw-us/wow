use clap::Parser;
use getset::Getters;

#[derive(Parser, Getters)]
#[get = "pub"]
#[command(name = "wow")]
pub struct Cli {
  #[arg(short = 'o', long = "open")]
  open: Option<String>,
  #[arg(short = 'c', long = "close")]
  close: Option<String>,
}
