use std::fmt::{self, Display, Formatter};
use std::io::{stdin, stdout, Read, Write};

#[derive(Debug)]
pub struct Tape {
    index: usize,
    inner: Vec<u8>,
}

impl Default for Tape {
    fn default() -> Self {
        Self {
            index: 0,
            inner: vec![0],
        }
    }
}

impl Display for Tape {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.inner.len() {
            1 => write!(f, "[{:03}]", self.inner[0]),
            _ => {
                if self.index == 0 {
                    write!(f, "[{:03}] {:03}", self.inner[0], self.inner[1])
                } else if self.index == (self.inner.len() - 1) {
                    write!(
                        f,
                        "{:03} [{:03}]",
                        self.inner[self.index - 1],
                        self.inner[self.index]
                    )
                } else {
                    write!(
                        f,
                        "{:03} [{:03}] {:03}",
                        self.inner[self.index - 1],
                        self.inner[self.index],
                        self.inner[self.index + 1]
                    )
                }
            }
        }
    }
}

impl Tape {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_bytes(bytes: &[u8], index: usize) -> Self {
        if index >= bytes.len() {
            panic!("index out of bounds");
        }
        Self {
            index,
            inner: Vec::from(bytes),
        }
    }

    pub fn move_pointer_right(&mut self) {
        self.index = self.index.wrapping_add(1);
        if self.inner.len() != usize::MAX {
            self.inner.push(0);
        }
    }

    pub fn move_pointer_left(&mut self) {
        if self.index != 0 {
            self.index -= 1;
        } else if self.inner.len() != usize::MAX {
            self.inner.insert(0, 0);
        } else {
            self.index = self.index.wrapping_sub(1);
        }
    }

    pub fn increment_cell(&mut self) {
        self.inner[self.index] = self.inner[self.index].wrapping_add(1);
    }

    pub fn decrement_cell(&mut self) {
        self.inner[self.index] = self.inner[self.index].wrapping_sub(1);
    }

    pub fn print_cell(&mut self) {
        stdout().write_all(&[self.inner[self.index]]).unwrap();
    }

    pub fn read_input(&mut self) {
        let mut input_buf = [0u8; 1];
        stdin().read_exact(&mut input_buf).unwrap();
        self.inner[self.index] = input_buf[0];
    }

    pub fn is_cell_zero(&self) -> bool {
        self.inner[self.index] == 0
    }
}
