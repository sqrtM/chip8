use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Registers {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,
    pub vf: u8,
    pub vi: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: Vec<u16>,
}

pub struct Ram(pub [u8; 4096]);

impl Ram {
    pub fn init() -> Ram {
        let mut r = Ram([0; 4096]);
        r.load("./data/inital_ram_data.chip8");
        r
    }

    fn load(&mut self, path: &str) {
        let file = File::open(path);
        let mut reader = BufReader::new(file.unwrap());
        let mut buffer = vec![0; 1];

        let mut index = 0;
        while reader.read_exact(&mut buffer).is_ok() {
            let number = u8::from_le_bytes(buffer.clone().try_into().unwrap());
            self.0[index + 0x200] = number;
            index += 1;
        }
    }
}
