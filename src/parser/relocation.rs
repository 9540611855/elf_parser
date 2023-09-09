use crate::parser;
use crate::parser::elf_header::FileHeader;
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file;
use crate::parser::file::Class;
use crate::parser::section::SectionHeader;

#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Rel {
    pub r_offset: u32,
    pub r_info: u32,
}
#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Rel {
    pub r_offset: u64,
    pub r_info: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rel {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
}

impl Rel {

    pub fn parse(ident: (AnyEndian, Class),data: &[u8],e_size:u64)->Vec<Rel>{
        let mut v: Vec<Rel> = Vec::new();
        let mut offset:usize=0;
        let (_,class)=ident;
        let size=Self::size_for(class);
        while  offset< e_size as usize {
            let ele=Self::parse_rel(ident,data,offset);
            offset+=size;
            v.push(ele);
        }
        return  v;
    }
    pub fn parse_rel(ident: (AnyEndian, Class),data: &[u8],mut offset:usize)->Rel{
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        let (endian,class)=ident;
        match class {
            Class::ELF32 => {
                let r_offset = endian.parse_u32_at(offset, data) as u64;
                offset+=U32SIZE;
                let r_info = endian.parse_u32_at(offset, data);
                (Rel {
                    r_offset,
                    r_sym: r_info >> 8,
                    r_type: r_info & 0xFF,
                })
            }
            Class::ELF64 => {
                let r_offset = endian.parse_u64_at(offset, data);
                offset+=U64SIZE;
                let r_info = endian.parse_u64_at(offset, data);
                (Rel {
                    r_offset,
                    r_sym: (r_info >> 32) as u32,
                    r_type: (r_info & 0xFFFFFFFF) as u32,
                })
            }
        }
    }
    fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 8,
            Class::ELF64 => 16,
        }
    }

}



#[derive(Debug)]
#[repr(C)]
pub struct Elf32_Rela {
    pub r_offset: u32,
    pub r_info: u32,
}
#[derive(Debug)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: u64,
    pub r_info: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rela {
    pub r_offset: u64,
    pub r_sym: u32,
    pub r_type: u32,
    pub r_addend: i64,
}


impl Rela {
    pub fn read_rela(file_path:&str,name:String,section_header:Vec<SectionHeader>,binary_header:FileHeader)->Option<Vec<Rela>>{
        let idents=(binary_header.endianness,binary_header.class);
        let rela_table_idx=parser::section::SectionHeader::
        find_section_header_by_name(section_header.clone(),name);
        let rela_table=&section_header[rela_table_idx as usize];

        if rela_table_idx==0{
            return None;
        }
        let e_rela_offset=rela_table.sh_offset;
        let e_rela_size=rela_table.sh_size;
        let rela_bytes=file::file_utils::
        read_file_range(file_path,e_rela_offset,e_rela_offset+e_rela_size);
        let rela_tables=parser::relocation::Rela::parse(idents,&rela_bytes.unwrap(),e_rela_size);
        println!("[*]解析重定位表成功");
        println!("{:?}",rela_tables);
        return Some(rela_tables);

    }
    pub fn parse(ident: (AnyEndian, Class),data: &[u8],e_size:u64)->Vec<Rela>{
        let mut v: Vec<Rela> = Vec::new();
        let mut offset:u64=0;
        let (_,class)=ident;
        let size=Self::size_for(class);
        while  offset< e_size {
            let ele=Self::parse_rela(ident,data,offset as usize);
            offset+=(size as u64);
            v.push(ele);
        }
        return  v;
    }
    pub fn parse_rela(ident: (AnyEndian, Class),data: &[u8],mut offset:usize)->Rela{
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        let (endian,class)=ident;
        match class {
            Class::ELF32 => {
                let r_offset = endian.parse_u32_at(offset as usize, data) as u64;
                offset+=U32SIZE;
                let r_info = endian.parse_u32_at(offset as usize, data);
                offset+=U32SIZE;
                let r_addend = endian.parse_i32_at(offset as usize, data) as i64;
                Rela {
                    r_offset,
                    r_sym: r_info >> 8,
                    r_type: r_info & 0xFF,
                    r_addend,
                }
            }
            Class::ELF64 => {
                let r_offset = endian.parse_u64_at(offset as usize, data);
                offset+=U64SIZE;
                let r_info = endian.parse_u64_at(offset as usize, data);
                offset+=U64SIZE;
                let r_addend = endian.parse_i64_at(offset as usize, data);
                Rela {
                    r_offset,
                    r_sym: (r_info >> 32) as u32,
                    r_type: (r_info & 0xFFFFFFFF) as u32,
                    r_addend,
                }
            }
        }
    }
    fn size_for(class: Class) -> usize {
        match class {
            Class::ELF32 => 12,
            Class::ELF64 => 24,
        }
    }

}


