use BitDepth::*;
use png::BitDepth;

pub fn unpack(
    data: &[u8],
    width: usize,
    height: usize,
    line_size: usize,
    bit_depth: BitDepth
) -> Vec<u8> {
    match bit_depth {
        One => unpack_one_bit(data, width, height, line_size),
        Two => unpack_two_bit(data, width, height, line_size),
        Four => unpack_four_bit(data, width, height, line_size),
        Eight => unpack_eight_bit(data),
        Sixteen => unpack_sixteen_bit(data),
    }
}

fn unpack_one_bit(data: &[u8], width: usize, height: usize, line_size: usize) -> Vec<u8> {
    let mut output = vec![0; line_size * height * 8];
    for line in 0..height {
        for i in 0..line_size {
            let packed_byte = data[line * line_size + i];
            let output_offset = line * width + i * 8;
            output[output_offset + 0] = (packed_byte >> 7) & 1;
            output[output_offset + 1] = (packed_byte >> 6) & 1;
            output[output_offset + 2] = (packed_byte >> 5) & 1;
            output[output_offset + 3] = (packed_byte >> 4) & 1;
            output[output_offset + 4] = (packed_byte >> 3) & 1;
            output[output_offset + 5] = (packed_byte >> 2) & 1;
            output[output_offset + 6] = (packed_byte >> 1) & 1;
            output[output_offset + 7] = (packed_byte >> 0) & 1;
        }
    }

    output.truncate(width * height);
    output
}

fn unpack_two_bit(data: &[u8], width: usize, height: usize, line_size: usize) -> Vec<u8> {
    let mut output = vec![0; line_size * height * 4];
    for line in 0..height {
        for i in 0..line_size {
            let packed_byte = data[line * line_size + i];
            let output_offset = line * width + i * 4;
            output[output_offset + 0] = (packed_byte >> 6) & 3;
            output[output_offset + 1] = (packed_byte >> 4) & 3;
            output[output_offset + 2] = (packed_byte >> 2) & 3;
            output[output_offset + 3] = (packed_byte >> 0) & 3;
        }
    }

    output.truncate(width * height);
    output
}

fn unpack_four_bit(data: &[u8], width: usize, height: usize, line_size: usize) -> Vec<u8> {
    let mut output = vec![0; line_size * height * 2];
    for line in 0..height {
        for i in 0..line_size {
            let packed_byte = data[line * line_size + i];
            let output_offset = line * width + i * 2;
            output[output_offset + 0] = (packed_byte >> 4) & 15;
            output[output_offset + 1] = (packed_byte >> 0) & 15;
        }
    }

    output.truncate(width * height);
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
