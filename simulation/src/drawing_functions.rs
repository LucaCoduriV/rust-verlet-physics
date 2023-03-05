use sdl2::rect::Point;
use sdl2::render::WindowCanvas;

pub trait DrawBasicShapes {
    fn fill_circle(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String>;
    fn fill_circlev2(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String>;
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

    fn fill_circlev2(&mut self, x: i32, y: i32, radius: i32) -> Result<(), String> {
        let pixels = pixelize_circle(Point::new(x,y), radius);
        self.draw_points(pixels.as_slice())
    }
}

fn round_up_to_multiple_of_eight(v: i32) -> i32
{
    return (v + (8 - 1)) & -8;
}

pub fn pixelize_circle(center: Point, radius: i32) -> Vec<Point>
{
    let mut points: Vec<Point>;

    // 35 / 49 is a slightly biased approximation of 1/sqrt(2)
    let arrSize: usize = round_up_to_multiple_of_eight(radius * 8 * 35 / 49) as usize;
    points = Vec::with_capacity(arrSize);

    let diameter = radius * 2;

    let mut x = radius - 1;
    let mut y = 0;
    let mut tx = 1;
    let mut ty = 1;
    let mut error = tx - diameter;

    while x >= y {
// Each of the following renders an octant of the circle
        points.push(Point::new(center.x + x, center.y - y));
        points.push(Point::new(center.x + x, center.y + y));
        points.push(Point::new(center.x - x, center.y - y));
        points.push(Point::new(center.x - x, center.y + y));
        points.push(Point::new(center.x + y, center.y - x));
        points.push(Point::new(center.x + y, center.y + x));
        points.push(Point::new(center.x - y, center.y - x));
        points.push(Point::new(center.x - y, center.y + x));

        if error <= 0 {
            y += 1;
            error += ty;
            ty += 2;
        }

        if error > 0 {
            x -= 1;
            tx += 2;
            error += tx - diameter;
        }
    }

    return points;
}