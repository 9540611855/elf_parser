

pub enum Class {
    ELF32,
    ELF64,
}

/// C-style 32-bit ELF File Header definition
///
/// These C-style definitions are for users who want to implement their own ELF manipulation logic.
#[derive(Debug)]
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

/// C-style 64-bit ELF File Header definition
///
/// These C-style definitions are for users who want to implement their own ELF manipulation logic.
#[derive(Debug)]
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



pub mod elf_header{



}