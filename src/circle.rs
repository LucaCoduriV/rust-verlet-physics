use std::f32::consts::PI;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub struct Circle {
    pub pos: (f32, f32),
    pub radius: f32,
    pub color: (u8,u8,u8,u8),
}

impl Circle {
    pub fn draw(&self, canvas: &mut DrawTarget){
        let mut pb = PathBuilder::new();
        pb.arc(self.pos.0, self.pos.1, self.radius, 0., 2. * PI);
        let path = pb.finish();

        canvas.fill(
            &path,
            &Source::Solid(SolidSource::from_unpremultiplied_argb(self.color.0, self.color.1,
                                                                  self.color.2, self.color.3)),
            &DrawOptions::new(),
        );
    }
}