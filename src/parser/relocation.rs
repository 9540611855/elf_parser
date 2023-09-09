
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;

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

    pub fn parse(ident: (AnyEndian, Class),data: &[u8],e_size:u64)->Vec<Rela>{
        let mut v: Vec<Rela> = Vec::new();
        let mut offset:u64=0;
        let (_,class)=ident;
        let size=Self::size_for(class);
        println!("{offset}");
        println!("123123123123");
        while  offset< e_size {
            println!("{offset}");
            println!("{e_size}");
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
                println!("{}",offset);
                let r_info = endian.parse_u64_at(offset as usize, data);
                offset+=U64SIZE;
                println!("{}",offset);
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


