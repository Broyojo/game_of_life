# Game of Life

![](https://github.com/Broyojo/game_of_life/blob/master/diagram.png)

## Description
This is an implementation of Conway's Game of Life in the Rust Programming Language. The program can either start out the grid in a random configuration of cells or a `.cells` file can be loaded which specifies the initial cell configuration. Each successive generation is printed to the console after the previous generation and the console is cleared between generations.

## How the `.cells` file format works
This program uses the `.cells` file format to load an initial cell configuration. In this file format, a dead cell is presented by `.` and an alive cell is represented by `O`. Note: The full grid does not need to be filled out with. Dead cells (`.`) are only used so that the positioning of alive cells (`O`) are correct. Empty space after alive cells are assumed to be dead cells. For reference look at [glider_gun.cells](https://github.com/Broyojo/game_of_life/blob/master/src/glider_gun.cells).

## How to Run

- Run with random initial cell configuration: `cargo run`
- Run with `.cells` configuration: `cargo run -- path/to/config.cells`

## Example

`cargo run -- src/glider_gun.cells`
![](https://github.com/Broyojo/game_of_life/blob/master/example_output.png)
