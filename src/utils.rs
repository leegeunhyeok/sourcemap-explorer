use std::{
    fs::File,
    io::{Read, Write},
};

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

use crate::types::Mark;

const READ_FILE_ERR_MSG: &'static str = "cannot read file contents";

pub fn read_file(path: String) -> Result<String, &'static str> {
    let mut file = File::open(path).map_err(|_| READ_FILE_ERR_MSG)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|_| READ_FILE_ERR_MSG)?;
    Ok(contents)
}

pub fn to_valid_sm(contents: String) -> String {
    let mut json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    if let Some(sm_obj) = json.as_object_mut() {
        if !sm_obj.contains_key("names") {
            sm_obj.insert("names".to_string(), serde_json::Value::Array(vec![]));
        }
    }

    json.to_string()
}

pub fn print_src(src: &str, mark: Mark) {
    let lines = src.lines();
    let max_line_num_width = lines.clone().count().to_string().len() + 1; // Additional space
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for (idx, line) in lines.enumerate() {
        print_line_num(&mut stdout, idx + 1, max_line_num_width);
        println!("{}", line);

        if idx == mark.line as usize {
            print_mark(&mut stdout, mark.col + max_line_num_width as u32, mark.len);
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
