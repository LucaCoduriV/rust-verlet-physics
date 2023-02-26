use cgmath::{MetricSpace, Vector2};

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
        let sub_steps = 8;
        let sub_dt = dt / sub_steps as f32;
        for _ in 0..sub_steps {
            Self::apply_gravity(objects);
            Self::apply_constraint(objects);
            Self::solve_collision(objects);
            Self::update_position(objects, sub_dt);
        }

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

    fn apply_constraint(objects: &mut Vec<VerletObject>){
        let constraint_center = Vec2::new(500., 500.);
        let constraint_radius:f32 = 300.;
        let object_radius:f32 = 10.;

        for object in objects.iter_mut(){
            let v = constraint_center - object.position_current;
            let dist:f32 = v.distance(Vec2::new(0.,0.));
            if dist > (constraint_radius - object_radius) {
                let n = v / dist;
                object.position_current = constraint_center + n * (object_radius - constraint_radius);
            }
        }
    }

    fn solve_collision(objects: &mut Vec<VerletObject>){
        let object_radius:f32 = 10.;

        for ia in 0..objects.len() {
            let (left, right) = objects.split_at_mut(ia);
            let (object_a, right) = right.split_first_mut().unwrap();
            for object_b in left.iter_mut().chain(right.iter_mut()) {
                let collision_axis = object_a.position_current - object_b.position_current;
                let dist = collision_axis.distance(Vec2::new(0.,0.));
                if dist < 2. * object_radius {
                    let n = collision_axis / dist;
                    let delta = 2. * object_radius - dist;
                    object_a.position_current += 0.5 * delta * n;
                    object_b.position_current -= 0.5 * delta * n;
                }
            }
        }

    }
}