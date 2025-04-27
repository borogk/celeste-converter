use celeste_converter::convert;
use image::DynamicImage;
use image::ImageFormat;
use rstest::rstest;
use rstest_reuse::apply;
use rstest_reuse::template;
use std::fs::File;
use std::io::Read;
use std::io::{Cursor, Seek};

#[template]
#[rstest]
#[case("white")]
#[case("red")]
#[case("green")]
#[case("blue")]
#[case("cyan")]
#[case("magenta")]
#[case("yellow")]
#[case("black")]
#[case("transparent")]
#[case("multi-color")]
#[case("big-test")]
#[case("big-test-no-background")]
fn all_image_cases(#[case] image: &str) {}

#[apply(all_image_cases)]
fn converted_png_equals_to_original(#[case] image: &str) {
    let original_data = load_data(image);
    let original_png = load_png(image);

    let converted_png = data_to_png(&original_data);

    assert_png_eq(&converted_png, &original_png);
}

#[apply(all_image_cases)]
fn png_survives_multiple_conversions(#[case] image: &str) {
    let original_data = load_data(image);

    let converted_png = data_to_png(&original_data);
    let converted_data = png_to_data(&converted_png);
    let twice_converted_png = data_to_png(&converted_data);

    assert_png_eq(&twice_converted_png, &converted_png);
}

fn load_png(image: &str) -> DynamicImage {
    let path = format!("tests/png/{image}.png");
    image::ImageReader::open(path).unwrap().decode().unwrap()
}

fn load_data(image: &str) -> Vec<u8> {
    let path = format!("tests/data/{image}.data");
    let mut file = File::open(path).unwrap();
    let mut data = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut data).unwrap();
    data
}

fn data_to_png(data: &Vec<u8>) -> DynamicImage {
    let mut input = Cursor::new(data);
    let mut output = Cursor::new(Vec::new());

    convert::data_to_png(&mut input, &mut output).expect("Couldn't convert PNG to DATA");
    output.rewind().unwrap();

    image::ImageReader::with_format(output, ImageFormat::Png)
        .decode()
        .unwrap()
}

fn png_to_data(png: &DynamicImage) -> Vec<u8> {
    let mut input = Cursor::new(Vec::new());
    png.write_to(&mut input, ImageFormat::Png).unwrap();
    input.rewind().unwrap();

    let mut output = Vec::new();
    convert::png_to_data(&mut input, &mut output).expect("Couldn't convert DATA to PNG");
    output
}

fn assert_png_eq(actual: &DynamicImage, expected: &DynamicImage) {
    assert_eq!(actual.width(), expected.width());
    assert_eq!(actual.height(), expected.height());

    assert_eq!(
        actual.color().channel_count(),
        expected.color().channel_count()
    );
    assert_eq!(actual.color().has_alpha(), expected.color().has_alpha());

    assert_eq!(actual.as_bytes(), expected.as_bytes());
}
