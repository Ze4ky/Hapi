use std::{fs::read_to_string, io};

use clap::Parser;
use hapi_core::{Api, model::RequestInfo};

#[derive(Debug, Parser)]
struct HeyApiCliArgs {
    #[arg(short, default_value = "./request.json")]
    file: String,

    #[arg(long, default_value_t = false)]
    enable_complete_display: bool,
}

#[tokio::main]
async fn main() {
    let args = HeyApiCliArgs::parse();

    let file = match read_to_string(&args.file) {
        Ok(f) => f,
        Err(e) => {
            println!("读取{}发生错误: {}", args.file, e);
            return;
        }
    };

    let request_info_vec: Vec<RequestInfo> = serde_json::from_str(&file).unwrap();
    let api_client = Api::new();

    let mut api_counter: i32 = 0;
    for request_info in request_info_vec.iter() {
        api_counter += 1;
        println!("{}. 请求描述: {}", api_counter, request_info.name);
        if args.enable_complete_display {
            println!("  请求URL: {}", request_info.url);
            println!("  请求方法: {}", request_info.method);
            println!("  请求头: {}", request_info.payload.headers);
            println!("  请求体: {}", request_info.payload.body);
        }
    }

    let mut input = String::new();
    loop {
        println!("请输入请求编号:");
        io::stdin().read_line(&mut input).unwrap();

        if input == String::new() {
            println!("输入内容不能为空默认值为: 1");
            input = "1".to_string();
        }

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
        let response = match api_client.request(request_info.clone()).await {
            Ok(resp) => resp,
            Err(err) => {
                println!("请求错误: {}", err);
                return;
            }
        };

        //println!("响应代码: {}", response.code);
        println!("响应头:");
        for (k, v) in response.response_headers {
            println!("{}: {}", k, v)
        }
        println!("响应体:");
        println!(
            "{}",
            serde_json::to_string_pretty(&response.response_body).unwrap()
        );
    }
}
