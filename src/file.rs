use crate::convert;
use anyhow::{bail, Result};
use pathdiff::diff_paths;
use same_file::is_same_file;
use std::fs::{create_dir_all, read_dir, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

pub fn data_to_png(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    convert(&input, output.as_ref(), "data", "png", convert::data_to_png)
}

pub fn png_to_data(input: PathBuf, output: Option<PathBuf>) -> Result<()> {
    convert(&input, output.as_ref(), "png", "data", convert::png_to_data)
}

pub fn convert<F: Fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<()>>(
    input: &PathBuf,
    output: Option<&PathBuf>,
    input_ext: &str,
    output_ext: &str,
    convert_fn: F,
) -> Result<()> {
    if input.is_file() {
        if output.is_none() {
            let input_dir = input.parent().unwrap().to_path_buf();
            convert_file_to_dir(input, &input_dir, output_ext, convert_fn)
        } else {
            convert_file_to_file(input, output.unwrap(), convert_fn)
        }
    } else if input.is_dir() {
        println!("Input path is a directory: {}", input.display());

        let output = match output {
            None => bail!("Output path must be specified"),
            Some(o) => o,
        };

        if output.exists() && !output.is_dir() {
            bail!("Output path exists, but isn't a directory: {}", output.display());
        }

        convert_dir_to_dir(input, output, input_ext, output_ext, convert_fn)
    } else {
        bail!("Input path can't be recognized as either file or directory: {}", input.display());
    }
}

fn convert_file_to_file<F: Fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<()>>(
    input: &PathBuf,
    output: &PathBuf,
    convert_fn: F,
) -> Result<()> {
    println!("Input file: {}", input.display());
    println!("Output file: {}", output.display());

    if output.exists() && is_same_file(&input, &output)? {
        bail!("Input and output paths point to the same file");
    }

    let output_dir = output.parent().unwrap();
    if !output_dir.exists() {
        println!("Ensuring output directory exists {}", output_dir.display());
        match create_dir_all(output_dir) {
            Ok(_) => (),
            Err(e) => bail!("Failed to create output directory {}: {}", output_dir.display(), e),
        }
    }

    let mut input_reader = match File::open(&input) {
        Ok(f) => BufReader::new(f),
        Err(e) => bail!("Failed to open input file {}: {}", input.display(), e),
    };

    let mut output_writer = match File::create(&output) {
        Ok(f) => BufWriter::new(f),
        Err(e) => bail!("Failed to create output file {}: {}", output.display(), e),
    };

    convert_fn(&mut input_reader, &mut output_writer)?;

    Ok(())
}

fn convert_file_to_dir<F: Fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<()>>(
    input: &PathBuf,
    output: &PathBuf,
    output_ext: &str,
    convert_fn: F,
) -> Result<()> {
    let file_name = input.file_stem().unwrap().to_str().unwrap();
    let output_file_path = output.join(file_name).with_extension(output_ext);

    convert_file_to_file(input, &output_file_path, convert_fn)
}

fn convert_dir_to_dir<F: Fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<()>>(
    input: &PathBuf,
    output: &PathBuf,
    input_ext: &str,
    output_ext: &str,
    convert_fn: F,
) -> Result<()> {
    let mut items: Vec<PathBuf> = Vec::new();
    scan_dir(&input, input_ext, 0, &mut items)?;

    println!("Found {} input files", items.len());
    let mut success = 0;
    for (i, item_input_path) in items.iter().enumerate() {
        println!("\n[{}/{}] Converting...", i + 1, items.len());

        let file_name = item_input_path.file_stem().unwrap().to_str().unwrap();
        let relative_file_path = diff_paths(item_input_path, &input).unwrap();
        let relative_dir_path = relative_file_path.parent().unwrap();
        let item_output_path = output.join(relative_dir_path).join(file_name).with_extension(output_ext);

        match convert_file_to_file(item_input_path, &item_output_path, &convert_fn) {
            Ok(_) => success += 1,
            Err(e) => eprintln!("Error converting: {}", e),
        }
    }

    if items.len() > 0 {
        println!("\n{}/{} converted successfully", success, items.len());
    }

    Ok(())
}

fn scan_dir(path: &PathBuf, ext: &str, depth: u8, result: &mut Vec<PathBuf>) -> Result<()> {
    const MAX_DEPTH: u8 = 16;
    if depth > MAX_DEPTH {
        return Ok(());
    }

    for entry in read_dir(path)? {
        let entry = entry?;
        let child_path = entry.path();
        let child_file_type = entry.file_type()?;

        if child_file_type.is_dir() {
            scan_dir(&child_path, ext, depth + 1, result)?;
        } else if child_path.extension().unwrap().eq_ignore_ascii_case(ext) {
            result.push(child_path);
        }
    }

    Ok(())
}
