#[derive(Debug, Copy, Clone)]
pub struct Rom {
    pub rom_data: [u8; 1024],
    //0x0000-0x03FF: ROM whole region
}

impl Rom {
    pub fn new() -> Rom {
        return Rom {
            rom_data: [0; 1024],
        };
    }
    pub fn write_example(&mut self) {
        let example: [u8; 88] = [
            0x13, 0x05, 0x00, 0x20, //
            0x93, 0x05, 0x40, 0x00, //
            0x93, 0x06, 0x40, 0x20, //
            0x13, 0x07, 0x10, 0x00, //
            0x63, 0x64, 0xb7, 0x00, //
            0x6F, 0x00, 0x00, 0x04, //
            0x03, 0xa8, 0x06, 0x00, //
            0x13, 0x86, 0x06, 0x00, //
            0x93, 0x07, 0x07, 0x00, //
            0x83, 0x28, 0xC6, 0xFF, //
            0x63, 0x5a, 0x18, 0x01, //
            0x23, 0x20, 0x16, 0x01, //
            0x93, 0x87, 0xF7, 0xFF, //
            0x13, 0x06, 0xC6, 0xFF, //
            0xE3, 0x96, 0x07, 0xFE, //
            0x93, 0x97, 0x27, 0x00, //
            0xB3, 0x07, 0xF5, 0x00, //
            0x23, 0xA0, 0x07, 0x01, //
            0x13, 0x07, 0x17, 0x00, //
            0x93, 0x86, 0x46, 0x00, //
            0x6F, 0xF0, 0x1F, 0xFC, //
            0x6F, 0x00, 0x00, 0x00, //
        ];
        let data: [u8; 16] = [
            0x06, 0x00, 0x00, 0x00, //
            0x04, 0x00, 0x00, 0x00, //
            0x07, 0x00, 0x00, 0x00, //
            0x01, 0x00, 0x00, 0x00,
        ];

        for i in 0..example.len() {
            self.rom_data[i] = example[i];
        }
        for i in 0..data.len() {
            self.rom_data[0x0200 + i] = data[i];
        }
        return;
    }
}
