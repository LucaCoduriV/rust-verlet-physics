use std::f32::consts::PI;
use std::time::{Duration, Instant};

use image::GenericImageView;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use physic_engine::{Solver, Vec2, VerletObject};
use crate::drawing_functions::DrawBasicShapes;

mod physic_engine;
mod drawing_functions;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const MAX_ANGLE: f32 = 2.;
const OBJECT_SPAWN_SPEED: f32 = 500.;
const MAX_OBJECT: usize = 2250;

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

    // create random colors
    let mut rng = StdRng::seed_from_u64(42);
    let mut colors: [(u8, u8, u8); MAX_OBJECT] = [(0, 0, 0); MAX_OBJECT];
    for i in 0..colors.len() {
        colors[i] = (rng.gen(), rng.gen(), rng.gen());
    }

    // run first simulation to get all objects end position
    let objects = run_simulation(&mut canvas, &mut event_pump, &colors)?;

    // set objects color from image
    let img = image::open("./planete.webp").unwrap();
    let mut colors = vec![];
    for object in objects.iter() {
        let pixel = img.get_pixel(
            object.position_current.x as u32,
            object.position_current.y as u32,
        );
        colors.push((pixel.0[0], pixel.0[1], pixel.0[2]));
    }

    // run second simulation with image colors
    run_simulation(&mut canvas, &mut event_pump, colors.as_slice())?;

    std::thread::sleep(Duration::new(10, 0));
    Ok(())
}

fn run_simulation(canvas: &mut WindowCanvas, event_pump: &mut EventPump, colors: &[(u8, u8, u8)]) -> Result<Vec<VerletObject>, String> {
    let mut last_time = Instant::now();
    let mut nb_update: u32 = 0;
    let mut angle_counter: f32 = 0.;

    let mut objects = Vec::with_capacity(MAX_OBJECT);
    let mut solver = Solver::new();

    // Load a font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("./OpenSans-Regular.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let texture_creator = canvas.texture_creator();

    fn create_text_texture<'a>(font: &Font, texture_creator: &'a TextureCreator<WindowContext>,
                               canvas: &WindowCanvas, text: &'a str) -> Result<Texture<'a>, String> {
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(Color::RGB(0, 0, 0))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        Ok(texture)
    }

    'running: loop {
        nb_update += 1;
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_time);
        last_time = current_time;

        if objects.len() >= MAX_OBJECT {
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

        if nb_update > 1 && objects.len() < MAX_OBJECT {
            let angle: f32 = MAX_ANGLE * angle_counter.sin() + PI * 0.5;
            angle_counter += 0.1;
            let color = colors[objects.len()];
            let mut object = VerletObject::new(
                Vec2::new(WIDTH as f32 / 3., HEIGHT as f32 / 10.),
                10.,
                (color.0, color.1, color.2),
            );
            solver.set_object_velocity(
                &mut object,
                OBJECT_SPAWN_SPEED * Vec2::new(angle.cos(), angle.sin()),
            );
            objects.push(object);

            let angle: f32 = MAX_ANGLE * angle_counter.sin() + PI * 0.5;
            angle_counter += 0.1;
            let color = colors[objects.len()];
            let mut object = VerletObject::new(
                Vec2::new(700., HEIGHT as f32 / 10.), 10.,
                (color.0, color.1, color.2),
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
        //solver.draw(canvas)?;
        for (_, object) in (&objects).iter().enumerate() {
            canvas.set_draw_color(Color::RGB(object.color.0, object.color.1, object.color.2));
            canvas.fill_circle(
                object.position_current.x as i32,
                object.position_current.y as i32,
                object.radius as i32,
            )?;
        }
        let text = format!("number of object: {}", objects.len());
        let text2 = format!("frametime: {}ms", delta_time.as_millis());
        let texture = create_text_texture(&font, &texture_creator, &canvas, text.as_str())?;
        let texture2 = create_text_texture(&font, &texture_creator, &canvas, text2.as_str())?;
        canvas.copy(&texture, None, Some(Rect::new(0, 0, (text.len() * 7) as u32, 30)))?;
        canvas.copy(&texture2, None, Some(Rect::new(0, 25, (text2.len() * 7) as u32, 30)))?;
        canvas.present();
        // std::thread::sleep(Duration::from_millis(200));
    }

    Ok(objects)
}
