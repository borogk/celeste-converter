use celeste_converter::convert;
use image::{DynamicImage, GenericImageView};
use image::ImageFormat;
use rstest::rstest;
use rstest_reuse::apply;
use rstest_reuse::template;
use std::fs::File;
use std::io::Read;
use std::io::{Cursor, Seek};

#[template]
#[rstest]
#[case::white("white", false)]
#[case::red("red", false)]
#[case::green("green", false)]
#[case::blue("blue", false)]
#[case::cyan("cyan", false)]
#[case::magenta("magenta", false)]
#[case::yellow("yellow", false)]
#[case::black("black", false)]
#[case::transparent("transparent", false)]
#[case::multi_color("multi-color", false)]
#[case::big_test("big-test", false)]
#[case::big_test_no_background("big-test-no-background", false)]
#[case::ffmpeg_rgb24("ffmpeg/rgb24", false)]
#[case::ffmpeg_rgba("ffmpeg/rgba", false)]
#[case::ffmpeg_rgb48be("ffmpeg/rgb48be", true)]
#[case::ffmpeg_rgba64be("ffmpeg/rgba64be", true)]
#[case::ffmpeg_pal8("ffmpeg/pal8", false)]
#[case::ffmpeg_gray("ffmpeg/gray", false)]
#[case::ffmpeg_ya8("ffmpeg/ya8", false)]
#[case::ffmpeg_gray16be("ffmpeg/gray16be", true)]
#[case::ffmpeg_ya16be("ffmpeg/ya16be", true)]
#[case::ffmpeg_monob("ffmpeg/monob", false)]
#[case::ffmpeg_monob_prime_dimensions("ffmpeg/monob-prime-dimensions", false)]
fn all_image_cases(#[case] case: &str, #[case] sixteen_bit: bool) {}

#[apply(all_image_cases)]
fn data_to_png_matches_original(#[case] case: &str, #[case] sixteen_bit: bool) {
    let original_data_bytes = load_data_bytes(case);

    let converted_png = data_bytes_to_png_image(&original_data_bytes);

    let original_png_image = load_png_image(case);
    assert_png_image_eq(&converted_png, &original_png_image, sixteen_bit);
}

#[apply(all_image_cases)]
fn data_to_png_twice_matches_original(#[case] case: &str, #[case] sixteen_bit: bool) {
    let original_data_bytes = load_data_bytes(case);

    let converted_png_image = data_bytes_to_png_image(&original_data_bytes);
    let converted_data_bytes = png_image_to_data_bytes(&converted_png_image);
    let twice_converted_png_image = data_bytes_to_png_image(&converted_data_bytes);

    assert_png_image_eq(&twice_converted_png_image, &converted_png_image, sixteen_bit);
}

#[apply(all_image_cases)]
fn png_to_data_and_back_matches_original(#[case] case: &str, #[case] sixteen_bit: bool) {
    let original_png_bytes = load_png_bytes(case);

    let converted_data_bytes = png_bytes_to_data_bytes(&original_png_bytes);
    let converted_png_image = data_bytes_to_png_image(&converted_data_bytes);

    let original_png_image = load_png_image(case);
    assert_png_image_eq(&converted_png_image, &original_png_image, sixteen_bit);
}

fn load_png_image(image: &str) -> DynamicImage {
    let path = format!("tests/png/{image}.png");
    image::ImageReader::open(path).unwrap().decode().unwrap()
}

fn load_png_bytes(image: &str) -> Vec<u8> {
    let path = format!("tests/png/{image}.png");
    let mut file = File::open(path).unwrap();
    let mut data = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data).unwrap();
    data
}

fn load_data_bytes(image: &str) -> Vec<u8> {
    let path = format!("tests/data/{image}.data");
    let mut file = File::open(path).unwrap();
    let mut data = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data).unwrap();
    data
}

fn data_bytes_to_png_image(data: &Vec<u8>) -> DynamicImage {
    let mut input = Cursor::new(data);
    let mut output = Cursor::new(Vec::new());

    convert::data_to_png(&mut input, &mut output).expect("Couldn't convert PNG to DATA");
    output.rewind().unwrap();

    image::ImageReader::with_format(output, ImageFormat::Png).decode().unwrap()
}

fn png_image_to_data_bytes(png: &DynamicImage) -> Vec<u8> {
    let mut input = Cursor::new(Vec::new());
    png.write_to(&mut input, ImageFormat::Png).unwrap();
    input.rewind().unwrap();

    let mut output = Vec::new();
    convert::png_to_data(&mut input, &mut output).expect("Couldn't convert DATA to PNG");
    output
}

fn png_bytes_to_data_bytes(png: &Vec<u8>) -> Vec<u8> {
    let mut input = Cursor::new(png);
    let mut output = Vec::new();
    convert::png_to_data(&mut input, &mut output).expect("Couldn't convert DATA to PNG");
    output
}

fn assert_png_image_eq(actual: &DynamicImage, expected: &DynamicImage, sixteen_bit: bool) {
    assert_eq!(actual.width(), expected.width(), "Images have different widths");
    assert_eq!(actual.height(), expected.height(), "Images have different heights");

    assert_eq!(actual.color().has_alpha(), expected.color().has_alpha(), "Images have different alpha");

    for x in 0..actual.width() {
        for y in 0..actual.height() {
            let image::Rgba(actual_pixel) = actual.get_pixel(x, y);
            let image::Rgba(expected_pixel) = expected.get_pixel(x, y);
            if !sixteen_bit {
                assert_eq!(actual_pixel, expected_pixel, "Images differ at X={}, Y={}", x, y);
            } else {
                // For 16-bit images a small tolerance is allowed, accounting for difference
                // in how graphics editors deal with precision loss between 16 and 8 bit
                for i in 0..4 {
                    let diff = actual_pixel[i].abs_diff(expected_pixel[i]);
                    assert!(diff <= 1, "Images differ at X={}, Y={}, channel={}", x, y, i);
                }
            }
        }
    }
}
