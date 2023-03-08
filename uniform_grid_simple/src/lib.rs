use fast_2d_array::Array2D;

type Point = (f32, f32);
pub type UniformGridSimple = Array2D<Vec<usize>>;

pub fn new(cell_size: f32, world_width: f32, world_height: f32) -> UniformGridSimple {
    let width = (world_width / cell_size) as usize;
    let height = (world_height / cell_size) as usize;
    Array2D::new(height, width)
}

pub fn insert(grid: &mut UniformGridSimple, point: Point, value: usize, cell_size: f32) {
    let (x, y) = world_to_grid(&point, cell_size);
    let cell_center = ((x as f32 * cell_size) + (cell_size / 2.), (y as f32 * cell_size) + (cell_size / 2.));

    if point.0 > cell_center.0 {
        if let Some(v) = grid.try_get_mut(x + 1, y) {
            v.push(value);
        }

        if point.1 > cell_center.1 {
            if let Some(v) = grid.try_get_mut(x + 1, y + 1) {
                v.push(value);
            }
        }
    }

    if point.0 < cell_center.0 {
        if let Some(x) = x.checked_sub(1) {
            if let Some(v) = grid.try_get_mut(x, y) {
                v.push(value);
            }
        }


        if point.1 < cell_center.1 {
            if let (Some(x), Some(y)) = (x.checked_sub(1), y.checked_sub(1)) {
                if let Some(v) = grid.try_get_mut(x, y) {
                    v.push(value);
                }
            }
        }
    }

    if point.1 > cell_center.1 {
        if let Some(v) = grid.try_get_mut(x, y + 1) {
            v.push(value);
        }
    }

    if point.1 < cell_center.1 {
        if let Some(y) = y.checked_sub(1) {
            if let Some(v) = grid.try_get_mut(x, y) {
                v.push(value);
            }
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

pub fn clear_uniform_grid_simple(grid: &mut UniformGridSimple) {
    for i in 0..grid.total_size() {
        grid.get_mut_as_1d(i).clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn insertion_test() {
        const CELL_SIZE: f32 = 10.;

        let mut grid = super::new(CELL_SIZE, 100., 100.);

        insert(&mut grid, (0., 0.), 0, CELL_SIZE);
        insert(&mut grid, (0., 0.), 1, CELL_SIZE);
        insert(&mut grid, (0.5, 0.), 2, CELL_SIZE);
        insert(&mut grid, (0., 0.5), 3, CELL_SIZE);
        insert(&mut grid, (0.5, 0.5), 4, CELL_SIZE);

        insert(&mut grid, (10., 0.), 5, CELL_SIZE);
        insert(&mut grid, (0., 10.), 6, CELL_SIZE);
        insert(&mut grid, (10., 10.), 7, CELL_SIZE);
        insert(&mut grid, (21., 21.), 8, CELL_SIZE);

        assert!(grid.get(0, 0).contains(&0)
            && grid.get(0, 0).contains(&1)
            && grid.get(0, 0).contains(&2)
            && grid.get(0, 0).contains(&3)
            && grid.get(0, 0).contains(&4)
        );

        println!("{}", grid);
    }
}

