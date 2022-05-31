use clap::{Parser, Subcommand};
use std::fs;

mod solver;
mod read_grid;

fn main() {
    let cli = Args::parse();

    match cli.cmd {
        SubCommand::Solve {file_name, output} => {
            let output = match output {
                Some(o) => { o },
                None => { file_name.clone() }
            };

            let mut grid = read_grid::read_from(file_name).unwrap();
            solver::solve(&mut grid);
            let mut grid_str = String::new();

            for row in &grid {
                for n in row {
                    grid_str.push_str(&n.to_string()[..]);
                }
                grid_str.push_str("\n");
            }

            match fs::write(&output, grid_str) {
                Ok(_) => {
                    println!("Success full solved sudoku, output at {}", output);
                }
                Err(err) => {
                    eprintln!("Error:\n\t{}", err);
                }
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