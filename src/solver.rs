
pub fn solve(grid: &mut Vec<Vec<u8>>) -> bool {

    if let Some((x, y)) = find_empty(grid) { // If found something
        for i in 1..10 { // Test every number (1 to 9)
            let (valid, _) = is_valid(grid, x, y, i);
            if valid {
                grid[y][x] = i;

                if solve(grid) { // If this move is correct return true
                    return true;
                }

                grid[y][x] = 0; // Else try the next number
            }
        };
        false // No number is valid
    } else {
        true // Solved
    }
}

pub fn is_valid(grid: &[Vec<u8>], x: usize, y: usize, n: u8) -> (bool, String) {
    // Check row + column
    for i in 0..9 {
        // Check if the ith element of the row and column is equals to n
        if grid[y][i] == n {
            return (false, format!("Repeated number at row {}, column {}", y + 1, i + 1));
        } else if grid[i][x] == n {
            return (false, format!("Repeated number at row {}, column {}", i + 1, x + 1));
        }
    }

    // Clamp value to between 0 and 2 and multiply it with three
    // Then we will get a subgrid
    let by = 3*(y/3);
    let bx = 3*(x/3);

    // Check subgrid
    for i in 0..3 {
        for j in 0..3 {
            if grid[by + i][bx + j] == n {
                return (false, format!("Repeated number at row {}, column {}", by + i + 1, bx + j + 1));
            }
        }
    }

    (true, String::new()) // If everything is ok return true
}

fn find_empty(grid: &[Vec<u8>]) -> Option<(usize, usize)> {
    // Loop through each cell and find the first 0
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &0 {
                return Some((x, y));
            }
        }
    }
    None // The grid is solved!
}