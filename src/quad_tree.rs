use std::cell::RefCell;

type Point = (f32, f32);

#[derive(Debug, Clone)]
pub struct Aabb {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Aabb {
    pub fn contains(&self, point: Point) -> bool {
        return point.0 >= self.x
            && point.0 <= self.x + self.width
            && point.1 >= self.y
            && point.1 <= self.y + self.height;
    }
}

#[derive(Debug)]
pub struct QuadTree {
    capacity: usize,
    boundary: Aabb,
    points: Vec<Point>,

    divided: bool,
    north_east: Option<Box<RefCell<QuadTree>>>,
    north_west: Option<Box<RefCell<QuadTree>>>,
    south_east: Option<Box<RefCell<QuadTree>>>,
    south_west: Option<Box<RefCell<QuadTree>>>,
}

impl QuadTree {
    pub fn new(boundary: Aabb, capacity: usize) -> Self {
        Self {
            capacity,
            boundary,
            divided: false,
            north_east: None,
            north_west: None,
            south_east: None,
            south_west: None,
            points: Vec::with_capacity(capacity),
        }
    }

    pub fn insert(&mut self, point: Point) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }

        if self.points.len() < self.capacity {
            self.points.push(point);
            return true;
        }

        if !self.divided {
            self.divide();
        }

        self.north_west.as_mut().unwrap().borrow_mut().insert(point)
            || self.north_east.as_mut().unwrap().borrow_mut().insert(point)
            || self.south_east.as_mut().unwrap().borrow_mut().insert(point)
            || self.south_west.as_mut().unwrap().borrow_mut().insert(point)
    }

    pub fn query(&self, range: Aabb) -> Vec<Point> {
        let mut arr = Vec::new();
        self.query_rec(range, &mut arr);
        arr
    }

    fn query_rec(&self, range: Aabb, arr: &mut Vec<Point>) -> bool {
        for p in self.points.iter() {
            if range.contains(p.clone()) {
                arr.push(p.clone())
            }
        }

        if let Some(qt) = self.north_west.as_ref() {
            if qt.borrow().query_rec(range.clone(), arr) {
                return true;
            }
        }

        if let Some(qt) = self.north_east.as_ref() {
            if qt.borrow().query_rec(range.clone(), arr) {
                return true;
            }
        }

        if let Some(qt) = self.south_west.as_ref() {
            if qt.borrow().query_rec(range.clone(), arr) {
                return true;
            }
        }

        if let Some(qt) = self.south_east.as_ref() {
            if qt.borrow().query_rec(range.clone(), arr) {
                return true;
            }
        }

        false
    }

    fn divide(&mut self) {
        let ne = Aabb {
            x: self.boundary.x + self.boundary.width / 2.,
            y: self.boundary.y,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.north_east = Some(Box::new(RefCell::new(QuadTree::new(ne, self.capacity))));

        let nw = Aabb {
            x: self.boundary.x,
            y: self.boundary.y,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.north_west = Some(Box::new(RefCell::new(QuadTree::new(nw, self.capacity))));

        let se = Aabb {
            x: self.boundary.x + self.boundary.width / 2.,
            y: self.boundary.y + self.boundary.height / 2.,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.south_east = Some(Box::new(RefCell::new(QuadTree::new(se, self.capacity))));

        let sw = Aabb {
            x: self.boundary.x,
            y: self.boundary.y + self.boundary.height / 2.,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.south_west = Some(Box::new(RefCell::new(QuadTree::new(sw, self.capacity))));

        self.divided = true;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quad_tree_test() {
        let boundary = Aabb {
            x: 0.,
            y: 0.,
            width: 500.,
            height: 500.,
        };
        let mut qt = QuadTree::new(boundary.clone(), 2);

        let pt1 = (50., 80.);
        let pt2 = (270., 80.);
        let pt3 = (100., 40.);

        qt.insert(pt1);
        qt.insert(pt2);
        qt.insert(pt3);

        println!("{:#?}", qt);

        let boundary = Aabb {
            x: 90.,
            y: 30.,
            width: 20.,
            height: 20.,
        };

        println!("{:#?}", qt.query(boundary.clone()))
    }
}
