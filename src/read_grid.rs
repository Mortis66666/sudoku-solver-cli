use std::{fs, io};

pub fn read_from(file_name: String) -> io::Result<Vec<Vec<u8>>> {
    let input = fs::read_to_string(file_name)?;

    read(input.trim_end_matches("\r\n"))
}

fn read(input: &str) -> Result<Vec<Vec<u8>>, io::Error> {
    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in input.lines() {
        let mut row: Vec<u8> = Vec::new();

        for c in line.chars() {
            if c.is_ascii_digit() {
                row.push(c.to_digit(10).unwrap() as u8);
            }
        }

        grid.push(row);
    }

    for (i, row) in grid.iter_mut().enumerate() {
        if row.len() != 9 {
            return make_error(format!(
                "Expected 9 digits at row {}, found {}",
                i + 1,
                row.len()
            ));
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

#[cfg(test)]
mod tests {
    #[test]
    fn read_from() {
        let input = r#"120070560
        507932080
        000001000
        010240050
        308000402
        070085010
        000700000
        080423701
        034010028"#;

        let expected = vec![
            vec![1, 2, 0, 0, 7, 0, 5, 6, 0],
            vec![5, 0, 7, 9, 3, 2, 0, 8, 0],
            vec![0, 0, 0, 0, 0, 1, 0, 0, 0],
            vec![0, 1, 0, 2, 4, 0, 0, 5, 0],
            vec![3, 0, 8, 0, 0, 0, 4, 0, 2],
            vec![0, 7, 0, 0, 8, 5, 0, 1, 0],
            vec![0, 0, 0, 7, 0, 0, 0, 0, 0],
            vec![0, 8, 0, 4, 2, 3, 7, 0, 1],
            vec![0, 3, 4, 0, 1, 0, 0, 2, 8],
        ];

        let output = super::read(input).unwrap();

        assert_eq!(output, expected)
    }
}
