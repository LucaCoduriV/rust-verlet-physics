use cgmath::{AbsDiffEq, InnerSpace, MetricSpace, Vector2};
use cgmath::num_traits::{abs, pow};
use stopwatch::Stopwatch;
use uniform_grid_simple::{clear_uniform_grid_simple};
use work_manager::WorkerPool;
use crate::sync_vec::{SyncUniformGridSimple, SyncVec, WorkerData};

pub type Vec2 = Vector2<f32>;

const NB_THREAD: usize = 8;

#[derive(Debug)]
pub struct VerletObject {
    pub position_current: Vec2,
    pub position_old: Vec2,
    pub acceleration: Vec2,
    pub color: (u8, u8, u8),
    pub radius: f32,
    pub inertia: f32,
    pub move_acc: f32,
}

impl VerletObject {
    pub fn new(position_current: Vec2, radius: f32, color: (u8, u8, u8)) -> Self {
        Self {
            position_current,
            position_old: position_current,
            acceleration: Vec2::new(0., 0.),
            color,
            radius,
            inertia: 1.,
            move_acc: 0.,
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        let velocity = self.position_current - self.position_old;

        self.inertia = 1. + self.move_acc / (velocity.magnitude() + 1.);
        self.move_acc *= 0.5;

        self.acceleration -= velocity * 35.;

        let anti_pressure_factor:f32 = pow(1. / self.inertia, 2);

        self.position_old = self.position_current;
        self.position_current += velocity + self.acceleration * anti_pressure_factor
            * dt * dt;

        self.acceleration = Vec2::new(0., 0.);
    }

    pub fn accelerate(&mut self, acc: Vec2) {
        self.acceleration = self.acceleration + acc;
    }

    pub fn move_self(&mut self, delta: Vec2) {
        self.position_current += delta;
        self.move_acc += abs(delta.x) + abs(delta.y);
    }

    pub fn set_velocity(&mut self, v: Vec2, dt: f32) {
        self.position_old += -v * dt;
    }

    pub fn velocity(&self) -> Vec2 {
        self.position_current - self.position_old
    }
}


pub struct Solver {
    gravity: Vec2,
    sub_steps: u32,
    frame_dt: f32,
    uniform_grid_simple: SyncUniformGridSimple,
    thread_pool: WorkerPool,
    world_height: f32,
    world_width: f32,
    cell_size: f32,
    pub timer: Stopwatch,
}

impl Solver {
    pub fn new(cell_size: f32, world_height: f32, world_width: f32) -> Self {
        Self {
            gravity: Vec2::new(0., 200.),
            sub_steps: 1,
            frame_dt: 1. / 60.,
            uniform_grid_simple: SyncUniformGridSimple(uniform_grid_simple::new(cell_size,
                                                                                world_width,
                                                                                world_height)),
            thread_pool: WorkerPool::new(NB_THREAD),
            world_height,
            world_width,
            cell_size,
            timer: Stopwatch::new(),
        }
    }

    pub fn update(&mut self, objects: &mut SyncVec) {
        self.timer.restart();
        self.apply_gravity(objects);

        Self::apply_constraint(objects);
        self.solve_collision_multithreaded(objects);
        Self::apply_constraint(objects);

        Self::update_position(objects, self.frame_dt);

        // let sub_dt = self.frame_dt / self.sub_steps as f32;
        // for _ in 0..self.sub_steps {
        //
        //
        //     //Self::solve_collision_brute_force(objects);
        //
        //     Self::update_position(objects, sub_dt);
        // }
        self.timer.stop();
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

            let pos = object.position_current;
            let radius = object.radius;

            if pos.x < radius {
                object.move_self(Vec2::new(radius - pos.x, 0.0));
            } else if pos.x > 1000. - radius{
                object.move_self(Vec2::new(1000. - radius - pos.x, 0.0));
            }

            if pos.y < radius {
                object.move_self(Vec2::new(0.0, radius - pos.y));
            } else if pos.y > 1000. - radius {
                object.move_self(Vec2::new(0., 1000. - radius - pos.y));
            }


            // let v = constraint_center - object.position_current;
            // let dist: f32 = v.distance(Vec2::new(0., 0.));
            // if dist > (constraint_radius - object.radius) {
            //     let n = v / dist;
            //     //object.position_current = constraint_center + n * (object.radius -
            //       //  constraint_radius);
            //     object.move_self(n * (object.radius - constraint_radius))
            // }
        }



    }

