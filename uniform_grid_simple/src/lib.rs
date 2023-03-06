type Point = (f32, f32);
pub type UniformGridSimple = Vec<Vec<Vec<usize>>>;

pub fn new(cell_size: f32, world_width: f32, world_height: f32) -> UniformGridSimple {
    let x = (world_width / cell_size) as usize;
    let y = (world_height / cell_size) as usize;
    vec![vec![Vec::new(); x]; y]
}

pub fn insert(grid: &mut UniformGridSimple, point: Point, value: usize, cell_size:f32) {
    let (x, y) = point;
    let x = (x / cell_size) as usize;
    let y = (y / cell_size) as usize;
    grid[x][y].push(value);
}

pub fn query_cell_and_neighbours(grid: UniformGridSimple, x: usize, y: usize) -> Vec<usize> {
    let mut values = Vec::new();

    for i in y.saturating_sub(1)..=x + 1 {
        for j in x.saturating_sub(1)..=y + 1 {
            values.extend(grid[i][j].iter().cloned());
        }
    }
    values
}

pub fn clear_uniform_grid_simple(grid:&mut UniformGridSimple){
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            grid[i][j].clear();
        }
    }
}
