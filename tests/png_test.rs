use ColorType::*;
use png::BitDepth::*;
use png::ColorType;
use rstest::rstest;
use celeste_converter::png::Png;

#[rstest]
fn as_chunk_inherits_png_data() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];
    let png = Png::new(2, 3, Grayscale, Eight, data, None).unwrap();
    let png_chunk = png.as_chunk();

    assert_eq!(png_chunk.data, [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF]);
    assert_eq!(png_chunk.len, 6);
}

#[rstest]
fn one_bit_into_single_chunk() {
    let data = vec![0b00001111];
    let png = Png::new(8, 1, Grayscale, One, data, None).unwrap();
    let png_chunks = png.chunks(8);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 8);
}

#[rstest]
fn one_bit_into_multiple_chunks() {
    let data = vec![0b00001111, 0b10101010];
    let png = Png::new(16, 1, Grayscale, One, data, None).unwrap();
    let png_chunks = png.chunks(8);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 8);
    assert_eq!(png_chunks[1].data, [0b10101010]);
    assert_eq!(png_chunks[1].len, 8);
}

#[rstest]
fn one_bit_into_multiple_chunks_with_remainder() {
    let data = vec![0b00001111, 0b10101010, 0b01000000];
    let png = Png::new(24, 1, Grayscale, One, data, None).unwrap();
    let png_chunks = png.chunks(16);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10101010]);
    assert_eq!(png_chunks[0].len, 16);
    assert_eq!(png_chunks[1].data, [0b01000000]);
    assert_eq!(png_chunks[1].len, 8);
}

#[rstest]
fn one_bit_into_multiple_chunks_with_width_not_divisible_by_eight() {
    let data = vec![0b00001111, 0b10100000, 0b10101010, 0b10000000];
    let png = Png::new(12, 2, Grayscale, One, data, None).unwrap();
    let png_chunks = png.chunks(8);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10100000]);
    assert_eq!(png_chunks[0].len, 12);
    assert_eq!(png_chunks[1].data, [0b10101010, 0b10000000]);
    assert_eq!(png_chunks[1].len, 12);
}

#[rstest]
fn two_bit_into_single_chunk() {
    let data = vec![0b00001111];
    let png = Png::new(4, 1, Grayscale, Two, data, None).unwrap();
    let png_chunks = png.chunks(4);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 4);
}

#[rstest]
fn two_bit_into_multiple_chunks() {
    let data = vec![0b00001111, 0b10101010];
    let png = Png::new(8, 1, Grayscale, Two, data, None).unwrap();
    let png_chunks = png.chunks(4);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 4);
    assert_eq!(png_chunks[1].data, [0b10101010]);
    assert_eq!(png_chunks[1].len, 4);
}

#[rstest]
fn two_bit_into_multiple_chunks_with_remainder() {
    let data = vec![0b00001111, 0b10101010, 0b01000000];
    let png = Png::new(12, 1, Grayscale, Two, data, None).unwrap();
    let png_chunks = png.chunks(8);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10101010]);
    assert_eq!(png_chunks[0].len, 8);
    assert_eq!(png_chunks[1].data, [0b01000000]);
    assert_eq!(png_chunks[1].len, 4);
}

#[rstest]
fn two_bit_into_multiple_chunks_with_width_not_divisible_by_four() {
    let data = vec![0b00001111, 0b10100000, 0b10101010, 0b10000000];
    let png = Png::new(6, 2, Grayscale, Two, data, None).unwrap();
    let png_chunks = png.chunks(4);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10100000]);
    assert_eq!(png_chunks[0].len, 6);
    assert_eq!(png_chunks[1].data, [0b10101010, 0b10000000]);
    assert_eq!(png_chunks[1].len, 6);
}

#[rstest]
fn four_bit_into_single_chunk() {
    let data = vec![0b00001111];
    let png = Png::new(2, 1, Grayscale, Four, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 2);
}

#[rstest]
fn four_bit_into_multiple_chunks() {
    let data = vec![0b00001111, 0b10101010];
    let png = Png::new(4, 1, Grayscale, Four, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0b10101010]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn four_bit_into_multiple_chunks_with_remainder() {
    let data = vec![0b00001111, 0b10101010, 0b01000000];
    let png = Png::new(6, 1, Grayscale, Four, data, None).unwrap();
    let png_chunks = png.chunks(4);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10101010]);
    assert_eq!(png_chunks[0].len, 4);
    assert_eq!(png_chunks[1].data, [0b01000000]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn four_bit_into_multiple_chunks_with_width_not_divisible_by_two() {
    let data = vec![0b00001111, 0b10100000, 0b10101010, 0b10000000];
    let png = Png::new(3, 2, Grayscale, Four, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0b00001111, 0b10100000]);
    assert_eq!(png_chunks[0].len, 3);
    assert_eq!(png_chunks[1].data, [0b10101010, 0b10000000]);
    assert_eq!(png_chunks[1].len, 3);
}

#[rstest]
fn eight_bit_single_channel_into_single_chunk() {
    let data = vec![0xAA, 0xBB];
    let png = Png::new(1, 2, Grayscale, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0xAA, 0xBB]);
    assert_eq!(png_chunks[0].len, 2);
}

