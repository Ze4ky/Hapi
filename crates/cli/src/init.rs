use std::{fs, io::Write, process::exit};

use crate::{API_FILE_CONTENT, API_FILE_PATH};

pub fn initial(){
    let hapi_dir_is_exists = match fs::exists(".hapi/"){
        Ok(res) => res,
        Err(_)=> {
            false
        },
    };

    if !hapi_dir_is_exists {
        match fs::create_dir(".hapi/") {
            Ok(_) => (),
            Err(_) => {
                exit(1)
            }
        };

        let mut api_file_result = match fs::File::create(".hapi/api.json") {
            Ok(res) => res,
            Err(_) => {
                return;
            }
        };

        match api_file_result.write(API_FILE_CONTENT.as_bytes()){
            Ok(s) => s,
            Err(_) => {
                return;
            },
        };
    }

    let api_file_is_exists = match fs::exists(API_FILE_PATH.to_string()){
        Ok(is_exist) => is_exist,
        Err(_)=>{
            return;
        },
    };

    if !api_file_is_exists {
        let mut api_file_result = match fs::File::create(".hapi/api.json") {
            Ok(res) => res,
            Err(_) => {
                return;
            }
        };

        match api_file_result.write(API_FILE_CONTENT.as_bytes()){
            Ok(s) => s,
            Err(_) => {
                return;
            },
        };
    }
}
