use celeste_converter::file;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Celeste converter v{}", env!("CARGO_PKG_VERSION"));
        println!();
        println!("Usage:");
        println!("    celeste-converter [COMMAND] [INPUT] [OUTPUT]");
        println!("Commands:");
        println!("    data2png    Convert from Celeste DATA format into PNG");
        println!("    png2data    Convert from PNG into Celeste DATA format");
        return;
    }

    let command = args[1].as_str();
    let from = Path::new(&args[2]);
    let to = Path::new(&args[2]);

    match command {
        "data2png" => file::data_to_png(from, to),
        "png2data" => file::png_to_data(from, to),
        _ => panic!("Unknown command {command}"),
    }
    .expect("Failed to execute");
}
