use std::{fs::read_to_string, io};

use clap::Parser;
use hapi_core::{Api, model::RequestInfo};

#[derive(Debug, Parser)]
struct HeyApiCliArgs {
    #[arg(short, default_value = "./request.json")]
    file: String,
}

#[tokio::main]
async fn main() {
    let args = HeyApiCliArgs::parse();
    let file = read_to_string(args.file).unwrap();
    let request_info_vec: Vec<RequestInfo> = serde_json::from_str(&file).unwrap();
    let api_client = Api::new();

    let mut api_counter: i32 = 0;
    for request_info in request_info_vec.iter() {
        api_counter += 1;
        println!("{}. 请求描述: {}", api_counter, request_info.name);
        println!("  请求URL: {}", request_info.url);
        println!("  请求方法: {}", request_info.method);
        println!("  请求头: {}", request_info.payload.headers);
        println!("  请求体: {}", request_info.payload.body);
    }

    let mut input = String::new();
    loop {
        println!("请输入请求编号:");
        io::stdin().read_line(&mut input).unwrap();
        let mut num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入数字");
                return;
            }
        };
        num -= 1;
        input = String::new();
        let request_info = request_info_vec[num as usize].clone();
        let response = match api_client.request(request_info).await {
            Ok(resp) => resp,
            Err(err) => {
                println!("请求错误: {}", err);
                return;
            }
        };
        //println!("响应代码: {}", response.code);
        println!("  响应头:");
        for (k, v) in response.response_headers {
            println!("{}: {}", k, v)
        }
        println!("  {}", response.response_body);
    }
}
