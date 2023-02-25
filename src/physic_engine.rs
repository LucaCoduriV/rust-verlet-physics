use cgmath::Vector2;

pub type Vec2 = Vector2<f32>;

pub struct VerletObject {
    pub position_current: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
}

impl VerletObject {
    pub fn new(position_current: Vec2) -> Self {
        Self{
            position_current,
            position_old: position_current,
            acceleration: Vec2::new(0.,0.),
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocitiy = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocitiy + self.acceleration * dt * dt ;

        self.acceleration = Vec2::new(0., 0.);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration = self.acceleration + acc;
    }
}

pub struct Solver {
    gravity: Vec2,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0., 1.),
        }
    }

    pub fn update(objects: &mut Vec<VerletObject>, dt: f32){
        Self::apply_gravity(objects);
        Self::update_position(objects, dt);
    }

    fn update_position(objects: &mut Vec<VerletObject>, dt: f32){
        for object in objects.iter_mut(){
            object.update_position(dt);
        }
    }

    fn apply_gravity(objects: &mut Vec<VerletObject>) {
        for object in objects.iter_mut(){
            object.accelerate(Vec2::new(0., 1000.));
        }
    }
}