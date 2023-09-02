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
    parser::elf_header::elf_header::read_file_range(file_path);




}
