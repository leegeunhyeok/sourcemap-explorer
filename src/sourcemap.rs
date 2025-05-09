use oxc_sourcemap::SourceMap as RawSourceMap;

use crate::{
    types::{LookupResult, Mark, Position},
    utils::to_valid_sm,
};

pub struct SourceMap {
    sm: RawSourceMap,
}

impl SourceMap {
    pub fn new(contents: String) -> Self {
        // Hermes bytecode sourcemap's `names` field can be optional,
        // so we need to convert it to a valid sourcemap.
        let contents = to_valid_sm(contents);

        SourceMap {
            sm: RawSourceMap::from_json_string(&contents).unwrap(),
        }
    }

    pub fn lookup(&self, position: Position) -> Result<LookupResult, String> {
        let mut token_size = 0;
        let lookup_table = self.sm.generate_lookup_table();
        let base_token = self.sm.lookup_source_view_token(
            &lookup_table,
            position.get_line(),
            position.get_col(),
        );

        if base_token.is_none() {
            return Err(format!("Lookup failed ({})", position));
        }

        let base_token = base_token.expect("unexpected error");
        let orig_line = base_token.get_src_line();
        let orig_col = base_token.get_src_col();

        for token in self.sm.get_tokens() {
            let dst_col = token.get_dst_col();

            if dst_col > position.1 {
                token_size = token.get_src_col().abs_diff(orig_col);
                break;
            }
        }

        let content = match base_token.get_source_content() {
            Some(content) => Some((
                content.to_string(),
                Mark {
                    line: orig_line,
                    col: orig_col,
                    len: token_size,
                },
            )),
            _ => None,
        };

        // println!(
        //     "File - {}",
        //     base_token.get_source().unwrap_or("<anonymous>"),
        // );
        // println!(
        //     "Position - {}:{}:{}",
        //     base_token.get_name().unwrap_or("<anonymous>"),
        //     orig_line + 1, // Add 1 because line is zero based index
        //     orig_col
        // );

        Ok(LookupResult {
            position: Position(orig_line, orig_col),
            source: base_token
                .get_source()
                .and_then(|name| Some(name.to_string())),
            name: base_token
                .get_name()
                .and_then(|name| Some(name.to_string())),
            content,
        })
    }
}
