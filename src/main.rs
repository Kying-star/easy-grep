use std::{env, process};
use easy_grep::{Config , run };


fn main() {

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("解析参数发生错误: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        println!("程序错误: {:?}", e);
        process::exit(1)
    }
}
