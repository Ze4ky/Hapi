use clap::Parser;
use hapi_cli::{Commands, HapiRunTarget, api::{api_test_all, api_test_for_num}, file::{read_api_file, write_result_file}, init::initial};
use hapi_core::Api;

#[derive(Parser)]
struct HapiCliArgs {
    #[command(subcommand)]
    commands: Commands
}

#[tokio::main]
async fn main() {
    let hapi = Api::new();
    let hapi_cli_args = HapiCliArgs::parse();
    match hapi_cli_args.commands {
        Commands::Init => {
            initial();
        }
        Commands::Run {target} => {
            match target {
                HapiRunTarget::All => {
                    let request_table = read_api_file();
                    let response_vec = api_test_all(hapi, request_table).await;
                    for response in response_vec.into_iter() {
                        write_result_file(target.clone(), response);
                    }
                },
                HapiRunTarget::Id(id) => {
                    let request_table = read_api_file();
                    let response = api_test_for_num(hapi, request_table, id).await;
                    write_result_file(target, response);
                }
            }
        }
    }
}
