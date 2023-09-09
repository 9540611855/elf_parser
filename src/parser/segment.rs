use crate::parser::elf_header::FileHeader;
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file;
use crate::parser::file::Class;
#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

/// C-style 64-bit ELF Program Segment Header definition
///
/// These C-style definitions are for users who want to implement their own ELF manipulation logic.
#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ProgramHeader {
    /// Program segment type
    pub p_type: u32,
    /// Offset into the ELF file where this segment begins
    pub p_offset: u64,
    /// Virtual adress where this segment should be loaded
    pub p_vaddr: u64,
    /// Physical address where this segment should be loaded
    pub p_paddr: u64,
    /// Size of this segment in the file
    pub p_filesz: u64,
    /// Size of this segment in memory
    pub p_memsz: u64,
    /// Flags for this segment
    pub p_flags: u32,
    /// file and memory alignment
    pub p_align: u64,
}

impl ProgramHeader {
    /// Helper method which uses checked integer math to get a tuple of (start, end) for
    /// the location in bytes for this ProgramHeader's data in the file.
    /// i.e. (p_offset, p_offset + p_filesz)

    pub fn read_program(file_path:&str,binary_header:FileHeader)->Option<Vec<ProgramHeader>>{
        let header_size = match binary_header.class{
            Class::ELF32 =>0x34,
            Class::ELF64 => 0x40,
        };
        let class=binary_header.class;
        let idents=(binary_header.endianness,binary_header.class);
        let program_header_offset=header_size.clone();
        let program_header_end=header_size.clone()+ProgramHeader::size_for(class) as u64;
        let program_header=file::file_utils::read_file_range(file_path,program_header_offset,program_header_end);
        //println!("{:?}",binary_header.expect("REASON").len());
        let program_header=ProgramHeader::parse_at
            (idents, 0, &program_header.unwrap());
        println!("[*]程序头部表头解析成功\n");
        println!("{:?}",program_header);


        let program_bytes=file::file_utils::read_file_range
            (file_path,program_header.p_offset+ProgramHeader::size_for(class) as u64,program_header.p_offset+program_header.p_filesz);
        let e_phnum=binary_header.e_phnum;
        let e_phsz=binary_header.e_phentsize;
        if ProgramHeader::check_program_size(binary_header,program_header){
            return None;
        }
        let vec_header=ProgramHeader::parse_program
            (idents.clone(),program_bytes.unwrap(),e_phnum,e_phsz);
        println!("[*]程序头部表解析成功:");
        println!("{:?}",vec_header);
        return Some(vec_header);

    }
    pub(crate) fn get_file_data_range(&self) -> (usize, usize){
        let start: usize = self.p_offset.try_into().expect("Failed to convert u64 to usize");
        let size: usize = self.p_filesz.try_into().expect("Failed to convert u64 to usize");
        let end=start+size;
        return  (start, end);
    }
    pub fn parse_program(ident: (AnyEndian, Class),program_bytes:Vec<u8>,e_phnum:u16,e_phsz:u16)->Vec<ProgramHeader>{
        let mut v: Vec<ProgramHeader> = Vec::new();
        for i in 0..e_phnum-1{
            let e_phdr=Self::parse_at(ident, (i * e_phsz) as usize, program_bytes.as_slice());
            v.push(e_phdr);
        }
        return  v;
    }
    pub(crate) fn parse_at(
        ident: (AnyEndian, Class),
        mut offset:usize,
        data: &[u8],
    ) -> Self{
        let (endian, class)=ident;
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        if class == Class::ELF32 {
               let p_type= endian.parse_u32_at(offset, data);
               offset+=U32SIZE;
               let p_offset= endian.parse_u32_at(offset, data)as u64;
                offset+=U32SIZE;
               let p_vaddr=endian.parse_u32_at(offset, data) as u64;
                offset+=U32SIZE;
               let p_paddr=endian.parse_u32_at(offset, data) as u64;
                offset+=U32SIZE;
               let p_filesz=endian.parse_u32_at(offset, data) as u64;
                offset+=U32SIZE;
               let p_memsz= endian.parse_u32_at(offset, data) as u64;
                offset+=U32SIZE;
               let p_flags= endian.parse_u32_at(offset, data);
                offset+=U32SIZE;
               let p_align= endian.parse_u32_at(offset, data) as u64;
            return (ProgramHeader {
                p_type,
                p_offset,
                p_vaddr,
                p_paddr,
                p_filesz,
                p_memsz,
                p_flags,
                p_align,
            });
        }

        // Note: 64-bit fields are in a different order
        let p_type = endian.parse_u32_at(offset, data);

        offset+=U32SIZE;
        let p_flags = endian.parse_u32_at(offset, data);
        offset+=U32SIZE;
        let p_offset = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        let p_vaddr = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        let p_paddr = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        let p_filesz = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        let p_memsz = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        let p_align = endian.parse_u64_at(offset, data);
        offset+=U64SIZE;
        return (ProgramHeader {
            p_type,
            p_offset,
            p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            p_flags,
            p_align,
        });
    }

    pub(crate) fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 32,
            Class::ELF64 => 56,
        }
    }
    pub fn check_program_size(binary_header:FileHeader,program_header:ProgramHeader)->bool{
        return u64::from(binary_header.e_phentsize*binary_header.e_phnum)!= program_header.p_filesz;
    }
}