type Point = (f32, f32);

#[derive(Debug, Clone)]
pub struct Aabb {
    pub id: usize,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Aabb {
    pub fn new(id: usize, x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            id,
            x,
            y,
            width,
            height,
        }
    }
    pub fn contains_point(&self, point: Point) -> bool {
        return point.0 >= self.left()
            && point.0 <= self.right()
            && point.1 >= self.top()
            && point.1 <= self.bottom();
    }
    pub fn contains_aabb(&self, other: &Aabb) -> bool {
        return other.left() >= self.left()
            && other.right() <= self.right()
            && other.top() >= self.top()
            && other.bottom() <= self.bottom();
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        !(other.left() > self.right()
            || other.right() < self.left()
            || other.top() > self.bottom()
            || other.bottom() < self.top())
    }

    pub fn center(&self) -> Point {
        (self.x + self.width / 2., self.y + self.height / 2.)
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn bottom_right(&self) -> Point {
        (self.right(), self.bottom())
    }

    pub fn bottom_left(&self) -> Point {
        (self.left(), self.bottom())
    }

    pub fn top_right(&self) -> Point {
        (self.right(), self.top())
    }

    pub fn top_left(&self) -> Point {
        (self.left(), self.top())
    }
}