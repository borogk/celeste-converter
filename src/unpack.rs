use BitDepth::*;
use png::BitDepth;

pub fn unpack(data: &[u8], len: usize, bit_depth: BitDepth) -> Vec<u8> {
    match bit_depth {
        One => unpack_one_bit(&data[..len / 8 + (len % 8 != 0) as usize], len),
        Two => unpack_two_bit(&data[..len / 4 + (len % 4 != 0) as usize], len),
        Four => unpack_four_bit(&data[..len / 2 + (len % 2 != 0) as usize], len),
        Eight => unpack_eight_bit(&data[..len]),
        Sixteen => unpack_sixteen_bit(&data[..len * 2]),
    }
}

fn unpack_one_bit(data: &[u8], len: usize) -> Vec<u8> {
    let mut result = vec![0; data.len() * 8];
    for i in 0..data.len() {
        let packed_byte = data[i];
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

    result.truncate(len);
    result
}

fn unpack_two_bit(data: &[u8], len: usize) -> Vec<u8> {
    let mut result = vec![0; data.len() * 4];
    for i in 0..data.len() {
        let packed_byte = data[i];
        let offset = i * 4;
        result[offset + 0] = (packed_byte >> 0) & 3;
        result[offset + 1] = (packed_byte >> 2) & 3;
        result[offset + 2] = (packed_byte >> 4) & 3;
        result[offset + 3] = (packed_byte >> 6) & 3;
    }

    result.truncate(len);
    result
}

fn unpack_four_bit(data: &[u8], len: usize) -> Vec<u8> {
    let mut result = vec![0; data.len() * 2];
    for i in 0..data.len() {
        let packed_byte = data[i];
        let offset = i * 2;
        result[offset + 0] = (packed_byte >> 0) & 15;
        result[offset + 1] = (packed_byte >> 4) & 15;
    }

    result.truncate(len);
    result
}

fn unpack_eight_bit(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

fn unpack_sixteen_bit(data: &[u8]) -> Vec<u8> {
    let mut result = vec![0; data.len() / 2];
    for i in 0..result.len() {
        let offset = i * 2;
        let big_value = ((data[offset] as u16) << 8) | (data[offset + 1] as u16);
        result[i] = (big_value / 257) as u8;
    }
    result
}
