use std::fmt::{self, Display};

const INVALID_POSITION_ERR_MSG: &'static str = "invalid position";

pub struct Position(
    /// Line
    pub u32,
    /// Column
    pub u32,
);

impl Position {
    pub fn get_line(&self) -> u32 {
        self.0
    }

    pub fn get_col(&self) -> u32 {
        self.0
    }
}

impl TryFrom<&String> for Position {
    type Error = String;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let vec: Vec<u32> = value
            .split(":")
            .map(|s| s.parse::<u32>().map_err(|_| INVALID_POSITION_ERR_MSG))
            .collect::<Result<Vec<u32>, &'static str>>()?;

        if vec.len() == 2 {
            Ok(Position(vec[0] - 1 /* Zero based index */, vec[1]))
        } else {
            Err(format!("cannot parse given position value: {}", value))
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}",
            self.get_line() + 1, /* Restore from zero based index */
            self.get_col()
        )
    }
}

pub struct Mark {
    pub line: u32,
    pub col: u32,
    pub len: u32,
}
