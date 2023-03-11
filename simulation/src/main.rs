use std::f32::consts::PI;
use std::time::{Duration};
use colors_transform::{Color, Hsl};

use image::GenericImageView;
use sdl2::event::Event;
use sdl2::EventPump;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color as SdlColor, PixelFormatEnum};
use sdl2::rect::{Rect};
use sdl2::render::{Texture, TextureCreator, WindowCanvas};
use sdl2::ttf::Font;
use sdl2::video::WindowContext;
use physic_engine::{Solver, Vec2, VerletObject};
use crate::sync_vec::SyncVec;

mod physic_engine;
mod drawing_functions;
mod sync_vec;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;
const OBJECT_SPAWN_SPEED: f32 = 500.;
const MAX_OBJECT: usize = 38_000;
const CIRCLE_RADIUS: f32 = 3.;

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
    canvas.set_draw_color(SdlColor::RGB(255, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // run first simulation to get all objects end position
    let objects = run_simulation(&mut canvas, &mut event_pump, None)?;
    // set objects color from image
    let img = image::open("./planete.webp").unwrap();
    let mut colors = vec![];
    for object in objects.iter() {
        let pixel = img.get_pixel(
            (object.position_current.x as u32).clamp(0, WIDTH - 1),
            (object.position_current.y as u32).clamp(0, HEIGHT - 1),
        );
        colors.push((pixel.0[0], pixel.0[1], pixel.0[2]));
    }

    // run second simulation with image colors
    run_simulation(&mut canvas, &mut event_pump, Some(colors.as_slice()))?;

    std::thread::sleep(Duration::new(10, 0));
    Ok(())
}

fn run_simulation(
    canvas: &mut WindowCanvas,
    event_pump: &mut EventPump,
    colors: Option<&[(u8, u8, u8)]>)
    -> Result<SyncVec, String> {
    let mut loop_count: u32 = 0;
    let mut stop_watch = stopwatch::Stopwatch::start_new();

    let mut objects = SyncVec::new(Vec::with_capacity(MAX_OBJECT));
    let mut solver = Solver::new(CIRCLE_RADIUS * 2., HEIGHT as f32, WIDTH as f32);

    // Load a font
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("./OpenSans-Regular.ttf", 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let texture_creator = canvas.texture_creator();
    fn create_text_texture<'a>(font: &Font, texture_creator: &'a TextureCreator<WindowContext>,
                               text: &'a str) -> Result<Texture<'a>, String> {
        // render a surface, and convert it to a texture bound to the canvas
        let surface = font
            .render(text)
            .blended(SdlColor::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;


        Ok(texture)
    }
    let circle_surface = sdl2::surface::Surface::from_file("circle.png")?.convert_format(PixelFormatEnum::ARGB8888)?;
    let mut circle_texture = texture_creator.create_texture_from_surface(circle_surface).map_err(|e| e.to_string())?;
    circle_texture.set_blend_mode(sdl2::render::BlendMode::Blend);
    let mut color_gradient_counter = 0.;

    'draw_loop: loop {
        loop_count += 1;
        let delta_time = stop_watch.elapsed();
        stop_watch.restart();

        if objects.len() >= MAX_OBJECT {
            break 'draw_loop;
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'draw_loop,
                _ => {}
            }
        }

        canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        canvas.clear();

        if loop_count > 1 && objects.len() < MAX_OBJECT {
            const CANNON_X: f32 = 10.;
            const CANNON_Y: f32 = 100.;

            let mut build_cannon = |cannon_x: f32, cannon_y: f32, angle: f32, speed: f32| {
                let color = if let Some(colors) = colors {
                    colors[objects.len()]
                } else {
                    let rgb = Hsl::from(color_gradient_counter, 100., 50.).to_rgb().as_tuple();
                    (rgb.0 as u8, rgb.1 as u8, rgb.2 as u8)
                };
                color_gradient_counter = if color_gradient_counter >= 360. {
                    0.
                } else {
                    color_gradient_counter + 0.01
                };

                let angle: f32 = PI * angle / 180.;
                let mut object = VerletObject::new(
                    Vec2::new(cannon_x, cannon_y),
                    CIRCLE_RADIUS,
                    (color.0, color.1, color.2),
                );
                solver.set_object_velocity(
                    &mut object,
                    speed * Vec2::new(angle.cos(), angle.sin()),
                );
                objects.push(object);
            };

            build_cannon(CANNON_X, CANNON_Y - 10., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 0.0, 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 10., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 20., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 30., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 40., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 50., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 60., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 70., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 80., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 90., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 100., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 110., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 120., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 130., 0., OBJECT_SPAWN_SPEED);
            build_cannon(CANNON_X, CANNON_Y + 140., 0., OBJECT_SPAWN_SPEED);

            loop_count = 0;
        }

        solver.update(&mut objects);
        // circle_texture.set_color_mod(0, 0, 0);
        // canvas.copy(&circle_texture, None, Rect::new((WIDTH / 2) as i32 - 500, (HEIGHT / 2) as i32 - 500, 1000, 1000))?;

        for (_, object) in (&objects).iter().enumerate() {
            circle_texture.set_color_mod(object.color.0, object.color.1, object.color.2);
            canvas.copy(&circle_texture, None, Rect::new(object.position_current.x as i32 - CIRCLE_RADIUS as i32, object.position_current.y as i32 - CIRCLE_RADIUS as i32, (CIRCLE_RADIUS * 2.) as u32, (CIRCLE_RADIUS * 2.) as u32))?;
        }
        let text = format!("number of object: {}", objects.len());
        let text2 = format!("frame time: {}ms", delta_time.as_millis());
        let text3 = format!("Physic time: {}ms", solver.timer.elapsed_ms());
        let text4 = format!("draw time: {}ms", delta_time.as_millis().saturating_sub(solver.timer.elapsed_ms() as u128));
        let texture = create_text_texture(&font, &texture_creator, text.as_str())?;
        let texture2 = create_text_texture(&font, &texture_creator, text2.as_str())?;
        let texture3 = create_text_texture(&font, &texture_creator, text3.as_str())?;
        let texture4 = create_text_texture(&font, &texture_creator, text4.as_str())?;
        canvas.copy(&texture, None, Some(Rect::new(0, 0, (text.len() * 7) as u32, 30)))?;
        canvas.copy(&texture2, None, Some(Rect::new(0, 25, (text2.len() * 7) as u32, 30)))?;
        canvas.copy(&texture3, None, Some(Rect::new(0, 50, (text3.len() * 7) as u32, 30)))?;
        canvas.copy(&texture4, None, Some(Rect::new(0, 75, (text4.len() * 7) as u32, 30)))?;
        canvas.present();
    }

    Ok(objects)
}
