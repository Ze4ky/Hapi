use std::{fs::{self, File}, process::exit, time::{SystemTime, UNIX_EPOCH}};

use hapi_core::model::RequestResponse;

use crate::{API_FILE_PATH, API_RESULT_DIR, HapiRunTarget, model::RequestTable};



pub fn read_api_file() -> RequestTable {
    let file = match fs::read_to_string(API_FILE_PATH) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[{}:{}]读取文件{API_FILE_PATH}错误: {e}", module_path!(), line!());
            exit(1);
        }
    };
    let result: RequestTable = match serde_json::from_str(&file){
        Ok(res) => res,
        Err(e) => {
            eprintln!("[{}:{}]序列化文件{API_FILE_PATH}错误: {e}", module_path!(), line!());
            exit(1);
        }
    };
    return result
}

pub fn write_result_file(target: HapiRunTarget, response: RequestResponse) {
    let result_dir_is_exists = match fs::exists(format!("{API_RESULT_DIR}/")){
        Ok(res) => res,
        Err(e) => {
            eprintln!("[{}:{}]检查目录{API_RESULT_DIR}错误: {e}",module_path!(), line!());
            return;
        }
    };
    let timestamp_src = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(ts) => ts,
        Err(e) => {
            eprintln!("[{}:{}]测试结果时间戳生成错误: {e}",module_path!(), line!());
            return;
        }
    };
    let timestamp = timestamp_src.as_secs().to_string();
    let target_type: String = match target {
        HapiRunTarget::All => {
            "all".to_string()
        }
        HapiRunTarget::Id(id) => {
            format!("index_{id}")
        }
    };

    if !result_dir_is_exists {
        let _ = match fs::create_dir(format!("{API_RESULT_DIR}")){
            Ok(_) => (),
            Err(e) => {
                eprintln!("[{}:{}]创建文件夹错误: {e}", module_path!(), line!());
                return;
            }
        };
    }

    let result_file_path = format!("{API_RESULT_DIR}/{target_type}_{timestamp}.json");
    let result_file_is_exists = match fs::exists(result_file_path.clone()) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[{}:{}]检查目录{result_file_path}错误: {e}",module_path!(), line!());
            return;
        }
    };

    if !result_file_is_exists {
        fs::File::create(result_file_path.clone()).unwrap();
    }

    let mut result_vec: Vec<RequestResponse> = Vec::new();

    let result_file = match File::options().write(true).open(result_file_path.clone()) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("[{}:{}]打开文件{result_file_path}错误: {e}", module_path!(), line!());
            exit(1)
        }
    };

    result_vec.push(response);

    match serde_json::to_writer_pretty(result_file, &result_vec){
        Ok(_) => (),
        Err(e) => {
            eprintln!("[{}:{}]写入{result_file_path}错误: {e}", module_path!(), line!())
        }
    };
}
