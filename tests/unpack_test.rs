use BitDepth::*;
use png::BitDepth;
use rstest::rstest;
use celeste_converter::unpack::unpack;

#[rstest]
fn one_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, One);
    assert_eq!(unpacked, []);
}

#[rstest]
fn one_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 8, One);
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1]);
}

#[rstest]
fn one_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 16, One);
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0]);
}

#[rstest]
fn one_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = unpack(&data, 18, One);
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0]);
}

#[rstest]
fn two_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, Two);
    assert_eq!(unpacked, []);
}

#[rstest]
fn two_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 4, Two);
    assert_eq!(unpacked, [1, 1, 2, 3]);
}

#[rstest]
fn two_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 8, Two);
    assert_eq!(unpacked, [1, 1, 2, 3, 2, 2, 1, 0]);
}

#[rstest]
fn two_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = unpack(&data, 9, Two);
    assert_eq!(unpacked, [1, 1, 2, 3, 2, 2, 1, 0, 1]);
}

#[rstest]
fn four_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, Four);
    assert_eq!(unpacked, []);
}

#[rstest]
fn four_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = unpack(&data, 2, Four);
    assert_eq!(unpacked, [5, 14]);
}

#[rstest]
fn four_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = unpack(&data, 4, Four);
    assert_eq!(unpacked, [5, 14, 10, 1]);
}

#[rstest]
fn four_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = unpack(&data, 5, Four);
    assert_eq!(unpacked, [5, 14, 10, 1, 1]);
}

#[rstest]
fn eight_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, Eight);
    assert_eq!(unpacked, []);
}

#[rstest]
fn eight_bit_single_byte() {
    let data = [100];
    let unpacked = unpack(&data, 1, Eight);
    assert_eq!(unpacked, [100]);
}

#[rstest]
fn eight_bit_multiple_bytes() {
    let data = [100, 200];
    let unpacked = unpack(&data, 2, Eight);
    assert_eq!(unpacked, [100, 200]);
}

#[rstest]
fn sixteen_bit_empty() {
    let data = [];
    let unpacked = unpack(&data, 0, Sixteen);
    assert_eq!(unpacked, []);
}

#[rstest]
fn sixteen_bit_single_byte() {
    let data = [0x64, 0x64];
    let unpacked = unpack(&data, 1, Sixteen);
    assert_eq!(unpacked, [100]);
}

#[rstest]
fn sixteen_bit_multiple_bytes() {
    let data = [0x64, 0x64, 0xC8, 0xC8];
    let unpacked = unpack(&data, 2, Sixteen);
    assert_eq!(unpacked, [100, 200]);
}

#[rstest]
fn sixteen_bit_zeroes() {
    let data = [0x00, 0x00, 0x00, 0x00];
    let unpacked = unpack(&data, 2, Sixteen);
    assert_eq!(unpacked, [0, 0]);
}

#[rstest]
fn sixteen_bit_max_values() {
    let data = [0xFF, 0xFF, 0xFF, 0xFF];
    let unpacked = unpack(&data, 2, Sixteen);
    assert_eq!(unpacked, [255, 255]);
}

#[rstest]
fn sixteen_bit_precision_loss() {
    let data = [
        0x00, 0x0A, // slightly above 0
        0x01, 0xF8, // slightly below 2
        0x64, 0x82, // slightly above 100
        0xC8, 0xA0, // slightly below 200
        0xFF, 0xDC, // slightly below max
    ];
    let unpacked = unpack(&data, 5, Sixteen);
    assert_eq!(unpacked, [0, 1, 100, 199, 254]);
}