    fn solve_collision_multithreaded(&mut self, objects: &mut SyncVec) {
        let nb_cell: usize = (self.world_width / self.cell_size) as usize;

        clear_uniform_grid_simple(&mut self.uniform_grid_simple);

        for (i, o) in objects.iter().enumerate() {
            uniform_grid_simple::insert(&mut self.uniform_grid_simple, (o.position_current.x, o.position_current.y), i, self.cell_size);
        }

        let worker_data = WorkerData(
            objects as *mut SyncVec,
            &self.uniform_grid_simple as *const SyncUniformGridSimple,
        );

        // first half
        self.thread_pool.execute_on_all(move |thread_id| {
            let data = worker_data;
            let objects = unsafe { &mut *data.0 };
            let ref uniform_grid_simple = unsafe { &(*data.1).0 };

            let width = nb_cell / NB_THREAD;

            let half_width = width / 2;

            let start_index = thread_id * width;
            let end_index = start_index + half_width;

            //println!("firsthalf -> thread_id: {}, start_index: {}, end_index: {}, nb_cell: {}", thread_id, start_index, end_index, nb_cell);

            for x in start_index..end_index {
                for y in 0..uniform_grid_simple.get_height() {
                    let cell = uniform_grid_simple.get(x, y);
                    for i in 0..cell.len() {
                        for j in (i + 1)..cell.len() {
                            Self::solve_object_to_object_collision(cell[i], cell[j], objects);
                        }
                    }
                }
            }
        });
        self.thread_pool.wait_all_finish();

        // second half
        self.thread_pool.execute_on_all(move |thread_id| {
            let data = worker_data;
            let objects = unsafe { &mut *data.0 };
            let ref uniform_grid_simple = unsafe { &(*data.1).0 };

            let width = nb_cell / NB_THREAD;
            let width_rest = nb_cell % NB_THREAD;

            let half_width = width / 2;
            let half_width_rest = width % 2;

            let start_index = thread_id * width + half_width;
            let end_index = if thread_id == NB_THREAD - 1 {
                start_index + half_width + half_width_rest + width_rest
            } else {
                start_index + half_width + half_width_rest
            };

            //println!("secondhalf -> thread_id: {}, start_index: {}, end_index: {}, nb_cell: {}", thread_id, start_index, end_index, nb_cell);


            for x in start_index..end_index {
                for y in 0..uniform_grid_simple.get_height() {
                    let cell = uniform_grid_simple.get(x, y);
                    for i in 0..cell.len() {
                        for j in (i + 1)..cell.len() {
                            Self::solve_object_to_object_collision(cell[i], cell[j], objects);
                        }
                    }
                }
            }
        });
        self.thread_pool.wait_all_finish();
    }

    fn solve_object_to_object_collision(object_a: usize, object_b: usize, objects: &mut SyncVec) {
        let (b1, b2) = unsafe{
            (
                &mut *(&mut objects[object_a] as *mut VerletObject),
                &mut *(&mut objects[object_b] as *mut VerletObject)
            )
        };


        let col_radius = b1.radius + b2.radius;
        let col_axe = b1.position_current - b2.position_current;
        let length2 = col_axe.magnitude2();

        if length2 < col_radius*col_radius && length2 > 0.01 {
            let m1 = b1.inertia;
            let m2 = b2.inertia;
            let mass_tot = 1. / (m1 + m2);
            let mass_factor_1 = m1 * mass_tot;
            let mass_factor_2 = m2 * mass_tot;
            let delta_col = 0.5 * (col_radius - col_axe.magnitude());

            let col_axe = col_axe.normalize();
            b1.move_self(col_axe * (delta_col * mass_factor_2));
            b2.move_self(col_axe * (-delta_col * mass_factor_1));

            let cohesion = 0.1;
            let delta_v = b1.velocity() - b2.velocity();

            b1.set_velocity(-cohesion * delta_v, 1./60.);
            b2.set_velocity(cohesion * delta_v, 1./60.);

        }

        //
        // let collision_axis = objects[object_a].position_current -
        //     objects[object_b].position_current;
        // let dist = collision_axis.distance(Vec2::new(0., 0.));
        // if dist < objects[object_a].radius + objects[object_b].radius {
        //     let n = collision_axis / dist;
        //     let delta = objects[object_a].radius + objects[object_b].radius - dist;
        //     objects[object_a].position_current += 0.5 * delta * n;
        //     objects[object_b].position_current -= 0.5 * delta * n;
        // }
    }

    pub fn set_object_velocity(&self, object: &mut VerletObject, velocity: Vec2) {
        object.set_velocity(velocity, self.frame_dt);
    }
}