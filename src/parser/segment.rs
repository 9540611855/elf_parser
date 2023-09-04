use crate::parser::endian::{AnyEndian, EndianParse};
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
    pub(crate) fn get_file_data_range(&self) -> (usize, usize){
        let start: usize = self.p_offset.try_into().expect("Failed to convert u64 to usize");
        let size: usize = self.p_filesz.try_into().expect("Failed to convert u64 to usize");
        let end=start+size;
        return  (start, end);
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
                offset+=U64SIZE;
               let p_paddr=endian.parse_u32_at(offset, data) as u64;
                offset+=U64SIZE;
               let p_filesz=endian.parse_u32_at(offset, data) as u64;
                offset+=U64SIZE;
               let p_memsz= endian.parse_u32_at(offset, data) as u64;
                offset+=U64SIZE;
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

    fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 32,
            Class::ELF64 => 56,
        }
    }
}