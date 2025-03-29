use oxc_sourcemap::SourceMap as RawSourceMap;

use crate::{types::Mark, utils::print_src};

pub struct SourceMap {
    sm: RawSourceMap,
}

impl SourceMap {
    pub fn new(contents: String) -> Self {
        SourceMap {
            sm: RawSourceMap::from_json_string(&contents).unwrap(),
        }
    }

    pub fn lookup(&self, line: u32, col: u32, print_content: bool) {
        let mut token_size = 0;
        let lookup_table = self.sm.generate_lookup_table();
        let base_token = self.sm.lookup_source_view_token(
            &lookup_table,
            line - 1, /* Zero based index */
            col,
        );

        if base_token.is_none() {
            println!("Lookup failed ({}:{})", line, col);
            return;
        }

        let base_token = base_token.expect("unexpected error");
        let orig_line = base_token.get_src_line();
        let orig_col = base_token.get_src_col();

        for token in self.sm.get_tokens() {
            let dst_col = token.get_dst_col();

            if dst_col > col {
                token_size = token.get_src_col().abs_diff(orig_col);
                break;
            }
        }

        if print_content {
            match base_token.get_source_content() {
                Some(contents) => print_src(
                    contents,
                    Mark {
                        line: orig_line,
                        col: orig_col,
                        len: token_size,
                    },
                ),
                _ => println!("source content not found"),
            }
            println!("");
        }

        println!(
            "File - {}",
            base_token.get_source().unwrap_or("<anonymous>"),
        );
        println!(
            "Position - {}:{}:{}",
            base_token.get_name().unwrap_or("<anonymous>"),
            orig_line + 1, // Add 1 because line is zero based index
            orig_col
        );
    }
}
