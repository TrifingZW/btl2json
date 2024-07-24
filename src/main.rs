use std::{env, fs};
use std::fs::File;
use std::io::{Read, Write};

use bincode::Error;

use crate::btl_parser::BtlParser;

mod btl_structure;
mod btl_parser;

fn main() -> Result<(), Error> {

    // 获取当前工作目录
    let current_dir = env::current_dir()?;
    println!("Current directory: {}", current_dir.display());

    // 遍历当前目录
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        // 检查是否是文件并且后缀是 .btl
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("btl") {
            let mut file = File::open(&path)?;

            // 读取文件内容
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            //解析文件内容
            let mut instance: BtlParser = BtlParser::new();
            instance.parse(buffer)?;

            //保存为json文件
            let out_file = path.with_extension("json");
            let mut file = File::create(&out_file)?;
            file.write_all(instance.to_json()?.as_ref())?;
        }
    }

    Ok(())
}
