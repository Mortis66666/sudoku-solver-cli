use clap::{Parser, Subcommand};
use std::fs;

mod solver;
mod read_grid;

fn main() {
    let cli = Args::parse();

    match cli.cmd {
        SubCommand::Solve {file_name, output} => {
            let output = output.unwrap_or(file_name.clone());
            let res = read_grid::read_from(file_name);

            if let Err(err) = res {
                if let Some(inner) = err.get_ref() {
                    return println!("{}", inner);
                } else {
                    return println!("{}", err);
                }
            }

            let mut grid = res.unwrap();

            if solver::solve(&mut grid) {
                let mut grid_str = String::new();

                for row in &grid {
                    for n in row {
                        grid_str.push_str(&n.to_string()[..]);
                    }
                    grid_str.push_str("\n");
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
    }
}