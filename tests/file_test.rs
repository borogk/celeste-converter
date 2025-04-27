use celeste_converter::file::convert;
use rand::random;
use rstest::rstest;
use std::env::temp_dir;
use std::fs::{create_dir_all, read_dir, File};
use std::path::PathBuf;

#[rstest]
fn convert_file_to_non_existing_file() {
    let dir = create_empty_dir();
    let input = create_empty_file(dir.join("input.from"));
    let output = dir.join("output.to");

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.is_file())
}

#[rstest]
fn convert_file_to_existing_file() {
    let dir = create_empty_dir();
    let input = create_empty_file(dir.join("input.from"));
    let output = create_empty_file(dir.join("output.to"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.is_file())
}

#[rstest]
fn convert_file_to_the_same_dir() {
    let dir = create_empty_dir();
    let input = create_empty_file(dir.join("input.from"));

    convert(&input, None, "from", "to", |_, _| Ok(())).unwrap();

    assert!(dir.join("input.to").is_file())
}

#[rstest]
fn convert_non_existing_file_to_non_existing_file() {
    let dir = create_empty_dir();
    let input = dir.join("input.from");
    let output = dir.join("output.to");

    let err = convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap_err();

    assert!(err.to_string().contains("Input path can't be recognized as either file or directory"));
}

#[rstest]
fn convert_non_existing_file_to_existing_file() {
    let dir = create_empty_dir();
    let input = dir.join("input.from");
    let output = create_empty_file(dir.join("output.to"));

    let err = convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap_err();

    assert!(err.to_string().contains("Input path can't be recognized as either file or directory"));
}

#[rstest]
fn convert_non_existing_file_to_the_same_dir() {
    let dir = create_empty_dir();
    let input = dir.join("input.from");

    let err = convert(&input, None, "from", "to", |_, _| Ok(())).unwrap_err();

    assert!(err.to_string().contains("Input path can't be recognized as either file or directory"));
}

#[rstest]
fn convert_dir_to_unspecified_path() {
    let input = create_empty_dir();

    let err = convert(&input, None, "from", "to", |_, _| Ok(())).unwrap_err();

    assert!(err.to_string().contains("Output path must be specified"));
}

#[rstest]
fn convert_dir_to_existing_file() {
    let input = create_empty_dir();
    let output = create_empty_file(create_empty_dir().join("output.to"));

    let err = convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap_err();

    assert!(err.to_string().contains("Output path exists, but isn't a directory"));
}

#[rstest]
fn convert_dir_to_non_existing_dir() {
    let input = create_empty_dir();
    let output = create_empty_dir().join("not_exist");
    create_empty_file(input.join("1.from"));
    create_empty_file(input.join("2.from"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.join("1.to").is_file());
    assert!(output.join("2.to").is_file());
}

#[rstest]
fn convert_dir_to_existing_dir() {
    let input = create_empty_dir();
    let output = create_empty_dir();
    create_empty_file(input.join("1.from"));
    create_empty_file(input.join("2.from"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.join("1.to").is_file());
    assert!(output.join("2.to").is_file());
}

#[rstest]
fn convert_empty_dir() {
    let input = create_empty_dir();
    let output = create_empty_dir();

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.exists());
    assert_eq!(read_dir(output).unwrap().count(), 0);
}

#[rstest]
fn convert_dir_with_all_wrong_extensions() {
    let input = create_empty_dir();
    let output = create_empty_dir();
    create_empty_file(input.join("1.wrong"));
    create_empty_file(input.join("2.wrong"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.exists());
    assert_eq!(read_dir(output).unwrap().count(), 0);
}

#[rstest]
fn convert_dir_with_some_wrong_extensions() {
    let input = create_empty_dir();
    let output = create_empty_dir();
    create_empty_file(input.join("1.wrong"));
    create_empty_file(input.join("2.from"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.exists());
    assert!(!output.join("1.wrong").is_file());
    assert!(output.join("2.to").is_file());
}

#[rstest]
fn convert_dir_with_deep_hierarchy() {
    let input = create_empty_dir();
    let output = create_empty_dir();
    create_empty_file(input.join("1.from"));
    create_empty_file(input.join("2.from"));
    create_empty_file(input.join("a/3.from"));
    create_empty_file(input.join("a/4.from"));
    create_empty_file(input.join("b/5.from"));
    create_empty_file(input.join("b/6.from"));
    create_empty_file(input.join("a/c/7.from"));
    create_empty_file(input.join("a/d/8.from"));

    convert(&input, Some(&output), "from", "to", |_, _| Ok(())).unwrap();

    assert!(output.join("1.to").is_file());
    assert!(output.join("2.to").is_file());
    assert!(output.join("a/3.to").is_file());
    assert!(output.join("a/4.to").is_file());
    assert!(output.join("b/5.to").is_file());
    assert!(output.join("b/6.to").is_file());
    assert!(output.join("a/c/7.to").is_file());
    assert!(output.join("a/d/8.to").is_file());
}

fn create_empty_dir() -> PathBuf {
    let path = temp_dir().join(random::<u64>().to_string());
    create_dir_all(&path).unwrap();
    path
}

fn create_empty_file(path: PathBuf) -> PathBuf {
    create_dir_all(&path.parent().unwrap()).unwrap();
    File::create(&path).unwrap();
    path
}
