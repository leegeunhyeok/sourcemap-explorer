use std::fs::File;

use sourcemap::SourceMap as RawSourceMap;

pub struct SourceMap {
    sm: RawSourceMap,
}

impl SourceMap {
    pub fn new(file: File) -> Self {
        SourceMap {
            sm: RawSourceMap::from_reader(file).unwrap(),
        }
    }

    pub fn lookup(&self, line: u32, col: u32, print_contents: bool) {
        match self.sm.lookup_token(line, col) {
            Some(token) => {
                if print_contents {
                    match token.get_source_view() {
                        Some(src_view) => println!("{}", src_view.source()),            
                        None => println!("Original source contents not found"),
                    }
                    println!("===========================");
                }

                println!("File - {}", token.get_source().unwrap_or("<anonymous>"),);
                println!(
                    "Position - {}:{}:{}",
                    token.get_name().unwrap_or("<anonymous>"),
                    token.get_src_line(),
                    token.get_src_col()
                );
            }
            None => println!("Lookup failed ({}:{})", line, col),
        }
    }
}
