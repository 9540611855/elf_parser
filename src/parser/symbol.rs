use std::collections::HashMap;
use crate::parser;
use crate::parser::elf_header::FileHeader;
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;
use crate::parser::section::SectionHeader;
use crate::parser::{file, symbol};

/// C-style 32-bit ELF Symbol definition
///
/// These C-style definitions are for users who want to implement their own ELF manipulation logic.
#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Sym {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u32,

}

/// C-style 64-bit ELF Symbol definition
///
/// These C-style definitions are for users who want to implement their own ELF manipulation logic.
#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Symbol {
    pub st_name: u32,
    pub st_shndx: u16,
    pub(super) st_info: u8,

    pub(super) st_other: u8,

    pub st_value: u64,

    pub st_size: u64,
    pub string_name:String,
    pub index:u16,
}

impl Symbol {
    pub fn read_symbol(file_path:&str,section_headers:Vec<SectionHeader>,binary_header:FileHeader)->Option<Vec<Symbol>>{
        //寻找symbol表并且读取symbol表的内容
        //SHT_DYNSYM=11
        let idents=(binary_header.endianness,binary_header.class);
        let symbol_index=parser::section::SectionHeader::
        find_section_header_by_type(section_headers.clone(), 11);
        let symbol_section_header=&section_headers[symbol_index as usize];
        //解析symbol
        let offset=symbol_section_header.sh_offset;
        let size=symbol_section_header.sh_size;
        let symbol_bytes=file::file_utils::read_file_range
            (file_path,offset,offset+size);
        let symbol_bytes_u8=symbol_bytes.unwrap();
        //检查读写大小是否能够被长度整除
        if symbol_bytes_u8.len()%Symbol::size_for(binary_header.class)!=0{
            return None;
        }
        //解析符号表
        let symbol_header=Symbol::parser_Symbol(idents,symbol_bytes_u8.as_slice(),0);
        println!("{:?}",symbol_header);
        return Some(symbol_header);
    }
    pub fn parser_str_symbol(file_path:&str,section_header:Vec<SectionHeader>)->Option<HashMap<u32,String>>{
        //解析符号字符串表
        let symbol_str_header_idx=SectionHeader::
        find_section_header_by_name(section_header.clone(),(&".dynstr").to_string());
        //获取符号str表
        if symbol_str_header_idx==-1{
            return None;
        }
        let symbol_str_section_header=&section_header[symbol_str_header_idx as usize];
        let e_shstr_offset=symbol_str_section_header.sh_offset;
        let e_shstr_size=symbol_str_section_header.sh_size;
        let symbol_str_byte=file::file_utils::read_file_range(file_path,e_shstr_offset,e_shstr_offset+e_shstr_size);
        let symbol_map_string=parser::section::SectionHeader::parser_string_table(symbol_str_byte.unwrap());
        println!("[*]符号字符串表解析成功:");
        println!("{:?}",symbol_map_string);
        return Some(symbol_map_string);

    }

    pub fn parser_Symbol(ident: (AnyEndian, Class),data:&[u8],mut offset: usize)->Vec<Symbol>{
        let (endian, class)=ident;
        let mut symbol_tables:Vec<Symbol>=Vec::new();
        let mut count:u16=0;
        while offset<data.len() {
            let symbol_table=Self::parse_at(ident,data,offset,count);
            symbol_tables.push(symbol_table);
            offset+=Self::size_for(class);
            count+=1;
        }
        return symbol_tables;
    }
    pub fn fix_symbol_name(string_table_map:HashMap<u32, String>,mut symbol_tables:Vec<Symbol>)->Vec<Symbol>{

        for mut symbol_table in symbol_tables.iter_mut() {
                let st_name=symbol_table.st_name;
            if let Some(string) = string_table_map.get(&st_name) {
                symbol_table.string_name = string.to_string();
            }
        }
        return  symbol_tables;

    }
    pub fn parse_at(ident: (AnyEndian, Class),data:&[u8],mut offset: usize,count:u16)->Symbol{
        let st_name: u32;
        let st_value: u64;
        let st_size: u64;
        let st_shndx: u16;
        let st_info: u8;
        let st_other: u8;
        let (endian, class)=ident;
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        if class == Class::ELF32 {
            st_name = endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            st_value = endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            st_size = endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            st_info = endian.parse_u8_at(offset, data);
            offset+=U8SIZE;
            st_other = endian.parse_u8_at(offset, data);
            offset+=U8SIZE;
            st_shndx = endian.parse_u16_at(offset, data);
        } else {
            st_name = endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            st_info = endian.parse_u8_at(offset, data);
            offset+=U8SIZE;
            st_other = endian.parse_u8_at(offset, data);
            offset+=U8SIZE;
            st_shndx = endian.parse_u16_at(offset, data);
            offset+=U16SIZE;
            st_value = endian.parse_u64_at(offset, data);
            offset+=U64SIZE;
            st_size = endian.parse_u64_at(offset, data);
        }

        return Symbol {
            st_name,
            st_value,
            st_size,
            st_shndx,
            st_info,
            st_other,
            string_name:"".to_string(),
            index:count,
        };
    }
    pub fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 16,
            Class::ELF64 => 24,
        }
    }
}