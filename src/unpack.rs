use BitDepth::*;
use png::BitDepth;

pub struct PackedData<'a> {
    data: &'a [u8],
    len: usize,
    bit_depth: BitDepth,
}

impl PackedData<'_> {
    pub fn new(data: &[u8], len: usize, bit_depth: BitDepth) -> PackedData {
        let data = match bit_depth {
            One => &data[..len / 8 + (len % 8 != 0) as usize],
            Two => &data[..len / 4 + (len % 4 != 0) as usize],
            Four => &data[..len / 2 + (len % 2 != 0) as usize],
            Eight => &data[..len],
            Sixteen => &data[..len * 2],
        };

        PackedData {
            data,
            len,
            bit_depth,
        }
    }

    pub fn unpack(&mut self) -> Vec<u8> {
        match self.bit_depth {
            One => self.unpack_one_bit(),
            Two => self.unpack_two_bit(),
            Four => self.unpack_four_bit(),
            Eight => self.unpack_eight_bit(),
            Sixteen => self.unpack_sixteen_bit(),
        }
    }

    fn unpack_one_bit(&self) -> Vec<u8> {
        let mut result = vec![0; self.data.len() * 8];
        for i in 0..self.data.len() {
            let packed_byte = self.data[i];
            let offset = i * 8;
            result[offset + 0] = (packed_byte >> 0) & 1;
            result[offset + 1] = (packed_byte >> 1) & 1;
            result[offset + 2] = (packed_byte >> 2) & 1;
            result[offset + 3] = (packed_byte >> 3) & 1;
            result[offset + 4] = (packed_byte >> 4) & 1;
            result[offset + 5] = (packed_byte >> 5) & 1;
            result[offset + 6] = (packed_byte >> 6) & 1;
            result[offset + 7] = (packed_byte >> 7) & 1;
        }

        result.truncate(self.len);
        result
    }

    fn unpack_two_bit(&self) -> Vec<u8> {
        let mut result = vec![0; self.data.len() * 4];
        for i in 0..self.data.len() {
            let packed_byte = self.data[i];
            let offset = i * 4;
            result[offset + 0] = (packed_byte >> 0) & 3;
            result[offset + 1] = (packed_byte >> 2) & 3;
            result[offset + 2] = (packed_byte >> 4) & 3;
            result[offset + 3] = (packed_byte >> 6) & 3;
        }

        result.truncate(self.len);
        result
    }

    fn unpack_four_bit(&self) -> Vec<u8> {
        let mut result = vec![0; self.data.len() * 2];
        for i in 0..self.data.len() {
            let packed_byte = self.data[i];
            let offset = i * 2;
            result[offset + 0] = (packed_byte >> 0) & 15;
            result[offset + 1] = (packed_byte >> 4) & 15;
        }

        result.truncate(self.len);
        result
    }

    fn unpack_eight_bit(&self) -> Vec<u8> {
        self.data.to_vec()
    }

    fn unpack_sixteen_bit(&self) -> Vec<u8> {
        let mut result = vec![0; self.data.len() / 2];
        for i in 0..result.len() {
            let offset = i * 2;
            let big_value = ((self.data[offset] as u16) << 8) | (self.data[offset + 1] as u16);
            result[i] = (big_value / 257) as u8;
        }
        result
    }
}
