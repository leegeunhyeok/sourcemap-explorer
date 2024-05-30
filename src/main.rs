use std::fs::File;

use clap::Parser;

use crate::utils::parse_position;

/// Sourcemap explorer
#[derive(Parser, Debug)]
#[command(version = "1.0.0")]
struct Args {
    /// Sourcemap file path
    #[arg()]
    sourcemap: String,

    /// Position of the source code (eg. 1:549)
    #[arg()]
    position: String,

    /// Print the original source contents
    #[arg(long)]
    contents: bool,
}

fn main() {
    let args = Args::parse();

    let file = match File::open(args.sourcemap) {
        Ok(file) => file,
        Err(e) => panic!("cannot read sourcemap file: {}", e.to_string()),
    };

    let (line, col) = match parse_position(&args.position) {
        Ok(position) => position,
        Err(e) => panic!("cannot parse given position value: {}", e.to_string()),
    };

    let sm: sourcemap::SourceMap = sourcemap::SourceMap::new(file);

    sm.lookup(line, col, args.contents);
}

mod sourcemap;
mod utils;
