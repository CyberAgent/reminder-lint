mod subcommand;

use subcommand::execute_subcommand;

use crate::args::Args;

mod args;

fn main() {
    let args: Args = Args::new();
    execute_subcommand(args.command);
}
