use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub trait DrawBasicShapes {
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String>;
}

impl DrawBasicShapes for WindowCanvas {
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String> {
        let mut offset_x: i32 = 0;
        let mut offset_y: i32 = radius;
        let mut d: i32 = radius - 1;

        while offset_y >= offset_x {
            self.draw_line(Point::new(x - offset_y, y + offset_x), Point::new(x + offset_y, y + offset_x))?;
            self.draw_line(Point::new(x - offset_x, y + offset_y), Point::new(x + offset_x, y + offset_y))?;
            self.draw_line(Point::new(x - offset_x, y - offset_y), Point::new(x + offset_x, y - offset_y))?;
            self.draw_line(Point::new(x - offset_y, y - offset_x), Point::new(x + offset_y, y - offset_x))?;

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