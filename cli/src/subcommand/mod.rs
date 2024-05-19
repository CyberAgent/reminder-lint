use crate::args::Subcommand;

use self::{init::execute_init, run::execute_run};

mod init;
mod run;

pub fn execute_subcommand(subcommand: Subcommand) {
    match subcommand {
        Subcommand::Run(command) => execute_run(command),
        Subcommand::Init(command) => execute_init(command),
    }
}
