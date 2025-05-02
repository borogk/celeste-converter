use anyhow::anyhow;
use celeste_converter::file::{data_to_png, png_to_data};
use celeste_converter::log;
use std::env;
use std::path::PathBuf;
use celeste_converter::rayon::init_rayon;

fn main() {
    init_rayon();
    
    log!("Celeste converter v{}\n", env!("CARGO_PKG_VERSION"));

    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        log!("Usage:");
        log!("    celeste-converter [COMMAND] [INPUT] [OUTPUT]");
        log!("Commands:");
        log!("    data2png    Convert from Celeste DATA format into PNG");
        log!("    png2data    Convert from PNG into Celeste DATA format");
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
        log!("Error: {}", command_result.unwrap_err());
    }
}
