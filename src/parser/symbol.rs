
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;
use crate::parser::symbol;

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

#[derive(Debug, Clone, PartialEq, Eq,Copy)]
pub struct Symbol {
    pub st_name: u32,
    pub st_shndx: u16,
    pub(super) st_info: u8,

    pub(super) st_other: u8,

    pub st_value: u64,

    pub st_size: u64,
}

impl Symbol {
    pub fn parser_Symbol(ident: (AnyEndian, Class),data:&[u8],mut offset: usize)->Vec<Symbol>{
        let (endian, class)=ident;
        let mut symbol_tables:Vec<Symbol>=Vec::new();
        while offset<data.len() {
            let symbol_table=Self::parse_at(ident,data,offset);
            symbol_tables.push(symbol_table);
            offset+=Self::size_for(class);
        }
        return symbol_tables;
    }
    pub fn parse_at(ident: (AnyEndian, Class),data:&[u8],mut offset: usize)->Symbol{
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
        };
    }
    pub fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 16,
            Class::ELF64 => 24,
        }
    }
}