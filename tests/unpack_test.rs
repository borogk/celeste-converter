use BitDepth::*;
use png::BitDepth;
use rstest::rstest;
use celeste_converter::unpack::unpack;

#[rstest]
fn one_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, 0, 0, One);
    assert_eq!(unpacked, []);
}

#[rstest]
fn one_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 8, 1, 1, One);
    assert_eq!(unpacked, [1, 1, 1, 0, 0, 1, 0, 1]);
}

#[rstest]
fn one_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 16, 1, 2, One);
    assert_eq!(unpacked, [1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0]);
}

#[rstest]
fn one_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b10000000];
    let unpacked = unpack(&data, 18, 1, 3, One);
    assert_eq!(unpacked, [1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0]);
}

#[rstest]
fn one_bit_multiple_bytes_with_multiple_remainders() {
    let data = [
        0b11100101, 0b00011010, 0b10000000,
        0b11100101, 0b00011010, 0b10000000
    ];
    let unpacked = unpack(&data, 18, 2, 3, One);
    assert_eq!(unpacked, [
        1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0,
        1, 1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0,
    ]);
}

#[rstest]
fn two_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, 0, 0, Two);
    assert_eq!(unpacked, []);
}

#[rstest]
fn two_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 4, 1, 1, Two);
    assert_eq!(unpacked, [3, 2, 1, 1]);
}

#[rstest]
fn two_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 8, 1, 2, Two);
    assert_eq!(unpacked, [3, 2, 1, 1, 0, 1, 2, 2]);
}

#[rstest]
fn two_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b10000000];
    let unpacked = unpack(&data, 9, 1, 3, Two);
    assert_eq!(unpacked, [3, 2, 1, 1, 0, 1, 2, 2, 2]);
}

#[rstest]
fn two_bit_multiple_bytes_with_multiple_remainders() {
    let data = [
        0b11100101, 0b00011010, 0b10000000,
        0b11100101, 0b00011010, 0b10000000
    ];
    let unpacked = unpack(&data, 9, 2, 3, Two);
    assert_eq!(unpacked, [
        3, 2, 1, 1, 0, 1, 2, 2, 2,
        3, 2, 1, 1, 0, 1, 2, 2, 2
    ]);
}

#[rstest]
fn four_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, 0, 0, Four);
    assert_eq!(unpacked, []);
}

#[rstest]
fn four_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 2, 1, 1, Four);
    assert_eq!(unpacked, [14, 5]);
}

#[rstest]
fn four_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 4, 1, 2, Four);
    assert_eq!(unpacked, [14, 5, 1, 10]);
}

#[rstest]
fn four_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b10000000];
    let unpacked = unpack(&data, 5, 1, 3, Four);
    assert_eq!(unpacked, [14, 5, 1, 10, 8]);
}

#[rstest]
fn four_bit_multiple_bytes_with_multiple_remainders() {
    let data = [
        0b11100101, 0b00011010, 0b10000000,
        0b11100101, 0b00011010, 0b10000000
    ];
    let unpacked = unpack(&data, 5, 2, 3, Four);
    assert_eq!(unpacked, [
        14, 5, 1, 10, 8,
        14, 5, 1, 10, 8
    ]);
}

#[rstest]
fn eight_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, 0, 0, Eight);
    assert_eq!(unpacked, []);
}

#[rstest]
fn eight_bit_single_byte() {
    let data = [100];
    let unpacked = unpack(&data, 1, 1, 1, Eight);
    assert_eq!(unpacked, [100]);
}

#[rstest]
fn eight_bit_multiple_bytes() {
    let data = [100, 200];
    let unpacked = unpack(&data, 2, 1, 2, Eight);
    assert_eq!(unpacked, [100, 200]);
}

#[rstest]
fn sixteen_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, 0, 0, Sixteen);
    assert_eq!(unpacked, []);
}

#[rstest]
fn sixteen_bit_single_byte() {
    let data = [0x64, 0x00];
    let unpacked = unpack(&data, 1, 1, 2, Sixteen);
    assert_eq!(unpacked, [100]);
}

#[rstest]
fn sixteen_bit_multiple_bytes() {
    let data = [0x64, 0x00, 0xC8, 0x00];
    let unpacked = unpack(&data, 2, 1, 4, Sixteen);
    assert_eq!(unpacked, [100, 200]);
}

#[rstest]
fn sixteen_bit_zeroes() {
    let data = [0x00, 0x00, 0x00, 0x00];
    let unpacked = unpack(&data, 2, 1, 4, Sixteen);
    assert_eq!(unpacked, [0, 0]);
}

#[rstest]
fn sixteen_bit_max_values() {
    let data = [0xFF, 0xFF, 0xFF, 0xFF];
    let unpacked = unpack(&data, 2, 1, 4, Sixteen);
    assert_eq!(unpacked, [255, 255]);
}

#[rstest]
fn sixteen_bit_precision_loss() {
    let data = [0x00, 0xAA, 0x64, 0xBB, 0xC8, 0xCC, 0xFF, 0xDD];
    let unpacked = unpack(&data, 4, 1, 8, Sixteen);
    assert_eq!(unpacked, [0, 100, 200, 255]);
}
