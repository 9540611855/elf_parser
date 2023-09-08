use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq)]
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
    pub string_name:String,
    pub index:u16,
}

impl  SectionHeader {
    pub fn parse_section(ident: (AnyEndian, Class),section_bytes:Vec<u8>,e_shnum:u16,e_shsz:u16)->Vec<SectionHeader>{
        let mut v: Vec<SectionHeader> = Vec::new();
        //let (endian, class)=ident;
        for i in 0..e_shnum{
            println!("{}",i);
            let e_shdr=Self::parse_at(ident, (i * e_shsz) as usize, section_bytes.as_slice(),i);
            v.push(e_shdr);
        }

        return v;

    }

    pub fn fix_section_name(string_table_map:HashMap<u32, String>, mut section_headers:Vec<SectionHeader>)->Vec<SectionHeader>{
        for mut section_header in  section_headers.iter_mut() {
            let sh_name = section_header.sh_name;
            if let Some(string) = string_table_map.get(&sh_name) {
                section_header.string_name = string.to_string();
            }
        }
        //println!("{:?}",section_headers);
        return section_headers;
    }

    pub fn parser_string_table(string_table_bytes:Vec<u8>)->HashMap<u32, String>{
        let mut result = HashMap::new();
        let mut start = 0;
        while start < string_table_bytes.len() {
            // Find the end of the current string
            let end = start + string_table_bytes[start..].iter().position(|&b| b == 0).unwrap();

            // Convert the bytes to a UTF-8 string
            let s = String::from_utf8_lossy(&string_table_bytes[start..end]).to_string();

            // Add the string and its index to the result
            result.insert(start as u32, s);

            // Move to the next string (add 1 to account for the null terminator)
            start = end + 1;
        }
        result

    }
    pub fn find_section_header_by_name(section_headers:Vec<SectionHeader>,name:String)
        ->i64{
        let mut count:i64=0;
        for section_header in section_headers{
           if section_header.string_name==name{
               return count;
           }
            count+=1;
        }
        return -1;
    }
    pub fn find_section_header_by_type(section_headers:Vec<SectionHeader>,sh_type:u32)
                                       ->i64{
        let mut count:i64=0;
        for section_header in section_headers{
            if section_header.sh_type==sh_type{
                return count;
            }
            count+=1;
        }
        return -1;
    }
    pub(crate) fn parse_at(
        ident: (AnyEndian, Class),
        mut offset: usize,
        data: &[u8],
        index:u16,
    ) -> SectionHeader {
        let (endian, class)=ident;
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
                 string_name:"".to_string(),
                 index:index,
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
            string_name:"".to_string(),
            index:index,
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