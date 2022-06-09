use clap::{Parser, Subcommand};
use std::fs;
use crate::solver::is_valid;

mod read_grid;
mod solver;

fn main() {
    let cli = Args::parse();

    match cli.cmd {
        SubCommand::Solve { file_name, output } => {
            let output = output.unwrap_or_else(|| file_name.clone());
            let res = read_grid::read_from(file_name);

            if let Err(err) = res {
                return if let Some(inner) = err.get_ref() {
                    println!("{}", inner)
                } else {
                    println!("{}", err)
                }
            }

            let mut grid = res.unwrap();

            if solver::solve(&mut grid) {
                let mut grid_str = String::new();

                for row in &grid {
                    for n in row {
                        grid_str.push_str(&n.to_string()[..]);
                    }
                    grid_str.push('\n');
                }
    
                match fs::write(&output, grid_str) {
                    Ok(_) => {
                        println!("Successfully solved sudoku, output at {}", output);
                    }
                    Err(err) => {
                        eprintln!("Error:\n\t{}", err);
                    }
                }
            } else {
                println!("Can't solve grid, probably because the grid is invalid");
            }
        }
        SubCommand::Check { file_name } => {
            let res = read_grid::read_from(file_name);

            if let Err(err) = res {
                return if let Some(inner) = err.get_ref() {
                    println!("{}", inner)
                } else {
                    println!("{}", err)
                }
            }

            let mut grid = res.unwrap();

            for y in 0..9 {
                for x in 0..9 {
                    let n = grid[y][x];
                    if n != 0 {
                        grid[y][x] = 0;
                        let (res, message) = is_valid(&mut grid, x, y, n.to_owned());
                        if !res {
                            return println!("{}", message);
                        }
                        grid[y][x] = n.to_owned();
                    }
                }
            }

            println!("Grid is ok!")
        }
    }
}


#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    cmd: SubCommand
}

#[derive(Subcommand)]
enum SubCommand {
    Solve {
        file_name: String,
        #[clap(short='o', long="output")]
        output: Option<String>
    },
    Check {
        file_name: String,
    }
}