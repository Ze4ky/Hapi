pub mod model;
pub mod api;
pub mod basic;
pub mod init;
pub mod file;

pub const API_FILE_DIR: &str = ".hapi";
pub const API_RESULT_DIR: &str = ".hapi_result";
pub const API_FILE_PATH: &str = concat!(".hapi/", "api.json");
pub const API_FILE_CONTENT: &str = include_str!("./default.json");

use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Run {
        #[arg(value_parser = parse_target)]
        target: HapiRunTarget
    }
}

#[derive(Clone, Debug)]
pub enum HapiRunTarget {
    All,
    Id(i32)
}

fn parse_target(s: &str) -> Result<HapiRunTarget, String> {
    if s == "all" {
        Ok(HapiRunTarget::All)
    } else {
        s.parse::<i32>()
            .map(HapiRunTarget::Id)
            .map_err(|_| format!("`{s}` 不是合法编号，也不是 all"))
    }
}