#[rstest]
fn eight_bit_single_channel_into_multiple_chunks() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD];
    let png = Png::new(1, 4, Grayscale, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0xAA, 0xBB]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0xDD]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn eight_bit_single_channel_into_multiple_chunks_with_remainder() {
    let data = vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE];
    let png = Png::new(1, 5, Grayscale, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 3);
    assert_eq!(png_chunks[0].data, [0xAA, 0xBB]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0xDD]);
    assert_eq!(png_chunks[1].len, 2);
    assert_eq!(png_chunks[2].data, [0xEE]);
    assert_eq!(png_chunks[2].len, 1);
}

#[rstest]
fn eight_bit_double_channel_into_single_chunk() {
    let data = vec![0xAA, 0x01, 0xBB, 0x02];
    let png = Png::new(1, 2, GrayscaleAlpha, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0xBB, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
}

#[rstest]
fn eight_bit_double_channel_into_multiple_chunks() {
    let data = vec![
        0xAA, 0x01,
        0xBB, 0x02,
        0xCC, 0x03,
        0xDD, 0x04
    ];
    let png = Png::new(1, 4, GrayscaleAlpha, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0xBB, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0xDD, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn eight_bit_double_channel_into_multiple_chunks_with_remainder() {
    let data = vec![
        0xAA, 0x01,
        0xBB, 0x02,
        0xCC, 0x03,
        0xDD, 0x04,
        0xEE, 0x05
    ];
    let png = Png::new(1, 5, GrayscaleAlpha, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 3);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0xBB, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0xDD, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
    assert_eq!(png_chunks[2].data, [0xEE, 0x05]);
    assert_eq!(png_chunks[2].len, 1);
}

#[rstest]
fn eight_bit_triple_channel_into_single_chunk() {
    let data = vec![
        0xAA, 0x01, 0x01,
        0xBB, 0x02, 0x02
    ];
    let png = Png::new(1, 2, Rgb, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0xBB, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
}

#[rstest]
fn eight_bit_triple_channel_into_multiple_chunks() {
    let data = vec![
        0xAA, 0x01, 0x01,
        0xBB, 0x02, 0x02,
        0xCC, 0x03, 0x03,
        0xDD, 0x04, 0x04
    ];
    let png = Png::new(1, 4, Rgb, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0xBB, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0x03, 0xDD, 0x04, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn eight_bit_triple_channel_into_multiple_chunks_with_remainder() {
    let data = vec![
        0xAA, 0x01, 0x01,
        0xBB, 0x02, 0x02,
        0xCC, 0x03, 0x03,
        0xDD, 0x04, 0x04,
        0xEE, 0x05, 0x05
    ];
    let png = Png::new(1, 5, Rgb, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 3);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0xBB, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0x03, 0xDD, 0x04, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
    assert_eq!(png_chunks[2].data, [0xEE, 0x05, 0x05]);
    assert_eq!(png_chunks[2].len, 1);
}

#[rstest]
fn eight_bit_quadruple_channel_into_single_chunk() {
    let data = vec![
        0xAA, 0x01, 0x01, 0x01,
        0xBB, 0x02, 0x02, 0x02
    ];
    let png = Png::new(1, 2, Rgba, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 1);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0x01, 0xBB, 0x02, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
}

#[rstest]
fn eight_bit_quadruple_channel_into_multiple_chunks() {
    let data = vec![
        0xAA, 0x01, 0x01, 0x01,
        0xBB, 0x02, 0x02, 0x02,
        0xCC, 0x03, 0x03, 0x03,
        0xDD, 0x04, 0x04, 0x04
    ];
    let png = Png::new(1, 4, Rgba, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 2);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0x01, 0xBB, 0x02, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0x03, 0x03, 0xDD, 0x04, 0x04, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
}

#[rstest]
fn eight_bit_quadruple_channel_into_multiple_chunks_with_remainder() {
    let data = vec![
        0xAA, 0x01, 0x01, 0x01,
        0xBB, 0x02, 0x02, 0x02,
        0xCC, 0x03, 0x03, 0x03,
        0xDD, 0x04, 0x04, 0x04,
        0xEE, 0x05, 0x05, 0x05
    ];
    let png = Png::new(1, 5, Rgba, Eight, data, None).unwrap();
    let png_chunks = png.chunks(2);

    assert_eq!(png_chunks.len(), 3);
    assert_eq!(png_chunks[0].data, [0xAA, 0x01, 0x01, 0x01, 0xBB, 0x02, 0x02, 0x02]);
    assert_eq!(png_chunks[0].len, 2);
    assert_eq!(png_chunks[1].data, [0xCC, 0x03, 0x03, 0x03, 0xDD, 0x04, 0x04, 0x04]);
    assert_eq!(png_chunks[1].len, 2);
    assert_eq!(png_chunks[2].data, [0xEE, 0x05, 0x05, 0x05]);
    assert_eq!(png_chunks[2].len, 1);
}

// TODO: add tests for 16-bit chunks
// TODO: add tests for RGB and RGBA data conversion of all 15 PNG formats
