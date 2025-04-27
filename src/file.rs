use crate::convert;
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn data_to_png(from: &Path, to: &Path) -> Result<()> {
    convert(from, to, convert::data_to_png)
}

pub fn png_to_data(from: &Path, to: &Path) -> Result<()> {
    convert(from, to, convert::png_to_data)
}

fn convert<F: Fn(&mut BufReader<File>, &mut BufWriter<File>) -> Result<()>>(
    from: &Path,
    to: &Path,
    f: F,
) -> Result<()> {
    let from_file = File::open(from)?;
    let to_file = File::create(to)?;

    let mut input = BufReader::new(from_file);
    let mut output = BufWriter::new(to_file);
    f(&mut input, &mut output)?;

    Ok(())
}
