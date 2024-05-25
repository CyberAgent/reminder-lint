use crate::args::InitCommand;
use crate::print::{pretty_print, Status};
use anyhow::Error;

const PATH: &str = "remind.yaml";
const CONFIG: &str = r"comment_regex: 'remind:\W?'
datetime_format: '%Y/%m/%d'
search_directory: .";

pub fn execute_init(_command: InitCommand) -> Result<(), Error> {
    if std::path::Path::new(PATH).exists() {
        pretty_print(format!("Error: {} already exists", PATH), Status::Error);
        std::process::exit(1);
    }

    std::fs::write(PATH, CONFIG)?;

    Ok(())
}
