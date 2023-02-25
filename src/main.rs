use std::f32::consts::PI;
use std::time::SystemTime;

use minifb::{MouseMode, Window, WindowOptions};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use crate::circle::Circle;
use crate::physic_engine::{Solver, Vec2, VerletObject};

mod circle;
mod physic_engine;

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let mut window = Window::new(
        "2D Physic engine",
        WIDTH,
        HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
        .unwrap();


    let mut objects = vec![
        VerletObject::new(Vec2::new(200., 0.)),
        VerletObject::new(Vec2::new(100., 0.)),
    ];

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut lastTime = SystemTime::now();


    while window.is_open() {
        let currentTime = SystemTime::now();

        let deltaTime = currentTime.duration_since(lastTime).unwrap().as_millis() as f32 / 1000.;

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));

        Solver::update(&mut objects , deltaTime);

        for object in &objects {
            Circle {
                pos: (object.position_current.x, object.position_current.y),
                radius: 20.,
            }.draw(&mut dt);
        }

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();

        println!("{}", 1./deltaTime);

        lastTime = currentTime;
    }
}