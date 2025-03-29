use std::fs::File;

use clap::Parser;
use utils::read_file;

use crate::utils::parse_position;

/// Sourcemap explorer
#[derive(Parser, Debug)]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct Args {
    /// Sourcemap file path
    #[arg()]
    sourcemap: String,

    /// Position of the source code (eg. 1:549)
    #[arg()]
    position: String,

    /// Print the original source content
    #[arg(long)]
    content: bool,
}

fn main() -> Result<(), &'static str> {
    let args = Args::parse();
    let contents = read_file(args.sourcemap)?;

    let (line, col) = match parse_position(&args.position) {
        Ok(position) => position,
        Err(e) => panic!("cannot parse given position value: {}", e.to_string()),
    };

    let sm: sourcemap::SourceMap = sourcemap::SourceMap::new(contents);

    sm.lookup(line, col, args.content);
    Ok(())
}

mod sourcemap;
mod types;
mod utils;
