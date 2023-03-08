use fast_2d_array::Array2D;

type Point = (f32, f32);
pub type UniformGridSimple = Array2D<Vec<usize>>;

pub fn new(cell_size: f32, world_width: f32, world_height: f32) -> UniformGridSimple {
    let width = (world_width / cell_size) as usize;
    let height = (world_height / cell_size) as usize;
    Array2D::new(height, width)
}

pub fn insert(grid: &mut UniformGridSimple, point: Point, value: usize, cell_size:f32) {
    let (x, y) = world_to_grid(&point, cell_size);
    let cell_center = ((x as f32 * cell_size) - (cell_size/2.), (y as f32 * cell_size) - (cell_size/2.));

    if point.0 > cell_center.0 {
        if let Some(v) = grid.try_get_mut(x + 1, y){
            v.push(value);
        }

        if point.1 > cell_center.1 {
            if let Some(v) = grid.try_get_mut(x + 1, y + 1){
                v.push(value);
            }
        }
    }

    if point.0 < cell_center.0 {
        if let Some(v) = grid.try_get_mut(x - 1, y){
            v.push(value);
        }

        if point.1 < cell_center.1 {
            if let Some(v) = grid.try_get_mut(x - 1, y - 1){
                v.push(value);
            }
        }
    }

    if point.1 > cell_center.1 {
        if let Some(v) = grid.try_get_mut(x, y + 1){
            v.push(value);
        }
    }

    if point.1 < cell_center.1 {
        if let Some(v) = grid.try_get_mut(x, y - 1){
            v.push(value);
        }
    }

    grid.get_mut(x, y).push(value);
}

fn world_to_grid(point: &Point, cell_size: f32) -> (usize, usize) {
    let (x, y) = point;
    let x = (x / cell_size) as usize;
    let y = (y / cell_size) as usize;
    (x, y)
}

pub fn query_cell_and_neighbours(grid: &UniformGridSimple, x: usize, y: usize) -> Vec<usize> {
    let mut values = Vec::new();
    let y_min = y.saturating_sub(1);
    let y_max = (y + 1).clamp(0, grid.get_height() - 1);
    let x_min = x.saturating_sub(1);
    let x_max = (x + 1).clamp(0, grid.get_width() - 1);

    for y2 in y_min..=y_max {
        for x2 in x_min..=x_max {
            if let Some(v) = grid.try_get(x2, y2) {
                values.extend(v.iter().cloned());
            }
        }
    }
    values
}

pub fn clear_uniform_grid_simple(grid:&mut UniformGridSimple){
    for i in 0..grid.total_size() {
        grid.get_mut_as_1d(i).clear();
    }
}


