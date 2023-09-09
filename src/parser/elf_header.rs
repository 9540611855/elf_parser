use crate::parser::{abi, file};
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file::Class;

#[derive(Debug,PartialEq,Copy,Clone)]
pub struct FileHeader {
    /// 32-bit vs 64-bit
    pub class: Class,
    // file byte order
    pub endianness: AnyEndian,
    /// elf version
    pub version: u32,
    /// OS ABI
    pub osabi: u8,
    /// Version of the OS ABI
    pub abiversion: u8,
    /// ELF file type
    pub e_type: u16,
    /// Target machine architecture
    pub e_machine: u16,
    /// Virtual address of program entry point
    /// This member gives the virtual address to which the system first transfers control,
    /// thus starting the process. If the file has no associated entry point, this member holds zero.
    ///
    /// Note: Type is Elf32_Addr or Elf64_Addr which are either 4 or 8 bytes. We aren't trying to zero-copy
    /// parse the FileHeader since there's only one per file and its only ~45 bytes anyway, so we use
    /// u64 for the three Elf*_Addr and Elf*_Off fields here.
    pub e_entry: u64,
    /// This member holds the program header table's file offset in bytes. If the file has no program header
    /// table, this member holds zero.
    pub e_phoff: u64,
    /// This member holds the section header table's file offset in bytes. If the file has no section header
    /// table, this member holds zero.
    pub e_shoff: u64,
    /// This member holds processor-specific flags associated with the file. Flag names take the form EF_machine_flag.
    pub e_flags: u32,
    /// This member holds the ELF header's size in bytes.
    pub e_ehsize: u16,
    /// This member holds the size in bytes of one entry in the file's program header table; all entries are the same size.
    pub e_phentsize: u16,
    /// This member holds the number of entries in the program header table. Thus the product of e_phentsize and e_phnum
    /// gives the table's size in bytes. If a file has no program header table, e_phnum holds the value zero.
    pub e_phnum: u16,
    /// This member holds a section header's size in bytes. A section header is one entry in the section header table;
    /// all entries are the same size.
    pub e_shentsize: u16,
    /// This member holds the number of entries in the section header table. Thus the product of e_shentsize and e_shnum
    /// gives the section header table's size in bytes. If a file has no section header table, e_shnum holds the value zero.
    ///
    /// If the number of sections is greater than or equal to SHN_LORESERVE (0xff00), this member has the value zero and
    /// the actual number of section header table entries is contained in the sh_size field of the section header at index 0.
    /// (Otherwise, the sh_size member of the initial entry contains 0.)
    pub e_shnum: u16,
    /// This member holds the section header table index of the entry associated with the section name string table. If the
    /// file has no section name string table, this member holds the value SHN_UNDEF.
    ///
    /// If the section name string table section index is greater than or equal to SHN_LORESERVE (0xff00), this member has
    /// the value SHN_XINDEX (0xffff) and the actual index of the section name string table section is contained in the
    /// sh_link field of the section header at index 0. (Otherwise, the sh_link member of the initial entry contains 0.)
    pub e_shstrndx: u16,
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
    use crate::parser;
    use crate::parser::{abi, endian, file, symbol};
    use crate::parser::elf_header::{Class, FileHeader};
    use crate::parser::endian::{AnyEndian, EndianParse};
    use crate::parser::segment::ProgramHeader;

    fn verify_magic(data: Vec<u8>) -> bool {
        if data[0] == abi::ELFMAG0 && data[1] == abi::ELFMAG1 && data[2] == abi::ELFMAG2 && data[3] == abi::ELFMAG3 {
            return true;
        }
        return false;
    }

   pub fn parse_ident(data: Vec<u8>) -> Result<(endian::AnyEndian, Class),endian::AnyEndian > {

       let endian_type=data[abi::EI_DATA];
       //获取大小端写法
        let endian_self=endian::AnyEndian::new(endian_type);
        //获取elf位数
        let elf_class=data[abi::EI_CLASS];
        let class = match elf_class {
            abi::ELFCLASS32 => Class::ELF32,
            abi::ELFCLASS64 => Class::ELF64,
            _ => Class::ELF64,
        };

        Ok((
            endian_self,
            class
        ))
    }

