mod parser;
use std::env;
use crate::parser::symbol::Symbol;

fn main() {
    let args: Vec<String> = env::args().collect();
    let args_len=args.len();
    if args_len<2{
        println!("[!]please input parser elf file path");
        return;
    }
    let file_path=&args[1];

    //parser::elf_header::elf_header::read_file_range(file_path);



    //parser elf header
    let elf_header=parser::elf_header::elf_header::read_header(file_path);
    if elf_header==None{
        println!("[!]解析elf header出错");
        return;
    }
    let elf_header=elf_header.unwrap();
    let program_headers=parser::segment::ProgramHeader::read_program(file_path,elf_header);
    if program_headers==None{
        println!("[!]解析elf program出错");
        return;
    }
    let section_headrs=parser::section::SectionHeader::read_section(file_path,elf_header);
    if section_headrs==None{
        println!("[!]解析elf section出错");
        return;
    }
    let section_headr=section_headrs.clone().unwrap();
    let string_map=parser::section::SectionHeader::parser_string_section(file_path,section_headr.clone(),elf_header);
    if string_map==None{
        println!("[!]解析elf string section出错");
        return;
    }
    //获取修复section header的名字
    let section_header=parser::section::SectionHeader::fix_section_name(string_map.unwrap(),section_headr.clone());
    println!("[*]解析elf  section name");
    println!("{:?}",section_header.clone());

    let symbol=parser::symbol::Symbol::read_symbol(file_path,section_header.clone(),elf_header);
    if  symbol==None{
        println!("[!]解析elf symbol section出错");
        return;
    }
    let sym_str=parser::symbol::Symbol::parser_str_symbol(file_path,section_header.clone());
    if sym_str==None{
        println!("[!]解析elf symbol string section出错");
        return;
    }
    //修复符号表内容
    let symbol_headers=parser::symbol::Symbol::fix_symbol_name(sym_str.unwrap(),symbol.unwrap());
    println!("[*]解析elf symbol string成功:");
    println!("{:?}",symbol_headers);
    //gun hash
    let gun_hash=parser::hash::hash::read_hash(file_path,section_header.clone(),elf_header);
    if gun_hash==None{
        println!("[!]解析elf gun_hash出错");
        return;
    }
    //测试hash表寻找符号
    let count=parser::hash::hash::all_sym_find(symbol_headers,gun_hash.unwrap(),elf_header);
    println!("[!]通过gun hash发现符号:");
    println!("{count}");
    //读取重定位表
    /*
                    find_section_header_by_name(section_header.clone(),".rela.dyn".to_string());
                let rela_plt_table_idx=parser::section::SectionHeader::
                find_section_header_by_name(section_header.clone(),".rela.plt".to_string());
    */
    let rela=parser::relocation::Rela::read_rela(file_path,".rela.dyn".to_string(),
                                                 section_header.clone(),elf_header);
    if rela ==None{
        println!("[!]解析elf .rela.dyn 出错");
        return;
    }
    let rela=parser::relocation::Rela::read_rela(file_path,".rela.plt".to_string(),
                                                 section_header.clone(),elf_header);
    if rela ==None{
        println!("[!]解析elf .rela.plt 出错");
        return;
    }
}
