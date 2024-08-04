use std::fs::File;
use std::io::BufReader;
use std::io::Read;

struct Registers {
    v0: u8,
    v1: u8,
    v2: u8,
    v3: u8,
    v4: u8,
    v5: u8,
    v6: u8,
    v7: u8,
    v8: u8,
    v9: u8,
    va: u8,
    vb: u8,
    vc: u8,
    vd: u8,
    ve: u8,
    vf: u8,
    vi: u16,
    delay: u8,
    sound: u8,
    pc: u16,
    sp: u8,
    stack: [u16; 16],
}

pub struct Ram([u8; 4096]);

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
