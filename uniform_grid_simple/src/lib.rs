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
    grid[y][x].push(value);
}

pub fn query_cell_and_neighbours(grid: &UniformGridSimple, x: usize, y: usize) -> Vec<usize> {
    let mut values = Vec::new();
    let y_min = y.saturating_sub(1);
    let y_max = (y + 1).clamp(0, grid.capacity() - 1);
    let x_min = x.saturating_sub(1);
    let x_max = (x + 1).clamp(0, grid[0].capacity() - 1);

    for y2 in y_min..=y_max {
        for x2 in x_min..=x_max {
            values.extend(grid[y2][x2].iter().cloned());
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


