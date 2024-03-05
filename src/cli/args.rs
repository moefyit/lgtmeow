use clap::{Parser, Subcommand};

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[clap(flatten)]
    pub choose: ChooseArgs,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    Choose(ChooseArgs),
    Setup(SetupArgs),
    Reset,
}

#[derive(Parser, Debug, PartialEq)]
pub struct ChooseArgs {
    #[clap(short, long)]
    pub random: bool,
    #[clap(short, long)]
    pub copy: bool,
}

#[derive(Parser, Debug, PartialEq)]
pub struct SetupArgs {
    #[clap(long)]
    pub default: bool,
}
