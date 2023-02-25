use std::time::{Duration, Instant, SystemTime};

use minifb::{MouseMode, ScaleMode, Window, WindowOptions};
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
    let mut draw_target = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut last_time = Instant::now();

    window.limit_update_rate(Some(Duration::from_micros(16600)));


    while window.is_open() {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time).as_millis();
        let delta_time_sec = delta_time as f32 / 1000.;
        last_time = current_time;

        draw_target.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0xff, 0xff, 0xff,
        ));

        Solver::update(&mut objects, delta_time_sec);

        for object in &objects {
            Circle {
                pos: (object.position_current.x, object.position_current.y),
                radius: 20.,
            }.draw(&mut draw_target);
        }
        println!("{delta_time} {delta_time_sec} {}", (1./ delta_time_sec) as i32);

        window
            .update_with_buffer(draw_target.get_data(), size.0, size.1)
            .unwrap();
    }
}