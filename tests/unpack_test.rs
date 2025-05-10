use BitDepth::*;
use png::BitDepth;
use rstest::rstest;
use celeste_converter::unpack::PackedData;

#[rstest]
fn one_bit_empty() {
    let data = [];
    let unpacked = PackedData::new(data.as_slice(), 0, One).unpack();
    assert_eq!(unpacked, []);
}

#[rstest]
fn one_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = PackedData::new(data.as_slice(), 8, One).unpack();
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1]);
}

#[rstest]
fn one_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = PackedData::new(data.as_slice(), 16, One).unpack();
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0]);
}

#[rstest]
fn one_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = PackedData::new(data.as_slice(), 18, One).unpack();
    assert_eq!(unpacked, [1, 0, 1, 0, 0, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 0, 1, 0]);
}

#[rstest]
fn two_bit_empty() {
    let data = [];
    let unpacked = PackedData::new(data.as_slice(), 0, Two).unpack();
    assert_eq!(unpacked, []);
}

#[rstest]
fn two_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = PackedData::new(data.as_slice(), 4, Two).unpack();
    assert_eq!(unpacked, [1, 1, 2, 3]);
}

#[rstest]
fn two_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = PackedData::new(data.as_slice(), 8, Two).unpack();
    assert_eq!(unpacked, [1, 1, 2, 3, 2, 2, 1, 0]);
}

#[rstest]
fn two_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = PackedData::new(data.as_slice(), 9, Two).unpack();
    assert_eq!(unpacked, [1, 1, 2, 3, 2, 2, 1, 0, 1]);
}

#[rstest]
fn four_bit_empty() {
    let data = [];
    let unpacked = PackedData::new(data.as_slice(), 0, Four).unpack();
    assert_eq!(unpacked, []);
}

#[rstest]
fn four_bit_single_byte() {
    let data = [0b11100101];
    let unpacked = PackedData::new(data.as_slice(), 2, Four).unpack();
    assert_eq!(unpacked, [5, 14]);
}

#[rstest]
fn four_bit_multiple_bytes() {
    let data = [0b11100101, 0b00011010];
    let unpacked = PackedData::new(data.as_slice(), 4, Four).unpack();
    assert_eq!(unpacked, [5, 14, 10, 1]);
}

#[rstest]
fn four_bit_multiple_bytes_with_remainder() {
    let data = [0b11100101, 0b00011010, 0b00000001];
    let unpacked = PackedData::new(data.as_slice(), 5, Four).unpack();
    assert_eq!(unpacked, [5, 14, 10, 1, 1]);
}
