use crate::parser::{abi, file};
use crate::parser::endian::{AnyEndian, EndianParse};
#[derive(Debug,PartialEq)]
pub enum Class {
    ELF32,
    ELF64,
}
#[derive(Debug)]
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
    use crate::parser::{abi, endian, file};
    use crate::parser::elf_header::{Class, FileHeader};
    use crate::parser::endian::{AnyEndian, EndianParse};

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


    pub fn read_file_range(file_path: &str) -> bool {
        //首先读取64位数的字节数
        let headr = file::file_utils::read_file_range(file_path, 0, 15);

        match headr {
            Ok(data) => {
                //验证读取的字节数是否对应目标
                if data.len() != 16 {
                    return false;
                }

                //验证.ELF魔数
                if !verify_magic(data.clone()) {
                    return false;
                }
               let ident=parse_ident(data);
                let idents=ident.unwrap();
                let header_size = match idents.1{
                    Class::ELF32 =>0x34,
                    Class::ELF64 => 0x40,
                };
                let headr = file::file_utils::read_file_range(file_path, 0, header_size);
                let binary_header=file_header(idents,&headr.unwrap());
                println!("{:?}", binary_header);


            }
            Err(error) => {
                println!("[!]read file fail");
                return false;
            }
        }
        return false;
    }
}
