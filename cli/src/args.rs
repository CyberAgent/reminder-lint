use argh::FromArgs;

#[derive(FromArgs, PartialEq, Debug)]
/// Args
pub struct Args {
    #[argh(subcommand)]
    pub command: Subcommand,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum Subcommand {
    Run(RunCommand),
    Init(InitCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "run")]
/// run reminder-lint with a path
pub struct RunCommand {
    /// path to the config file (default: ./remind.yml)
    #[argh(option, short = 'c')]
    pub config_file_path: Option<String>,
    /// path to the ignore file (default: ./.remindignore)
    #[argh(option, short = 'i')]
    pub ignore_file_path: Option<String>,
    /// sort by deadline (default: false)
    #[argh(option)]
    pub sort_by_deadline: Option<bool>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "init")]
/// initialize a config of reminder-lint
pub struct InitCommand {}

impl Args {
    pub fn new() -> Self {
        argh::from_env()
    }
}
