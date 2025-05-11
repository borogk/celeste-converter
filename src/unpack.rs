use BitDepth::*;
use png::BitDepth;
use crate::math::make_divisible_by;

pub fn unpack(data: &[u8], len: usize, span: usize, bit_depth: BitDepth) -> Vec<u8> {
    match bit_depth {
        One => unpack_one_bit(data, len, span),
        Two => unpack_two_bit(data, len, span),
        Four => unpack_four_bit(data, len, span),
        Eight => unpack_eight_bit(data),
        Sixteen => unpack_sixteen_bit(data),
    }
}

fn unpack_one_bit(data: &[u8], len: usize, span: usize) -> Vec<u8> {
    let mut output = vec![0; data.len() * 8];

    let input_span = make_divisible_by(span, 8) / 8;
    for i in 0..data.len() {
        let packed_byte = data[i];
        let output_offset = i / input_span * span + i % input_span * 8;
        output[output_offset + 0] = (packed_byte >> 7) & 1;
        output[output_offset + 1] = (packed_byte >> 6) & 1;
        output[output_offset + 2] = (packed_byte >> 5) & 1;
        output[output_offset + 3] = (packed_byte >> 4) & 1;
        output[output_offset + 4] = (packed_byte >> 3) & 1;
        output[output_offset + 5] = (packed_byte >> 2) & 1;
        output[output_offset + 6] = (packed_byte >> 1) & 1;
        output[output_offset + 7] = (packed_byte >> 0) & 1;
    }

    output.truncate(len);
    output
}

fn unpack_two_bit(data: &[u8], len: usize, span: usize) -> Vec<u8> {
    let mut output = vec![0; data.len() * 4];

    let input_span = make_divisible_by(span * 2, 8) / 8;
    for i in 0..data.len() {
        let packed_byte = data[i];
        let output_offset = i / input_span * span + i % input_span * 4;
        output[output_offset + 0] = (packed_byte >> 6) & 3;
        output[output_offset + 1] = (packed_byte >> 4) & 3;
        output[output_offset + 2] = (packed_byte >> 2) & 3;
        output[output_offset + 3] = (packed_byte >> 0) & 3;
    }

    output.truncate(len);
    output
}

fn unpack_four_bit(data: &[u8], len: usize, span: usize) -> Vec<u8> {
    let mut output = vec![0; data.len() * 2];

    let input_span = make_divisible_by(span * 4, 8) / 8;
    for i in 0..data.len() {
        let packed_byte = data[i];
        let output_offset = i / input_span * span + i % input_span * 2;
        output[output_offset + 0] = (packed_byte >> 4) & 15;
        output[output_offset + 1] = (packed_byte >> 0) & 15;
    }

    output.truncate(len);
    output
}

fn unpack_eight_bit(data: &[u8]) -> Vec<u8> {
    data.to_vec()
}

fn unpack_sixteen_bit(data: &[u8]) -> Vec<u8> {
    let mut result = vec![0; data.len() / 2];
    for i in 0..result.len() {
        let offset = i * 2;
        result[i] = data[offset];
    }
    result
}
