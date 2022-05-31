```sh
sudoku 

USAGE:
    sudoku.exe <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help     Print this message or the help of the given subcommand(s)
    solve
```

## Examples:
```sh
sudoku solve sudoku.txt
# solves the sudoku in sudoku.txt and output the solution in the same file
```
```sh
sudoku solve sudoku.txt --output "solution.txt"
# or
sudoku solve sudoku.txt -o "solution.txt"
# Output solution at specific file
```