use std::ops::Add;
use crate::parser::endian::{AnyEndian, EndianParse};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct hash {
    pub nbucket: u32,
    pub nchain: u32,
    pub bucket:*const u32,
    pub chain:*const u32,

}

impl hash {
    pub fn elf_hash(name: &str) -> usize {
        let mut h: u32 = 0;
        for b in name.as_bytes() {
            h = h.wrapping_shl(4).overflowing_add(*b as u32).0;
            let g = h & 0xf0000000;
            if g != 0 {
                h ^= g.wrapping_shr(24);
            }
            h &= !g;
        }
        h as usize
    }
    pub fn parser_hash_tables(endian:AnyEndian,hash_bytes:&[u8])->hash{
        const U64SIZE: usize = core::mem::size_of::<u64>();
        const U32SIZE: usize = core::mem::size_of::<u32>();
        const U16SIZE: usize = core::mem::size_of::<u16>();
        const U8SIZE: usize = core::mem::size_of::<u8>();
        let mut offset=0;
        let nbucket=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let nchain=endian.parse_u32_at(offset,hash_bytes);
        offset+=U32SIZE;
        let bucket = ((hash_bytes.as_ptr() as *const u8 as usize) + 2 * U32SIZE) as *const u32;
        let chain = unsafe { bucket.add(nbucket as usize) };

        return  hash {
            nbucket,
            nchain,
            bucket,
            chain,
        };
    }


}


