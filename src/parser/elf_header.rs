use crate::parser::{abi, file};

pub enum Class {
    ELF32,
    ELF64,
}


#[repr(C)]
pub struct Elf32_Ehdr {
    pub e_ident: [u8; abi::EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [u8; abi::EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}



pub mod elf_header {
    use std::string::ParseError;
    use crate::parser::{abi, endian, file};
    use crate::parser::elf_header::Class;
    use crate::parser::endian::EndianParse;

    fn verify_magic(data: Vec<u8>) -> bool {
        if data[0] == abi::ELFMAG0 && data[1] == abi::ELFMAG1 && data[2] == abi::ELFMAG2 && data[3] == abi::ELFMAG3 {
            return true;
        }
        return false;
    }

    fn parse_ident(data: Vec<u8>) -> bool {
        //验证.ELF魔数
        if !verify_magic(data.clone()) {
            return false;
        }

       let endian_type=data[abi::EI_DATA];
       //获取大小端写法
        let endian_self=endian::AnyEndian::new(endian_type);
        //获取elf位数
        let elf_class=data[abi::EI_CLASS];






        return true;
    }


    pub fn read_file_range(file_path: &str) -> bool {
        //首先读取64位数的字节数
        let headr = file::file_utils::read_file_range(file_path, 0, 64);

        match headr {
            Ok(data) => {
                println!("{}",data.len());
                //验证读取的字节数是否对应目标
                if data.len() != 65 {
                    return false;
                }
                return  parse_ident(data);
            }
            Err(error) => {
                println!("[!]read file fail");
                return false;
            }
        }
        return false;
    }
}
