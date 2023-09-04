#[derive(PartialEq,Debug,Clone)]
pub enum Class {
    ELF32,
    ELF64,
}
pub mod file_utils{
    use std::fs::File;
    use std::io::{self, Read, Seek, SeekFrom};

    //read offest file data
    pub fn read_file_range(path: &str, start_offset: u64, end_offset: u64) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;

        // 设置文件的读取偏移量
        file.seek(SeekFrom::Start(start_offset))?;

        // 计算要读取的字节数
        let num_bytes = (end_offset - start_offset + 1) as usize;

        // 创建一个缓冲区来存储读取的字节
        let mut buffer = vec![0; num_bytes];

        // 读取文件中的字节到缓冲区
        file.read_exact(&mut buffer)?;
        Ok(buffer)
    }




}