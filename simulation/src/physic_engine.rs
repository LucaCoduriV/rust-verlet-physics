use cgmath::{MetricSpace, Vector2};
use uniform_grid_simple::{clear_uniform_grid_simple, query_cell_and_neighbours};
use work_manager::WorkerPool;
use crate::sync_vec::{SyncUniformGridSimple, SyncVec, WorkerData};

pub type Vec2 = Vector2<f32>;

const NB_THREAD: usize = 8;

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
        let velocity = self.position_current - self.position_old;
        self.position_old = self.position_current;
        self.position_current = self.position_current + velocity + self.acceleration * dt * dt;

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
    uniform_grid_simple: SyncUniformGridSimple,
    thread_pool: WorkerPool,
    world_height:f32,
    world_width:f32,
    cell_size:f32,
}

impl Solver {
    pub fn new(cell_size:f32, world_height:f32, world_width:f32) -> Self {

        Self {
            gravity: Vec2::new(0., 1.),
            sub_steps: 8,
            frame_dt: 1. / 60.,
            uniform_grid_simple: SyncUniformGridSimple(uniform_grid_simple::new(cell_size,
                                                                                world_width,
                                                                                world_height)),
            thread_pool: WorkerPool::new(8),
            world_height,
            world_width,
            cell_size,
        }
    }

    pub fn update(&mut self, objects: &mut SyncVec) {
        let sub_dt = self.frame_dt / self.sub_steps as f32;
        for _ in 0..self.sub_steps {
            self.apply_gravity(objects);
            Self::apply_constraint(objects);
            self.solve_collision_multithreaded(objects);
            Self::update_position(objects, sub_dt);
        }
    }

    fn update_position(objects: &mut Vec<VerletObject>, dt: f32) {
        for object in objects.iter_mut() {
            object.update_position(dt);
        }
    }

    fn apply_gravity(&self, objects: &mut Vec<VerletObject>) {
        for object in objects.iter_mut() {
            object.accelerate(self.gravity);
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

    fn solve_collision_multithreaded(&mut self, objects: &mut SyncVec) {
        const CELL_SIZE: f32 = 5.;
        const WORLD_HEIGHT: f32 = 1000.;
        const WORLD_WIDTH: f32 = 1000.;
        const NB_CELL: usize = (WORLD_WIDTH / CELL_SIZE) as usize;

        clear_uniform_grid_simple(&mut self.uniform_grid_simple);

        for (i, o) in objects.iter().enumerate() {
            uniform_grid_simple::insert(&mut self.uniform_grid_simple, (o.position_current.x, o.position_current.y), i, CELL_SIZE);
        }

        let workerdata = WorkerData(
            objects as *mut SyncVec,
            &self.uniform_grid_simple as *const SyncUniformGridSimple,
        );

        self.thread_pool.execute_on_all(move |thread_id| {
            let data = workerdata;
            let objects = unsafe { &mut *data.0 };
            let ref uniform_grid_simple = unsafe { &(*data.1).0 };
            let from = thread_id * (NB_CELL / NB_THREAD);
            let to = ((thread_id * (NB_CELL / NB_THREAD)) + (NB_CELL / NB_THREAD))
                .clamp(0, NB_CELL);

            for x in from..to {
                for y in 0..uniform_grid_simple.len() {
                    let others = query_cell_and_neighbours(uniform_grid_simple, x, y);

                    // TODO c'est faux ce que je fais ici mais ça marche quand même,
                    //      mais surrement moins bien
                    for o1 in &others {
                        for o2 in &others {
                            if o1 != o2 {
                                let collision_axis = objects[*o1].position_current -
                                    objects[*o2].position_current;
                                let dist = collision_axis.distance(Vec2::new(0., 0.));
                                if dist < objects[*o1].radius + objects[*o2].radius {
                                    let n = collision_axis / dist;
                                    let delta = objects[*o1].radius + objects[*o2].radius - dist;
                                    objects[*o1].position_current += 0.5 * delta * n;
                                    objects[*o2].position_current -= 0.5 * delta * n;
                                }
                            }
                        }
                    }
                }
            }
        });

        self.thread_pool.wait_all_finish();
    }

    pub fn set_object_velocity(&self, object: &mut VerletObject, velocity: Vec2) {
        object.set_velocity(velocity, self.frame_dt);
    }
}