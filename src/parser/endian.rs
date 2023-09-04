use std::string::ParseError;
use crate::parser::abi;
#[derive(Debug,Copy,Clone,PartialEq)]
pub struct  AnyEndian {
    pub endian_type:u8,
}


pub trait EndianParse{
    fn new(endian_type:u8) -> Self;
    //解析大小端的文件处理 后续考虑用宏来解决
    fn parse_u8_at(self,offset:usize, data: &[u8]) ->u8 where Self: Sized{
        const SIZE: usize = core::mem::size_of::<u8>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();


        if Self::is_little(self){
            return u8::from_le_bytes(buf);
        }else {
            return u8::from_be_bytes(buf)
        }

    }
    fn parse_u16_at(self,offset: usize, data: &[u8]) ->u16 where Self: Sized{
        const SIZE: usize = core::mem::size_of::<u16>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();

        if Self::is_little(self){
            return u16::from_le_bytes(buf);
        }else {
            return u16::from_be_bytes(buf)
        }
    }

    fn parse_u32_at(self,offset:usize, data: &[u8]) ->u32 where Self: Sized{
        const SIZE: usize = core::mem::size_of::<u32>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();


        if Self::is_little(self){
            return u32::from_le_bytes(buf);
        }else {
            return u32::from_be_bytes(buf)
        }
    }

    fn parse_u64_at(self,offset:usize, data: &[u8]) ->u64 where Self: Sized{
        const SIZE: usize = core::mem::size_of::<u64>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();


        if Self::is_little(self){
            return u64::from_le_bytes(buf);
        }else {
            return u64::from_be_bytes(buf)
        }
    }

    fn parse_i32_at(self, offset:usize, data: &[u8]) -> i32 where Self: Sized {
        const SIZE: usize = core::mem::size_of::<i32>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();


        if Self::is_little(self){
            return i32::from_le_bytes(buf);
        }else {
            return i32::from_be_bytes(buf)
        }
    }

    fn parse_i64_at(self,offset: usize, data: &[u8]) -> i64 where Self: Sized{
        const SIZE: usize = core::mem::size_of::<i64>();
        let end=offset+SIZE;
        let opbuf= data.get(offset..end);
        let buf = opbuf.unwrap().try_into().unwrap();


        if Self::is_little(self){
            return i64::from_le_bytes(buf);
        }else {
            return i64::from_be_bytes(buf)
        }
    }



    fn is_little(self) -> bool;

    fn is_big(self) -> bool where Self: Sized{
        !Self::is_little(self)
    }
}

impl EndianParse for AnyEndian {
    fn new(endian_type: u8) -> Self {
       return AnyEndian{endian_type};
    }
    fn is_little(self) -> bool {
        match  self.endian_type{
            1 => return true,
            2 => return false,
            _ => return false
        }
    }
}
