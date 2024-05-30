use std::io::{self, Write};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::types::Mark;

const INVALID_POSITION_ERR_MSG: &'static str = "invalid position";

pub type Position = (u32, u32);

pub fn parse_position(position: &String) -> Result<Position, &'static str> {
    let vec = position
        .split(":")
        .map(|s| s.parse::<u32>().map_err(|_| INVALID_POSITION_ERR_MSG))
        .collect::<Result<Vec<u32>, &'static str>>()?;

    if vec.len() == 2 {
        // Convert to zero based index
        Ok((vec[0] - 1, vec[1]))
    } else {
        Err(INVALID_POSITION_ERR_MSG)
    }
}

pub fn print_src(src: &str, mark: Mark) {
    let lines = src.lines();
    let max_line_num_width = lines.clone().count().to_string().len() + 1; // Additional space
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for (idx, line) in lines.enumerate() {
        print_line_num(&mut stdout, idx + 1, max_line_num_width);
        println!("{}", line);

        if idx == mark.line as usize {
            print_mark(
                &mut stdout,
                mark.col + max_line_num_width as u32,
                mark.len.unwrap_or(1),
            );
        }
    }
}

pub fn print_line_num(stdout: &mut StandardStream, num: usize, max_width: usize) {
    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(8)))) {
        // Grey
        Ok(_) => {
            let _ = write!(
                stdout,
                "{}{}",
                num,
                " ".repeat(max_width - num.to_string().len())
            );
        }
        Err(_) => {}
    }
    let _ = stdout.reset();
}

pub fn print_mark(stdout: &mut StandardStream, from: u32, len: u32) {
    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))) {
        Ok(_) => {
            let _ = writeln!(
                stdout,
                "{}",
                format!("{}{}", " ".repeat(from as usize), "^".repeat(len as usize))
            );
        }
        Err(_) => {}
    }
    let _ = stdout.reset();
}
