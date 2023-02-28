use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use std::f32::consts::PI;
use std::time::{Duration, Instant};
use image::GenericImageView;
use sdl2::EventPump;

use crate::physic_engine::{Solver, Vec2, VerletObject};
use crate::quad_tree::{Aabb, QuadTree};

mod physic_engine;
mod quad_tree;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const MAX_ANGLE: f32 = 2.;
const OBJECT_SPAWN_SPEED: f32 = 100.;
const MAX_OBJECT: usize = 1500;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust verlet physics", WIDTH, HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    let objects = run_loop(&mut canvas, &mut event_pump)?;

    let img = image::open("./planete.webp").unwrap();
    let mut colors = vec![];
    for object in objects.iter() {
        let pixel = img.get_pixel(
            object.position_current.x as u32,
            object.position_current.y as u32,
        );
        colors.push((pixel.0[0], pixel.0[1], pixel.0[2]));
    }

    let objects = run_loop(&mut canvas, &mut event_pump)?;

    std::thread::sleep(Duration::new(10, 0));

    Ok(())

}

fn run_loop(canvas: &mut WindowCanvas, event_pump: &mut EventPump) -> Result<Vec<VerletObject>, String> {
    let mut last_time = Instant::now();
    let mut nb_update: u32 = 0;
    let mut angle_counter: f32 = 0.;

    let mut objects = vec![];
    let solver = Solver::new();
    let mut rng = StdRng::seed_from_u64(42);

    'running: loop {
        nb_update += 1;
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);
        last_time = current_time;

        if objects.len() > MAX_OBJECT {
            break 'running;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        std::thread::sleep(Duration::saturating_sub(
            Duration::from_micros(16333),
            delta_time,
        ));

        if nb_update > 1 {
            let angle: f32 = MAX_ANGLE * angle_counter.sin() + PI * 0.5;
            angle_counter += 0.1;
            let mut object = VerletObject::new(
                Vec2::new(WIDTH as f32 / 3., HEIGHT as f32 / 10.),
                10.,
                (rng.gen(), rng.gen(), rng.gen()),
            );
            solver.set_object_velocity(
                &mut object,
                OBJECT_SPAWN_SPEED * Vec2::new(angle.cos(), angle.sin()),
            );
            objects.push(object);

            nb_update = 0;
        }

        solver.update(&mut objects);

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas
            .fill_circle((WIDTH / 2) as i32, (HEIGHT / 2) as i32, 500)
            .unwrap();

        for (_, object) in (&objects).iter().enumerate() {
            canvas.set_draw_color(Color::RGB(object.color.0, object.color.1, object.color.2));
            canvas.fill_circle(
                object.position_current.x as i32,
                object.position_current.y as i32,
                object.radius as i32,
            )?;
        }

        canvas.present();
    }

    Ok(objects)
}

trait DrawBasicShapes {
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String>;
}

impl DrawBasicShapes for WindowCanvas {
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String> {
        let mut offset_x: i32 = 0;
        let mut offset_y: i32 = radius;
        let mut d: i32 = radius - 1;

        while offset_y >= offset_x {
            self.draw_line((x - offset_y, y + offset_x), (x + offset_y, y + offset_x))?;
            self.draw_line((x - offset_x, y + offset_y), (x + offset_x, y + offset_y))?;
            self.draw_line((x - offset_x, y - offset_y), (x + offset_x, y - offset_y))?;
            self.draw_line((x - offset_y, y - offset_x), (x + offset_y, y - offset_x))?;

            if d >= 2 * offset_x {
                d -= 2 * offset_x + 1;
                offset_x += 1;
            } else if d < 2 * (radius - offset_y) {
                d += 2 * offset_y - 1;
                offset_y -= 1;
            } else {
                d += 2 * (offset_y - offset_x - 1);
                offset_y -= 1;
                offset_x += 1;
            }
        }

        return Ok(());
    }
}
