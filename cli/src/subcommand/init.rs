use crate::args::InitCommand;

const PATH: &str = "remind.yaml";
const CONFIG: &str = r"comment_regex: 'remind:\W?'
datetime_format: '%Y/%m/%d'
search_directory: .";

pub fn execute_init(_command: InitCommand) {
    if std::path::Path::new(PATH).exists() {
        eprintln!("Error: {} already exists", PATH);
        std::process::exit(1);
    }

    std::fs::write(PATH, CONFIG).unwrap();
}