    pub fn file_header(ident: (AnyEndian, Class),data: &[u8])-> FileHeader{
        let (file_endian, class,)=ident;
        let mut  offset=0x10;
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        let e_type= file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_machine =file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let version = file_endian.parse_u32_at(offset, data);
        offset=offset+U32SIZE;
        let e_entry: u64;
        let e_phoff: u64;
        let e_shoff: u64;
        if class ==  Class::ELF32 {
            e_entry = file_endian.parse_u32_at(offset, data) as u64;
            offset=offset+U32SIZE;
            e_phoff = file_endian.parse_u32_at(offset, data) as u64;
            offset=offset+U32SIZE;
            e_shoff = file_endian.parse_u32_at(offset, data) as u64;
            offset=offset+U32SIZE;
        } else {
            e_entry = file_endian.parse_u64_at(offset, data);
            offset=offset+U64SIZE;
            e_phoff = file_endian.parse_u64_at(offset, data);
            offset=offset+U64SIZE;
            e_shoff = file_endian.parse_u64_at(offset, data);
            offset=offset+U64SIZE;
        }

        let e_flags = file_endian.parse_u32_at(offset, data);
        offset=offset+U32SIZE;
        let e_ehsize = file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_phentsize = file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_phnum = file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_shentsize = file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_shnum = file_endian.parse_u16_at(offset, data);
        offset=offset+U16SIZE;
        let e_shstrndx = file_endian.parse_u16_at(offset, data);


        return  FileHeader{
            class:class,
            endianness:file_endian ,
            version:version,
            osabi: 0,
            abiversion: 0,
            e_type:e_type,
            e_machine:e_machine,
            e_entry:e_entry,
            e_phoff:e_phoff,
            e_shoff:e_shoff,
            e_flags:e_flags,
            e_ehsize:e_ehsize,
            e_phentsize:e_phentsize,
            e_phnum: e_phnum,
            e_shentsize: e_shentsize,
            e_shnum: e_shnum,
            e_shstrndx: e_shstrndx
        }

    }

