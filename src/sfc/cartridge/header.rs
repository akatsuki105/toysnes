use bytesize::ByteSize;
use std::fmt;

#[derive(Debug)]
pub struct Header {
    /// FFC0..FFD4h
    pub title: [u8; 21],

    /// FFD5h
    pub map_mode: u8,

    /// FFD6h
    pub chipset: u8,

    /// FFD7h
    pub rom_size: u8,

    /// FFD8h
    pub ram_size: u8,

    /// FFD9h
    pub destination: u8,

    /// FFDAh
    pub maker: u8,

    /// FFDBh
    pub version: u8,

    /// FFDC..FFDDh
    pub checksumc: [u8; 2],

    /// FFDE..FFDFh
    pub checksum: [u8; 2],
}

impl Header {
    pub fn new(rom_data: &[u8]) -> Self {
        let rom_size = rom_data.len();

        let mut offset = 0xffc0;
        if offset >= rom_size {
            offset = 0x7fc0;
        }
        if rom_size & 0x3ff == 0x200 {
            offset += 0x200
        }

        let mut rom_header = &rom_data[offset..offset + 32];

        let mut title: [u8; 21] = rom_header[0..21].try_into().unwrap();
        let utf8 = String::from_utf8(title.to_vec());
        if utf8.is_err() {
            let mut offset = 0x7fc0;
            if rom_size & 0x3ff == 0x200 {
                offset += 0x200
            }
            rom_header = &rom_data[offset..offset + 32];
            title = rom_header[0..21].try_into().unwrap();
        }

        Self {
            title: title,
            map_mode: rom_header[0x15],
            chipset: rom_header[0x16],
            rom_size: rom_header[0x17],
            ram_size: rom_header[0x18],
            destination: rom_header[0x19],
            maker: rom_header[0x1a],
            version: rom_header[0x1b],
            checksumc: [0; 2],
            checksum: [0; 2],
        }
    }
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let title = String::from_utf8(self.title.to_vec()).unwrap();
        let rom_size = ByteSize((1 << self.rom_size) as u64 * 1024);
        let ram_size = ByteSize((1 << self.ram_size) as u64 * 1024);

        let ram_size = if self.ram_size > 0 {
            ram_size.to_string()
        } else {
            String::from("None")
        };
        let checksum_c = ((self.checksumc[1] as u16) << 8) | (self.checksumc[0] as u16);
        let checksum = ((self.checksum[1] as u16) << 8) | (self.checksum[0] as u16);

        write!(
            f,
            "Title: {}
    ROM Size:        {}
    RAM Size:        {}
    Destination:     {}
    Maker:           {}
    Version:         v1.{}
    Checksum:        {}(Complement: {})",
            title,
            rom_size,
            ram_size,
            self.destination,
            self.maker,
            self.version,
            format!("{:#06x}", checksum),
            format!("{:#06x}", checksum_c),
        )
    }
}

impl Default for Header {
    fn default() -> Self {
        Self {
            title: [0; 21],
            map_mode: 0,
            chipset: 0,
            rom_size: 0,
            ram_size: 0,
            destination: 0,
            maker: 0,
            version: 0,
            checksumc: [0; 2],
            checksum: [0; 2],
        }
    }
}
