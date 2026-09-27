use std::process::exit;

use hapi_core::{
    Api, basic::RequestMethod::{self, NULL},
    model::{
        RequestInfo,
        RequestPayload,
        RequestResponse
    }
};

use crate::model::RequestTable;

pub async fn api_test_for_num(api: Api, request_table: RequestTable, num: i32) -> RequestResponse {
    let api_index = num - 1;
    let mut current_index = 0;

    //设置默认的request_info
    let mut request_info: RequestInfo = RequestInfo {
        name: String::new(),
        url: String::new(),
        method: hapi_core::basic::RequestMethod::NULL,
        payload: RequestPayload {
            headers: serde_json::Value::Null,
            body: serde_json::Value::Null
        }
    };

    for request in request_table.0.iter(){
        if current_index == api_index {
            request_info = request.clone();
            break;
        }
        current_index += 1;
    }

    match request_info.method {
        NULL => {
            println!("请求方法为空");
            exit(1)
        }
        _ => ()
    }

    let result = match api.request(request_info).await{
        Ok(res)=>res,
        Err(e)=>{
            println!("请求错误: {e}");
            exit(1);
        }
    };

    if !result.is_success {
        println!("请求失败");
        exit(1);
    }

    return result;
}


pub async fn api_test_all(api: Api, request_table: RequestTable) -> Vec<RequestResponse> {
    let mut response_vec: Vec<RequestResponse> = Vec::new();

    for request in request_table.0.iter(){

        match request.method {
            RequestMethod::NULL => {
                eprintln!("请求方法为空无法请求");
                exit(1)
            }
            _ => ()
        }

        let result = match api.request(request.clone()).await{
            Ok(res) => res,
            Err(e) => {
                println!("请求错误: {e}");
                continue;
            }
        };

        response_vec.push(result);
    }
    return response_vec
}
