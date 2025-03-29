use clap::Parser;
use types::{Position, RuntimeType};
use utils::{print_src, read_file};

const ANONYMOUS: &str = "<anonymous>";

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

    if args.content {
        match res.content {
            Some((src, mark)) => print_src(&src, mark),
            None => println!("No content found"),
        }
        print!("\n");
    }

    println!("File - {}", res.source.unwrap_or(ANONYMOUS.into()),);
    println!(
        "Position - {}:{}",
        res.name.unwrap_or(ANONYMOUS.into()),
        res.position,
    );

    Ok(())
}

mod sourcemap;
mod types;
mod utils;
