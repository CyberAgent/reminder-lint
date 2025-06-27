use validate::execute_validates;

use crate::{
    args::Subcommand,
    print::{pretty_print, Status},
};

use self::{init::execute_init, list::execute_list, run::execute_run};

mod init;
mod list;
mod run;
mod validate;

pub fn execute_subcommand(subcommand: Subcommand) {
    let result = match subcommand {
        Subcommand::Run(command) => execute_run(command),
        Subcommand::Init(command) => execute_init(command),
        Subcommand::List(command) => execute_list(command),
        Subcommand::Validate(command) => execute_validates(command),
    };

    if let Err(e) = result {
        pretty_print(e, Status::Error);
        std::process::exit(1);
    }
}
