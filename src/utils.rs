use std::vec::Vec;

const INVALID_POSITION_ERR_MSG: &'static str = "invalid position";

pub type Position = (u32, u32);

pub fn parse_position(position: &String) -> Result<Position, &'static str> {
    let vec = position
        .split(":")
        .map(|s| s.parse::<u32>().map_err(|_| INVALID_POSITION_ERR_MSG))
        .collect::<Result<Vec<u32>, &'static str>>()?;

    if vec.len() == 2 {
        // Convert to zero based index
        Ok((vec[0] - 1, vec[1] - 1))
    } else {
        Err(INVALID_POSITION_ERR_MSG)
    }
}
