use std::{fs, io};

pub fn read_from(file_name: String) -> Result<Vec<Vec<u8>>, io::Error> {
    let raw_lines: String = fs::read_to_string(file_name)?.trim_end_matches("\r\n").to_owned();

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in raw_lines.lines() {
        let mut row: Vec<u8> = Vec::new();
        
        for c in line.chars() {
            if c.is_ascii_digit() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }

        grid.push(row);
    };

    for (i, row) in grid.iter_mut().enumerate() {
        if row.len() != 9 {
            return make_error(format!("Expected 9 digits at row {}, found {}", i+1, row.len()));
        }
    }

    if grid.len() == 9 {
        Ok(grid)
    } else {
        make_error(format!("Expected 9 rows, found {}", grid.len()))
    }
}

fn make_error(msg: String) -> Result<Vec<Vec<u8>>, io::Error> {
    Err(io::Error::new(io::ErrorKind::InvalidData, msg))
}