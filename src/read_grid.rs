use std::{fs, io};

pub fn read_from(file_name: String) -> Result<Vec<Vec<u8>>, io::Error> {
    let raw_lines: String = fs::read_to_string(file_name)?;

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in raw_lines.split("\n") {
        let mut row: Vec<u8> = Vec::new();
        
        for c in line.chars() {
            if c.is_ascii_digit() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }

        grid.push(row);
    };

    if grid.len() == 9 && (&grid).into_iter().all(|r| r.len() == 9) {
        Ok(grid)
    } else {
        Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid grid"))
    }
}