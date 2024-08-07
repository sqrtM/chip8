use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Registers {
    pub r: [u8; 16],
    pub vi: u16,
    pub delay: u8,
    pub sound: u8,
    pub pc: u16,
    pub sp: u8,
    pub stack: Vec<u16>,
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            r: [0; 16],
            vi: 0,
            delay: 0,
            sound: 0,
            pc: 0x200,
            sp: 0,
            stack: Vec::new(),
        }
    }
}

pub struct Ram(pub [u8; 4096]);

impl Ram {
    pub fn init() -> Ram {
        let mut r = Ram([0; 4096]);
        r.load("./data/inital_ram_data.chip8");
        r
    }

    pub fn load(&mut self, path: &str) {
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

impl Default for Ram {
    fn default() -> Self {
        Ram([0; 4096])
    }
}
