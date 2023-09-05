use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SectionHeader {
    /// Section Name
    pub sh_name: u32,
    /// Section Type
    pub sh_type: u32,
    /// Section Flags
    pub sh_flags: u64,
    /// in-memory address where this section is loaded
    pub sh_addr: u64,
    /// Byte-offset into the file where this section starts
    pub sh_offset: u64,
    /// Section size in bytes
    pub sh_size: u64,
    /// Defined by section type
    pub sh_link: u32,
    /// Defined by section type
    pub sh_info: u32,
    /// address alignment
    pub sh_addralign: u64,
    /// size of an entry if section data is an array of entries
    pub sh_entsize: u64,
}

impl  SectionHeader {
    fn parse_at(
        endian: AnyEndian,
        class: Class,
        mut offset: usize,
        data: &[u8],
    ) -> SectionHeader {
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        if class == Class::ELF32 {
            let sh_name= endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            let sh_type= endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            let sh_flags=endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            let sh_addr=endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            let sh_offset=endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            let sh_size= endian.parse_u32_at(offset, data) as u64;
            offset+=U32SIZE;
            let sh_link= endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            let sh_info= endian.parse_u32_at(offset, data);
            offset+=U32SIZE;
            let sh_addralign= endian.parse_u32_at(offset, data)as u64;
            offset+=U32SIZE;
            let sh_entsize= endian.parse_u32_at(offset, data)as u64;
            return (SectionHeader {
                 sh_name:sh_name ,
                 sh_type: sh_type,
                 sh_flags: sh_flags,
                 sh_addr: sh_addr,
                 sh_offset: sh_offset,
                 sh_size: sh_size,
                 sh_link: sh_link,
                 sh_info: sh_info,
                 sh_addralign: sh_addralign,
                 sh_entsize: sh_entsize,
            });
        }

        let sh_name= endian.parse_u32_at(offset, data);
        offset+=U32SIZE;
        let sh_type= endian.parse_u32_at(offset, data);
        offset+=U32SIZE;
        let sh_flags=endian.parse_u64_at(offset, data) as u64;
        offset+=U64SIZE;
        let sh_addr=endian.parse_u64_at(offset, data) as u64;
        offset+=U64SIZE;
        let sh_offset=endian.parse_u64_at(offset, data) as u64;
        offset+=U64SIZE;
        let sh_size= endian.parse_u64_at(offset, data) as u64;
        offset+=U64SIZE;
        let sh_link= endian.parse_u32_at(offset, data);
        offset+=U32SIZE;
        let sh_info= endian.parse_u32_at(offset, data);
        offset+=U32SIZE;
        let sh_addralign= endian.parse_u64_at(offset, data)as u64;
        offset+=U64SIZE;
        let sh_entsize= endian.parse_u64_at(offset, data)as u64;
        return (SectionHeader {
            sh_name:sh_name ,
            sh_type: sh_type,
            sh_flags: sh_flags,
            sh_addr: sh_addr,
            sh_offset: sh_offset,
            sh_size: sh_size,
            sh_link: sh_link,
            sh_info: sh_info,
            sh_addralign: sh_addralign,
            sh_entsize: sh_entsize,
        });
    }

    #[inline]
    fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 40,
            Class::ELF64 => 64,
        }
    }
}