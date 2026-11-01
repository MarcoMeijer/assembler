pub const RAM_SIZE: usize = 65536;

pub struct Ram {
    pub data: [u8; RAM_SIZE],
}

impl Default for Ram {
    fn default() -> Self {
        Self {
            data: [0; RAM_SIZE],
        }
    }
}

impl Ram {
    pub fn get_u8(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn get_u16(&self, address: u16) -> u16 {
        let x: [u8; 2] = self.data[address as usize..address as usize + 2]
            .try_into()
            .unwrap();
        let res = u16::from_be_bytes(x);
        res
    }

    pub fn set_u8(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn set_u16(&mut self, address: u16, value: u16) {
        self.data[address as usize] = (value / 256) as u8;
        self.data[address as usize + 1] = (value % 256) as u8;
    }
}