    pub fn read_header(file_path: &str)->Option<(FileHeader)>{
        let headr = file::file_utils::read_file_range(file_path, 0, 16).unwrap();
        if headr.len() != 16 {
            return None;
        }

        if !verify_magic(headr.clone()) {
            return None;
        }
        let ident=parse_ident(headr);
        let idents=ident.clone().unwrap();
        let header_size = match idents.1{
            Class::ELF32 =>0x34,
            Class::ELF64 => 0x40,
        };

        let headr = file::file_utils::read_file_range(file_path, 0, header_size);
        //读取文件头
        let binary_header=file_header(idents,&headr.unwrap());
        println!("[*]程序头解析成功:");
        println!("{:?}\n", binary_header);

        return Some(binary_header);

    }
    pub fn read_file_range(file_path: &str) -> bool {

        let binary_header=read_header(file_path).unwrap();
        let header_size = match binary_header.class{
            Class::ELF32 =>0x34,
            Class::ELF64 => 0x40,
        };
        let class=binary_header.class;
        let idents=(binary_header.endianness,binary_header.class);
        let program_header_offset=header_size.clone();
        let program_header_end=header_size.clone()+ProgramHeader::size_for(class) as u64;
        let program_header=file::file_utils::read_file_range(file_path,program_header_offset,program_header_end);
        println!("{}",program_header_end);
        println!("{}",program_header_offset);
        //println!("{:?}",binary_header.expect("REASON").len());
        let program_header=ProgramHeader::parse_at
                    (idents, 0, &program_header.unwrap());
                println!("{:?}",program_header);


                let program_bytes=file::file_utils::read_file_range
                    (file_path,program_header.p_offset+ProgramHeader::size_for(class) as u64,program_header.p_offset+program_header.p_filesz);
                println!("{:?}",program_bytes);
                let e_phnum=binary_header.e_phnum;
                let e_phsz=binary_header.e_phentsize;
                let e_shnum=binary_header.e_shnum;
                let e_shsz=binary_header.e_shentsize;
                let e_shoff=binary_header.e_shoff;
                let e_shstrndx=binary_header.e_shstrndx;
                if ProgramHeader::check_program_size(binary_header,program_header){
                        println!("[!]ProgramHeader fail!");
                        return false;
                }
                let vec_header=ProgramHeader::parse_program
                    (idents.clone(),program_bytes.unwrap(),e_phnum,e_phsz);
                println!("{:?}",vec_header);
                let section_bytes=file::file_utils::read_file_range
                    (file_path,e_shoff,(e_shoff+(e_shsz*e_shnum) as u64));
                //解析section
                let section_header=parser::section::SectionHeader::parse_section
                    (idents, section_bytes.unwrap(), e_shnum,e_shsz);
                //println!("{:?}",section_header);
                //解析section string tables
                if e_shstrndx>= section_header.len() as u16 {
                    return  false;
                }
                let e_shstr=&section_header[e_shstrndx as usize];
                let e_shstr_offset=e_shstr.sh_offset;
                let e_shstr_size=e_shstr.sh_size;

                let string_table_bytes=file::file_utils::read_file_range
                    (file_path,e_shstr_offset,e_shstr_offset+e_shstr_size);
                let string_map=parser::section::SectionHeader::parser_string_table(string_table_bytes.unwrap());
                println!("{:?}",string_map);
                //获取修复section header的名字
                let section_header=parser::section::SectionHeader::fix_section_name(string_map,section_header.clone());
                println!("{:?}",section_header.clone());
                //寻找symbol表并且读取symbol表的内容
                //SHT_DYNSYM=11
                let symbol_index=parser::section::SectionHeader::
                find_section_header_by_type(section_header.clone(), 11);
                let symbol_section_header=&section_header[symbol_index as usize];
                //解析symbol
                let offset=symbol_section_header.sh_offset;
                let size=symbol_section_header.sh_size;
                let symbol_bytes=file::file_utils::read_file_range
                    (file_path,offset,offset+size);
                let symbol_bytes_u8=symbol_bytes.unwrap();
                //检查读写大小是否能够被长度整除
                if symbol_bytes_u8.len()%parser::symbol::Symbol::size_for(class)!=0{
                    return false;
                }
                //解析符号表
                let symbol_header=parser::symbol::Symbol::parser_Symbol(idents,symbol_bytes_u8.as_slice(),0);
                println!("{:?}",symbol_header);
                //解析符号字符串表
                let symbol_str_header_idx=parser::section::SectionHeader::
                find_section_header_by_name(section_header.clone(),(&".dynstr").to_string());
                //获取符号str表
                let symbol_str_section_header=&section_header[symbol_str_header_idx as usize];
                let e_shstr_offset=symbol_str_section_header.sh_offset;
                let e_shstr_size=symbol_str_section_header.sh_size;
                let symbol_str_byte=file::file_utils::read_file_range(file_path,e_shstr_offset,e_shstr_offset+e_shstr_size);
                let symbol_map_string=parser::section::SectionHeader::parser_string_table(symbol_str_byte.unwrap());
                println!("{:?}",symbol_map_string);

                //修复符号表内容
                let symbol_headers=parser::symbol::Symbol::fix_symbol_name(symbol_map_string,symbol_header);
                println!("{:?}",symbol_headers);

                //寻找hash表
                let hash_table_idx=parser::section::SectionHeader::
                find_section_header_by_type(section_header.clone(),1879048182);
                //println!("{:?}",hash_table_idx);
                let hash_section_header=&section_header[hash_table_idx as usize];



                let e_hash_offset=hash_section_header.sh_offset;
                let e_hash_size=hash_section_header.sh_size;


                //读取hash表
                let hash_bytes=file::file_utils::read_file_range(file_path,e_hash_offset,e_hash_offset+e_hash_size);
                let hash_tables=parser::hash::hash::parser_hash_tables(idents,&hash_bytes.unwrap());
                println!("{:?}",hash_tables);
                let mut count=0;
                for symbol_header in symbol_headers.clone(){
                    let res=hash_tables.find(symbol_headers.clone(),symbol_header.string_name.as_bytes(),class);
                    match res {
                        Some((index, character)) => {
                            println!("gun hash:Symbol at index {}: {:?}", index, character);
                            count+=1;
                        },
                        None => {
                            println!("No symbol found.");
                        },
                    }
                }
                //寻找重定位表
                let rela_dyn_table_idx=parser::section::SectionHeader::
                find_section_header_by_name(section_header.clone(),".rela.dyn".to_string());
                let rela_plt_table_idx=parser::section::SectionHeader::
                find_section_header_by_name(section_header.clone(),".rela.plt".to_string());
                println!("{}",count);
                let rela_dyn_table=&section_header[rela_dyn_table_idx as usize];
                let rela_plt_table=&section_header[rela_plt_table_idx as usize];


                let e_rela_offset=rela_dyn_table.sh_offset;
                let e_rela_size=rela_dyn_table.sh_size;
                let rela_bytes=file::file_utils::
                read_file_range(file_path,e_rela_offset,e_rela_offset+e_rela_size);
                let rela_dyn_tables=parser::relocation::Rela::parse(idents,&rela_bytes.unwrap(),e_rela_size);
                println!("{:?}",rela_dyn_tables);
                let e_rela_offset=rela_plt_table.sh_offset;
                let e_rela_size=rela_plt_table.sh_size;
                let rela_bytes=file::file_utils::
                read_file_range(file_path,e_rela_offset,e_rela_offset+e_rela_size);
                let rela_plt_tables=parser::relocation::Rela::parse(idents,&rela_bytes.unwrap(),e_rela_size);
                println!("{:?}",rela_plt_tables);


        return false;
    }
}
