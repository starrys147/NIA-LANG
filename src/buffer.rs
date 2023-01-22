use std::io::{Read, Write};

pub struct Buffer {
    buffer: [u8; 1000],
    idx: usize,
}

impl Buffer {
    pub fn new() -> Buffer {
        Buffer {
            buffer: [0; 1000],
            idx: 0,
        }
    }
    pub fn val(&self) -> u8 {
        self.buffer[self.idx]
    }

    pub fn inc(&mut self) {
        if self.idx == 999 {
            self.idx = 0;
        } else {
            self.idx += 1;
        }
    }

    pub fn dec(&mut self) {
        if self.idx == 0 {
            self.idx = 999;
        } else {
            self.idx -= 1;
        }
    }

    pub fn add(&mut self) {
        self.buffer[self.idx] = self.buffer[self.idx].wrapping_add(1);
    }

    pub fn sub(&mut self) {
        self.buffer[self.idx] = self.buffer[self.idx].wrapping_sub(1);
    }

    pub fn getc(&mut self) {
        let mut buf = [0];
        std::io::stdin()
            .read(&mut buf[..])
            .expect("input error: Unexpected input");
        self.buffer[self.idx] = buf[0];
    }

    pub fn putc(&mut self) {
        std::io::stdout()
            .write(&[self.buffer[self.idx]])
            .expect("output error: Unexpected output");
    }
}
