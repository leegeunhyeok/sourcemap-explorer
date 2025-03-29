use clap::Parser;
use types::{Position, RuntimeType};
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

    /// Type of runtime
    #[arg(long, default_value_t = RuntimeType::Default)]
    r#type: RuntimeType,

    /// Print the original source content
    #[arg(long)]
    content: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let contents = read_file(args.sourcemap)?;
    let position = Position::try_from(&args.position)?;
    let sm: sourcemap::SourceMap = sourcemap::SourceMap::new(contents, args.r#type);

    sm.lookup(position, args.content)?;

    Ok(())
}

mod sourcemap;
mod types;
mod utils;
