use clap::{Parser, Subcommand};

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

            for row in &grid {
                println!("{:?}", row);
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
        #[clap(short='o')]
        output: Option<String>
    }
}