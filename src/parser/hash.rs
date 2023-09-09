use std::mem::size_of;
use std::ops::Add;
use crate::parser;
use crate::parser::elf_header::FileHeader;
use crate::parser::endian::{AnyEndian, EndianParse};
use crate::parser::file;
use crate::parser::file::Class;
use crate::parser::section::SectionHeader;
use crate::parser::symbol::Symbol;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct hash {
    pub nbucket: u32,
    pub symoffset: u32,
    pub bloom_size: u32,
    pub bloom_shift: u32,
    pub bloom:Vec<u64>,
    pub buckets:Vec<u64>,
    pub chains:Vec<u64>,
    //pub bloom[bloom_size];
    //pub buckets[nbuckets];
    //pub chain[];


}

impl hash {
    pub fn all_sym_find(symbol_headers:Vec<Symbol>,hash_tables:hash,binary_header:FileHeader)->u32{
        let mut count =0;
        for symbol_header in symbol_headers.clone(){
            let res=hash_tables.find(symbol_headers.clone(),symbol_header.string_name.as_bytes(),binary_header.class);
            match res {
                Some((index, character)) => {
                    println!("[*]gun hash:Symbol at index {}: {:?}", index, character);
                    count+=1;
                },
                None => {

                },
                }
            }
            return  count;

    }
    pub fn read_hash(file_path:&str,section_headers:Vec<SectionHeader>,binary_header:FileHeader)->Option<hash>{
        let idents=(binary_header.endianness,binary_header.class);
        //寻找hash表
        let hash_table_idx=parser::section::SectionHeader::
        find_section_header_by_type(section_headers.clone(),1879048182);
        //println!("{:?}",hash_table_idx);
        if hash_table_idx==-1{
            return None;
        }
        let hash_section_header=&section_headers[hash_table_idx as usize];



        let e_hash_offset=hash_section_header.sh_offset;
        let e_hash_size=hash_section_header.sh_size;


        //读取hash表
        let hash_bytes=file::file_utils::read_file_range(file_path,e_hash_offset,e_hash_offset+e_hash_size);
        let hash_tables=hash::parser_hash_tables(idents,&hash_bytes.unwrap());
        println!("{:?}",hash_tables);
        return Some(hash_tables);
    }
    pub fn gnu_hash(name: &[u8]) -> u32 {
        let mut hash = 5381u32;
        for byte in name {
            hash = hash.wrapping_mul(33).wrapping_add(u32::from(*byte));
        }
        hash
    }
    //跟据符号名 寻找符号表
    pub fn find(&self,symbol_table:Vec<Symbol>,name:&[u8],class:Class)->Option<(usize, Symbol)>{

        if self.buckets.is_empty() || self.bloom_size == 0 {
            return None;
        }

        let hash = Self::gnu_hash(name);
        let (bloom_width, filter) =match class {
            Class::ELF32 => {
                let bloom_width: u32 = 8 * size_of::<u32>() as u32; // 32
                let bloom_idx = (hash / (bloom_width)) % self.bloom_size;
                (bloom_width, *self.bloom.get(bloom_idx as usize).unwrap())
            }
            Class::ELF64 => {
                let bloom_width: u32 = 8 * size_of::<u64>() as u32; // 64
                let bloom_idx = (hash / (bloom_width)) % self.bloom_size;
                (bloom_width, *self.bloom.get(bloom_idx as usize).unwrap())
            }
            _=>{
                let bloom_width=0;
                let bloom_idx:u64=0;
                (bloom_width,bloom_idx)
            }

        };

        if filter & (1 << (hash % bloom_width)) == 0 {
            return None;
        }
        let hash2 = hash>>self.bloom_shift;
        if filter & (1 << (hash2 % bloom_width)) == 0 {
            return None;
        }
        let table_start_idx = self.symoffset as usize;
        let chain_start_idx = *self.buckets.get((hash as usize) % self.buckets.len())? as usize;
        if chain_start_idx < table_start_idx {
            return None;
        }

        let chain_len = self.chains.len();
        for chain_idx in (chain_start_idx - table_start_idx)..chain_len {
            let chain_hash = *self.chains.get(chain_idx).unwrap();

            if hash | 1 == (chain_hash | 1) as u32 {
                let sym_idx = chain_idx+table_start_idx;
                let symbol = symbol_table.get(sym_idx).unwrap();

                if symbol.string_name == String::from_utf8(name.to_vec()).unwrap() {
                    return (Some((sym_idx, symbol.clone())));
                }
            }
            if chain_hash & 1 != 0 {
                break;
            }
        }

        return None;

    }

    pub fn parser_buf_vec(esize:usize,bytes:&[u8],endian:AnyEndian,buf_size:u32)->Vec<u64>{
        let mut v: Vec<u64> = Vec::new();
        let mut offset:usize=0;
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        while  offset<(buf_size as usize){
            let ele = match esize {
                U32SIZE => endian.parse_u32_at(offset,bytes) as u64,
                U64SIZE => endian.parse_u64_at(offset,bytes),
                _=>endian.parse_u32_at(offset,bytes) as u64,
            };
            offset+=esize;
            v.push(ele);
        }
        return  v;
    }

    pub fn parser_hash_tables(ident: (AnyEndian, Class),hash_bytes:&[u8])->hash{
        let (endian, class)=ident;
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        let mut offset=0;
        let nbucket=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let symoffset=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let  bloom_size=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let  bloom_shift=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let bloom_len = match class {
            Class::ELF32 => bloom_size*(U32SIZE as u32),
            Class::ELF64 => bloom_size*(U64SIZE as u32),
        };
        let bloom_end:usize= (bloom_len + (offset as u32)) as usize;
        let buckets_buf = hash_bytes.get(offset..bloom_end).unwrap();
        let bloom = match class {
            Class::ELF32 =>Self::parser_buf_vec(U32SIZE,buckets_buf,endian,bloom_len),
            Class::ELF64 =>Self::parser_buf_vec(U64SIZE,buckets_buf,endian,bloom_len),
        };
        offset=bloom_end;

        let buckets_size:u32 = (nbucket * (U32SIZE as u32)) as u32;
        let buckets_end:usize = offset+(buckets_size as usize);
        let buckets_buf = hash_bytes.get(offset..buckets_end).unwrap();
        let buckets = Self::parser_buf_vec(U32SIZE,buckets_buf,endian,buckets_size);
        offset = buckets_end;
        let chains_buf = hash_bytes.get(offset..).unwrap();
        let chains_size=hash_bytes.len()-offset;
        let chains = Self::parser_buf_vec(U32SIZE, chains_buf, endian, chains_size as u32);

        return  hash{
            nbucket,
            symoffset,
            bloom_size,
            bloom_shift,
            bloom,
            buckets,
            chains,
        }

    }


}


