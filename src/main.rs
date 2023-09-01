mod parser;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len=args.len();
    if args_len<2{
        println!("[!]please input parser elf file path");
        return;
    }
    let file_path=&args[1];
    let headr=parser::file::file_utils::read_file_range(file_path,0,64);
    match headr {
        Ok(data) => {
            
        }
        Err(error) => {
            println!("[!]read file fail");
            return;
        }
    }




}
