use clap::Parser;
use printer::{JsonPrinter, Printer, TextPrinter};
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

    /// If you use Hermes runtime (--type hermes),
    /// you need to provide the sourcemap file path of the packager (eg. Metro).
    #[arg(long)]
    hermes_packager_sourcemap: Option<String>,

    /// Print the original source content
    #[arg(long)]
    content: bool,

    /// Print the result in JSON format
    #[arg(long)]
    json: bool,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let contents = read_file(args.sourcemap)?;
    let position = Position::try_from(&args.position)?;
    let sm: sourcemap::SourceMap = sourcemap::SourceMap::new(contents);

    let res = match args.r#type {
        RuntimeType::Default => sm.lookup(position)?,
        RuntimeType::Hermes => match args.hermes_packager_sourcemap {
            Some(hermes_packager_sourcemap) => {
                let pkg_contents = read_file(hermes_packager_sourcemap)?;
                let hbc_res = sm.lookup(position)?;
                let pkg_res = sourcemap::SourceMap::new(pkg_contents).lookup(hbc_res.position)?;

                pkg_res
            }
            None => return Err("packager sourcemap path is required".into()),
        },
    };

    let printer: Box<dyn Printer> = match args.json {
        true => Box::new(JsonPrinter::new(res)),
        false => Box::new(TextPrinter::new(res)),
    };

    printer.print(args.content);

    Ok(())
}

mod printer;
mod sourcemap;
mod types;
mod utils;
