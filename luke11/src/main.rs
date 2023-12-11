fn flood_fill(grid_input: Vec<Vec<bool>>, x: usize, y: usize) -> (bool, Vec<Vec<bool>>) {
    let mut grid = grid_input;
    // If the selected value in the list is false, return false + the grid
    // If it is true, use the flood fill algorithm to remove the island,
    // and return true + the grid
    // Copy flood fill implementation from luke 10
    if !grid[y][x] { return (false, grid) }
    (true, grid)
}

fn main() {
    let kart: Vec<Vec<bool>> = std::fs::read_to_string("src/kart.txt")
        .expect("errr")
        .trim()
        .lines()
        .map(|l| l.chars().map(|c| c=='X').collect())
        .collect();
    println!("{:?}", kart);
    // Loop through every x and y position in the grid, and run flood_fill function.
}
