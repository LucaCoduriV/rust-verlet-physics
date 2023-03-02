use cgmath::{MetricSpace, Vector2};
use sdl2::render::WindowCanvas;
use crate::quad_tree::{Aabb, QuadTree};

pub type Vec2 = Vector2<f32>;

pub struct VerletObject {
    pub position_current: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
    pub color: (u8, u8, u8),
    pub radius: f32,
}

impl VerletObject {
    pub fn new(position_current: Vec2, radius: f32, color: (u8, u8, u8)) -> Self {
        Self {
            position_current,
            position_old: position_current,
            acceleration: Vec2::new(0., 0.),
            color,
            radius,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocitiy = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocitiy + self.acceleration * dt * dt;

        self.acceleration = Vec2::new(0., 0.);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration = self.acceleration + acc;
    }

    pub fn set_velocity(&mut self, v: Vec2, dt: f32) {
        self.position_old = self.position_current - (v * dt);
    }
}

pub struct Solver {
    gravity: Vec2,
    sub_steps: u32,
    frame_dt: f32,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0., 1.),
            sub_steps: 10,
            frame_dt: 1. / 60.,
        }
    }

    pub fn update(&mut self, objects: &mut Vec<VerletObject>) {
        let sub_dt = self.frame_dt / self.sub_steps as f32;
        for _ in 0..self.sub_steps {
            Self::apply_gravity(objects);
            Self::apply_constraint(objects);
            self.solve_collision_quadtree(objects);
            Self::update_position(objects, sub_dt);
        }
    }

    fn update_position(objects: &mut Vec<VerletObject>, dt: f32) {
        for object in objects.iter_mut() {
            object.update_position(dt);
        }
    }

    fn apply_gravity(objects: &mut Vec<VerletObject>) {
        for object in objects.iter_mut() {
            object.accelerate(Vec2::new(0., 1000.));
        }
    }

    fn apply_constraint(objects: &mut Vec<VerletObject>) {
        let constraint_center = Vec2::new(1000. / 2., 1000. / 2.);
        let constraint_radius: f32 = 500.;

        for object in objects.iter_mut() {
            let v = constraint_center - object.position_current;
            let dist: f32 = v.distance(Vec2::new(0., 0.));
            if dist > (constraint_radius - object.radius) {
                let n = v / dist;
                object.position_current = constraint_center + n * (object.radius - constraint_radius);
            }
        }
    }
    fn solve_collision_bruteforce(objects: &mut Vec<VerletObject>) {
        for ia in 0..objects.len() {
            let (left, right) = objects.split_at_mut(ia);
            let (object_a, right) = right.split_first_mut().unwrap();
            for object_b in left.iter_mut().chain(right.iter_mut()) {
                let collision_axis = object_a.position_current - object_b.position_current;
                let dist = collision_axis.distance(Vec2::new(0., 0.));
                if dist < object_a.radius + object_b.radius {
                    let n = collision_axis / dist;
                    let delta = object_a.radius + object_b.radius - dist;
                    object_a.position_current += 0.5 * delta * n;
                    object_b.position_current -= 0.5 * delta * n;
                }
            }
        }
    }
    fn solve_collision_quadtree(& mut self, objects: &mut Vec<VerletObject>) {
        const RESPONSE_COEF:f32 = 0.75;

        let mut quad_tree = QuadTree::with_store_size(Aabb::new(0, 0., 0., 1000., 1000.), 4, objects.len());

        for (id, object) in objects.iter().enumerate() {
            let x = object.position_current.x - object.radius;
            let y = object.position_current.y - object.radius;
            quad_tree.insert(Aabb::new(id, x, y, object.radius * 2., object.radius * 2.))
        }

        let intersections = quad_tree.find_all_intersection();
        for intersection in intersections {
            let a = intersection.0;
            let b = intersection.1;

            let collision_axis = Vec2::new(a.center().0, a.center().1)
                - Vec2::new(b.center().0, b.center().1);
            let dist = collision_axis.distance(Vec2::new(0., 0.));
            let min_dist:f32 = a.width / 2. + b.width / 2.;

            if dist < min_dist {
                let n = collision_axis / dist;

                let mass_ratio_1:f32 = (a.width / 2.) / min_dist;
                let mass_ratio_2:f32 = (b.width / 2.) / min_dist;

                let delta = 0.5 * RESPONSE_COEF * (dist - min_dist);
                objects[a.id].position_current -= n * (mass_ratio_2 * delta);
                objects[b.id].position_current += n * (mass_ratio_1 * delta);
            }
        }
    }

    pub fn set_object_velocity(&self, object: &mut VerletObject, velocity: Vec2) {
        object.set_velocity(velocity, self.frame_dt);
    }
}