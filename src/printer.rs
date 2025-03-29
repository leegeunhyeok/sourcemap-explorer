use crate::types::{LookupResult, Mark};
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

const ANONYMOUS: &str = "<anonymous>";

pub trait Printer {
    fn print(&self, with_content: bool);
}

pub struct TextPrinter {
    res: LookupResult,
}

impl TextPrinter {
    pub fn new(res: LookupResult) -> Self {
        Self { res }
    }

    fn print_line_num(&self, stdout: &mut StandardStream, num: usize, max_width: usize) {
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

    fn print_src(&self, src: &String, mark: &Mark) {
        let lines = src.lines();
        let max_line_num_width = lines.clone().count().to_string().len() + 1; // Additional space
        let mut stdout = StandardStream::stdout(ColorChoice::Always);

        for (idx, line) in lines.enumerate() {
            self.print_line_num(&mut stdout, idx + 1, max_line_num_width);
            println!("{}", line);

            if idx == mark.line as usize {
                self.print_mark(&mut stdout, mark.col + max_line_num_width as u32, mark.len);
            }
        }
    }

    fn print_mark(&self, stdout: &mut StandardStream, from: u32, len: u32) {
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
}

impl Printer for TextPrinter {
    fn print(&self, with_content: bool) {
        if with_content {
            match &self.res.content {
                Some((src, mark)) => self.print_src(&src, &mark),
                None => println!("No content found"),
            }
            print!("\n")
        }

        let source = self.res.source.as_deref().unwrap_or(ANONYMOUS);
        let name = self.res.name.as_deref().unwrap_or(ANONYMOUS);

        println!("File - {}", source);
        println!("Position - {}:{}", name, self.res.position);
    }
}

pub struct JsonPrinter {
    res: LookupResult,
}

impl JsonPrinter {
    pub fn new(res: LookupResult) -> Self {
        Self { res }
    }
}

impl Printer for JsonPrinter {
    fn print(&self, with_content: bool) {
        let source: Option<&str> = self.res.source.as_deref();
        let name = self.res.name.as_deref();
        let data = serde_json::json!({
            "line": self.res.position.0,
            "column": self.res.position.1,
            "source": source,
            "name": name,
            "content": if with_content {
                match &self.res.content {
                    Some((src, _)) => Some(src),
                    None => None,
                }
            } else {
                None
            },
        });

        println!(
            "{}",
            serde_json::to_string_pretty(&data).expect("failed to print json")
        );
    }
}
