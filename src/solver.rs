
pub fn solve(grid: &mut Vec<Vec<u8>>) -> bool {
    let (x, y) = find_empty(&grid);

    if x == 9 {
        true
    } else {
        for i in 1..10 as u8 {
            if is_valid(grid, x, y, i) {
                grid[y][x] = i;

                if solve(grid) {
                    return true;
                }
    
                grid[y][x] = 0;
            }
        };
        false
    }
}

fn is_valid(grid: &Vec<Vec<u8>>, x: usize, y: usize, n: u8) -> bool {
    for i in 0..9 as usize {
        if grid[y][i] == n || grid[i][x] == n {
            return false;
        }
    }
    
    for i in 0..3 {
        for j in 0..3 {
            if grid[3*(y/3)+i][3*(x/3)+j] == n {
                return false;
            }
        }
    }

    true
}

fn find_empty(grid: &Vec<Vec<u8>>) -> (usize, usize) {
    for (y, row) in grid.into_iter().enumerate() {
        for (x, cell) in row.into_iter().enumerate() {
            if cell == &0 {
                return (x as usize, y as usize);
            }
        }
    }
    (9 as usize, 9 as usize)
}