use std::panic::resume_unwind;
use aoc_iter_tools::{BitArray, ByteArray};

pub struct MessageDigest {
}

impl MessageDigest {

    pub fn new() -> Self {
        MessageDigest {
        }
    }

    pub fn v5(&self, message: &[u8]) -> u128 {
        let length = ((message.len() as f32 / 512.0).ceil() * 512.0) as usize;
        let mut buffer = BitArray::with_capacity(length * 8);
        let mut registers = [0x67452301u32, 0xefcfab89u32, 0x98badcfeu32, 0x10325476u32];

        let mut table = [0; 64];
        for i in 0..64 {
            table[i] = (4294967296f64 * ((i + 1) as f64).sin().abs()) as u32
        }

        buffer.set(0, &message);
        buffer.set(message.len(), &[1]);
        buffer.set(length - 64, &(message.len() as u64).to_ne_bytes());

        for x in buffer.chunked(64) {
            let mut _f = 0;
            let mut _g = 0;
            let mut _r = registers.clone();
            let mut _s = [0; 64];
            for i in 0..63 {
                match i {
                    0..=15 => {
                        _f = f(_r[1], _r[2], _r[3]);
                        _g = i;
                        _s[i] = [7, 12, 17, 22][i % 4];
                    },
                    16..=31 => {
                        _f = g(_r[1], _r[2], _r[3]);
                        _g = (5*i + 1) % 16;
                        _s[i] = [5, 9, 14, 20][i % 4];
                    },
                    32..=47 => {
                        _f = h(_r[1], _r[2], _r[3]);
                        _g = (3*i + 5) % 16;
                        _s[i] = [4, 11, 16, 23][i % 4];
                    },
                    48..=63 => {
                        _f = j(_r[1], _r[2], _r[3]);
                        _g = (7*i) % 16;
                        _s[i] = [6, 10, 15, 21][i % 4];
                    },
                    _ => {}
                }
                println!("--------\n{}", _r[0]);
                _f = _f + _r[0] + table[i] + (((x[_g*4] as u32) << 24) + ((x[_g*4 + 1] as u32) << 16) + ((x[_g*4 + 2] as u32) << 8) + ((x[_g*4 + 3] as u32)));
                _r[0] = _r[3];
                _r[3] = _r[2];
                _r[2] = _r[1];
                _r[1] = _r[1] + _f.rotate_left(_s[i])
            }
            registers[0] += _r[0];
            registers[1] += _r[1];
            registers[2] += _r[2];
            registers[3] += _r[3];
        }

        let a: u128 = (((registers[0] as u128) << 96) + ((registers[1] as u128) << 64) + ((registers[2] as u128) << 32) + (registers[3] as u128));
        return a;

        fn f(b: u32, c: u32, d: u32) -> u32 {
            (b & c) | (!b & d)
        }
        fn g(b: u32, c: u32, d: u32) -> u32 {
            (b & d) | (c & !d)
        }
        fn h(b: u32, c: u32, d: u32) -> u32 {
            b^c^d
        }
        fn j(b: u32, c: u32, d: u32) -> u32 {
            c ^ (b | !d)
        }
    }
}
