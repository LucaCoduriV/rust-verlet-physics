// An implementation of a uniform grid in Rust for the purpose of collision detection.

struct UniformGrid<'a, V> {
    cells:Vec<Vec<Cell>>,
    cell_size: f32,
    store: Vec<Entity<'a, V>>,
}

impl<'a, V> UniformGrid<'a, V> {
    pub fn new (width:usize, height:usize, cell_size: f32) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Cell::new());
            }
            cells.push(row);
        }

        Self {
            cells,
            cell_size,
            store: Vec::new(),
        }
    }

    pub fn get_grid_height(&self) -> f32 {
        self.cells.len() as f32 * self.cell_size
    }

    pub fn get_grid_width(&self) -> f32 {
        self.cells[0].len() as f32 * self.cell_size
    }
}


struct Cell {
    objects: Vec<usize>,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
}

struct Entity<'a, V> {
    data: &'a V,
    cells: Vec<usize>,
}