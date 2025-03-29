use clap::Parser;
use types::Position;
use utils::read_file;

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

fn main() -> Result<(), String> {
    let args = Args::parse();

    let contents = read_file(args.sourcemap)?;
    let position = Position::try_from(&args.position)?;
    let sm: sourcemap::SourceMap = sourcemap::SourceMap::new(contents);

    sm.lookup(position, args.content);

    Ok(())
}

mod sourcemap;
mod types;
mod utils;
