mod puzzle_input;


fn count_trees_3_1_slope(forest_grid: &Vec<Vec<ForestTile>>) -> i32 {
    const HORIZONTAL: i32 = 3;
    const VERTICAL: i32 = 1;
    let height = forest_grid.len() as i32;
    let width = forest_grid[0].len() as i32;

    let mut trees_hit = 0;
    
    let mut pos = (0, 0);
    while pos.1 != height {
        if forest_grid[pos.1 as usize][(pos.0 % width) as usize] == ForestTile::Tree {
            trees_hit += 1;
        }
        
        pos.0 += HORIZONTAL;
        pos.1 += VERTICAL;
    }

    trees_hit
}

fn count_trees(slope: (i32, i32), forest_grid: &Vec<Vec<ForestTile>>) -> i32 {
    let height = forest_grid.len() as i32;
    let width = forest_grid[0].len() as i32;

    let mut trees_hit = 0;
    
    let mut pos = (0, 0);
    while pos.1 < height {
        if forest_grid[pos.1 as usize][(pos.0 % width) as usize] == ForestTile::Tree {
            trees_hit += 1;
        }
        
        pos.0 += slope.0;
        pos.1 += slope.1;
    }

    trees_hit
}

#[derive(Debug, PartialEq, Eq)]
enum ForestTile{
    Tree,
    Empty
}

impl From<char> for ForestTile {
    fn from(c: char) -> Self {
        match c {
            '.' => { ForestTile::Empty }
            '#' => { ForestTile::Tree }
            _ => { panic!("Unknown tile in puzzle_input!"); }
        }
    }
}

fn main() {
    let forest_grid = puzzle_input::FOREST_GRID.lines().map(|line| line.chars().map(ForestTile::from).collect()).collect();

    let trees_hit_3_1 = count_trees_3_1_slope(&forest_grid);
    println!("Trees hit (slope 3-1 specifically): {}", trees_hit_3_1);
    println!("");
    
    let mut tree_multiplied_accumulator = 1;
    for slope in vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)] {
        let trees = count_trees(slope, &forest_grid);
        tree_multiplied_accumulator *= trees;
        println!("Tree count (slope{:?}): {}", slope, trees);
    }
    println!("Tree count multiplied together: {}", tree_multiplied_accumulator);
}
