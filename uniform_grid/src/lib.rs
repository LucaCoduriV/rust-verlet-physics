// An implementation of a uniform grid in Rust for the purpose of collision detection.

use crate::aabb::Aabb;

mod aabb;

struct UniformGrid<'a, V> {
    cells:Vec<Vec<Cell>>,
    cell_width: f32,
    cell_height: f32,
    store: Vec<Entity<'a, V>>,
}

impl<'a, V> UniformGrid<'a, V> {
    pub fn new (grid_width:f32, grid_height:f32, nb_cell_horizontal:usize,
                nb_cell_vertical:usize) -> Self {

        let mut cells = Vec::with_capacity(nb_cell_vertical);
        for _ in 0..nb_cell_vertical {
            let mut row = Vec::with_capacity(nb_cell_horizontal);
            for _ in 0..nb_cell_horizontal {
                row.push(Cell::new());
            }
            cells.push(row);
        }

        Self {
            cells,
            cell_width: grid_width/ nb_cell_horizontal as f32,
            cell_height:grid_height/ nb_cell_vertical as f32,
            store: Vec::new(),
        }
    }

    fn get_grid_height(&self) -> f32 {
        self.cells.len() as f32 * self.cell_height
    }

    fn get_grid_width(&self) -> f32 {
        self.cells[0].len() as f32 * self.cell_width
    }

    pub fn insert(&mut self, bbox:Aabb, data:&'a V) {
        let top_left = bbox.top_left();
        let grid_position = ((top_left.0 / self.cell_width) as usize,
                             (top_left.1 / self.cell_height) as usize);
        let width = (bbox.width / self.cell_width) as usize;
        let height = (bbox.height / self.cell_height) as usize;

        let entity_index = self.store.len();
        self.store.push(Entity::new(data));

        for i in grid_position.1..=grid_position.1 + width {
            for j in grid_position.0..=grid_position.0 + height {
                self.cells[i][j].objects.push(entity_index);
                self.store[entity_index].cells.push((i,j));
            }
        }
    }

    pub fn clear(&mut self){
        for entity in &self.store {
            for (i, j) in &entity.cells {
                self.cells[*i][*j].objects.clear()
            }
        }

        self.store.clear()
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
    cells: Vec<(usize, usize)>,
}

impl<'a, V> Entity<'a, V> {
    pub fn new(data: &'a V) -> Self{
        Self{
            data,
            cells: Vec::new(),
        }
    }
}