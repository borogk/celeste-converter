use anyhow::anyhow;
use celeste_converter::file::{data_to_png, png_to_data};
use std::env;
use std::path::PathBuf;

fn main() {
    println!("Celeste converter v{}", env!("CARGO_PKG_VERSION"));
    println!();

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage:");
        println!("    celeste-converter [COMMAND] [INPUT] [OUTPUT]");
        println!("Commands:");
        println!("    data2png    Convert from Celeste DATA format into PNG");
        println!("    png2data    Convert from PNG into Celeste DATA format");
        return;
    }

    let command = args[1].as_str();
    let input = PathBuf::from(&args[2]);
    let output = if args.len() >= 4 { Some(PathBuf::from(&args[3])) } else { None };

    let command_result = match command {
        "data2png" => data_to_png(input, output),
        "png2data" => png_to_data(input, output),
        _ => Err(anyhow!("Unknown command {command}")),
    };

    if command_result.is_err() {
        eprintln!("Error: {}", command_result.unwrap_err());
    }
}
