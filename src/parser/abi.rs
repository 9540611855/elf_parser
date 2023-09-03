pub const EI_NIDENT: usize = 16;
/// ELF magic number byte 1
pub const ELFMAG0: u8 = 0x7f;
/// ELF magic number byte 2
pub const ELFMAG1: u8 = 0x45;
/// ELF magic number byte 3
pub const ELFMAG2: u8 = 0x4c;
/// ELF magic number byte 4
pub const ELFMAG3: u8 = 0x46;
pub const ELFMAGIC: [u8; 4] = [ELFMAG0, ELFMAG1, ELFMAG2, ELFMAG3];
pub const EI_CLASS: usize = 4;
/// Location of data format field in ELF file header ident array
pub const EI_DATA: usize = 5;
/// Invalid ELF data format
pub const ELFDATANONE: u8 = 0;
/// 2's complement values, with the least significant byte occupying the lowest address.
pub const ELFDATA2LSB: u8 = 1;
/// 2's complement values, with the most significant byte occupying the lowest address.
pub const ELFDATA2MSB: u8 = 2;