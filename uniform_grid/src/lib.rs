// An implementation of a uniform grid in Rust for the purpose of collision detection.

pub use crate::aabb::Aabb;

mod aabb;

pub struct UniformGrid<V>
    where V: Clone {
    cells: Vec<Vec<Cell>>,
    cell_width: f32,
    cell_height: f32,
    pub store: Vec<Entity<V>>,
}

impl<V> UniformGrid<V> where V: Clone {
    pub fn new(grid_width: f32, grid_height: f32, nb_cell_horizontal: usize,
               nb_cell_vertical: usize) -> Self {
        let mut cells = Vec::with_capacity(nb_cell_vertical);
        for _ in 0..=nb_cell_vertical {
            let mut row = Vec::with_capacity(nb_cell_horizontal);
            for _ in 0..=nb_cell_horizontal {
                row.push(Cell::new());
            }
            cells.push(row);
        }

        Self {
            cells,
            cell_width: grid_width / nb_cell_horizontal as f32,
            cell_height: grid_height / nb_cell_vertical as f32,
            store: Vec::new(),
        }
    }

    fn get_grid_height(&self) -> f32 {
        self.cells.len() as f32 * self.cell_height
    }

    fn get_grid_width(&self) -> f32 {
        self.cells[0].len() as f32 * self.cell_width
    }

    pub fn insert(&mut self, bbox: Aabb, data: V) {
        let top_left = bbox.top_left();
        let grid_position = (
            (top_left.0 / self.cell_width) as usize,
            (top_left.1 / self.cell_height) as usize
        );
        let width = ((top_left.0 + bbox.width) / self.cell_width) as usize;
        let height = ((top_left.1 + bbox.height) / self.cell_height) as usize;

        let entity_index = self.store.len();
        self.store.push(Entity::new(data));

        for i in grid_position.1..=height {
            for j in grid_position.0..=width {
                self.cells[i][j].objects.push(entity_index);
                self.store[entity_index].cells.push((i, j));
            }
        }
    }

    pub fn clear(&mut self) {
        for entity in &self.store {
            for (i, j) in &entity.cells {
                self.cells[*i][*j].objects.clear()
            }
        }

        self.store.clear()
    }

    pub fn get_all_collisions(&mut self) -> Vec<(V, V)> {
        let mut result = Vec::new();

        for object in 0..self.store.len() {
            if self.store[object].checked {
                continue;
            }
            for (i, j) in self.store[object].cells.iter() {
                for other in self.cells[*i][*j].objects.iter() {
                    if *other == object || self.store[*other].checked {
                        continue;
                    }
                    result.push((self.store[object].data.clone(), self.store[*other].data.clone()))
                }
            }
            self.store[object].checked = true;
        }
        result
    }
}

#[derive(Debug)]
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

pub struct Entity<V> where V: Clone {
    pub data: V,
    pub cells: Vec<(usize, usize)>,
    pub checked: bool,
}

impl<V> Entity<V> where V: Clone {
    pub fn new(data: V) -> Self {
        Self {
            data,
            cells: Vec::new(),
            checked: false,
        }
    }
}