use std::cell::RefCell;

use sdl2::{rect::Rect, render::WindowCanvas};
use sdl2::pixels::Color;

type Point = (f32, f32);

#[derive(Debug, Clone)]
pub struct Aabb {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Aabb {
    pub fn contains_point(&self, point: Point) -> bool {
        return point.0 >= self.left()
            && point.0 <= self.right()
            && point.1 >= self.top()
            && point.1 <= self.bottom();
    }
    pub fn contains_aabb(&self, other: &Aabb) -> bool {
        return other.left() >= self.left() && other.right() <= self.right()
        && other.top() >= self.top() && other.bottom() <= self.bottom();
    }

    pub fn intersects(&self, other: &Aabb) -> bool {
        !(self.x > other.x + other.width
            || other.x > self.x + self.width
            || self.y + self.height < other.y
            || other.y + other.height < self.y)
    }

    pub fn center(&self) -> Point {
        ((self.x + self.width) / 2., (self.y + self.height) / 2.)
    }

    pub fn left(&self) -> f32 {
        self.x
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn top(&self) -> f32 {
        self.y
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }
}

#[derive(Debug)]
struct QuadTreeNode {
    boundary: Aabb,
    capacity: usize,
    values: Vec<Aabb>,
    north_east: Option<Box<RefCell<QuadTreeNode>>>,
    north_west: Option<Box<RefCell<QuadTreeNode>>>,
    south_east: Option<Box<RefCell<QuadTreeNode>>>,
    south_west: Option<Box<RefCell<QuadTreeNode>>>,
}

impl QuadTreeNode {
    fn new(boundary: Aabb, capacity: usize) -> Self {
        Self {
            boundary,
            capacity,
            values: Vec::new(),
            north_east: None,
            north_west: None,
            south_east:None,
            south_west:None,
        }
    }

    fn is_leaf(&self) -> bool {
        self.north_west.is_none()
    }

    fn insert(&mut self, value: Aabb) {
        if !self.boundary.contains_aabb(&value) {
            return;
        }

        if self.is_leaf(){
            if self.values.len() < self.capacity {
                self.values.push(value);
                return;
            }else{
                self.divide();
                self.insert(value);
            }
        }else{
            let mut nw = self.north_west.as_mut().unwrap().borrow_mut();
            let mut ne = self.north_east.as_mut().unwrap().borrow_mut();
            let mut sw = self.south_west.as_mut().unwrap().borrow_mut();
            let mut se = self.south_east.as_mut().unwrap().borrow_mut();

            if nw.boundary.contains_aabb(&value){
                nw.insert(value);
            }else if ne.boundary.contains_aabb(&value){
                ne.insert(value);
            } else if sw.boundary.contains_aabb(&value){
                sw.insert(value);
            } else if se.boundary.contains_aabb(&value){
                se.insert(value);
            } else{
                self.values.push(value);
            }
        }
    }

    fn query(&self, range: Aabb, arr: &mut Vec<Aabb>) {
        assert!(self.boundary.contains_aabb(&range));

        for value in self.values.iter() {
            if range.intersects(value) {
                arr.push(value.clone())
            }
        }

        if !self.is_leaf() {
            let nw = self.north_west.as_ref().unwrap().borrow();
            if nw.boundary.contains_aabb(&range) {
                return nw.query(range.clone(), arr);
            }

            let ne = self.north_east.as_ref().unwrap().borrow();
            if ne.boundary.contains_aabb(&range) {
                return ne.query(range.clone(), arr);
            }

            let sw = self.south_west.as_ref().unwrap().borrow();
            if sw.boundary.contains_aabb(&range) {
                return sw.query(range.clone(), arr);
            }

            let se = self.south_east.as_ref().unwrap().borrow();
            if se.boundary.contains_aabb(&range) {
                return se.query(range.clone(), arr);
            }
        }
    }

    fn find_all_intersections(&self, arr: &mut Vec<(Aabb, Aabb)>){
        for i in 0..self.values.len(){
            for j in i+1..self.values.len(){
                if self.values[i].intersects(&self.values[j]){
                    arr.push((self.values[i].clone(), self.values[j].clone()))
                }
            }
        }
        if !self.is_leaf(){
            let ne = self.north_east.as_ref().unwrap().borrow();
            let nw = self.north_west.as_ref().unwrap().borrow();
            let se = self.south_east.as_ref().unwrap().borrow();
            let sw = self.south_west.as_ref().unwrap().borrow();


            for value in self.values.iter(){
                ne.find_intersections_in_descendants(value, arr);
                nw.find_intersections_in_descendants(value, arr);
                se.find_intersections_in_descendants(value, arr);
                sw.find_intersections_in_descendants(value, arr);
            }

            ne.find_all_intersections(arr);
            nw.find_all_intersections(arr);
            se.find_all_intersections(arr);
            sw.find_all_intersections(arr);
        }


    }

    fn find_intersections_in_descendants(&self, value:&Aabb, arr: &mut Vec<(Aabb, Aabb)>){
        for d_value in self.values.iter(){
            if value.intersects(d_value){
                arr.push((value.clone(), d_value.clone()))
            }
        }
        if !self.is_leaf() {
            let ne = self.north_east.as_ref().unwrap().borrow();
            let nw = self.north_west.as_ref().unwrap().borrow();
            let se = self.south_east.as_ref().unwrap().borrow();
            let sw = self.south_west.as_ref().unwrap().borrow();

            ne.find_intersections_in_descendants(value, arr);
            nw.find_intersections_in_descendants(value, arr);
            se.find_intersections_in_descendants(value, arr);
            sw.find_intersections_in_descendants(value, arr);
        }
    }

    fn divide(&mut self) {
        let ne = Aabb {
            x: self.boundary.x + self.boundary.width / 2.,
            y: self.boundary.y,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.north_east = Some(Box::new(RefCell::new(QuadTreeNode::new(ne, self.capacity))));

        let nw = Aabb {
            x: self.boundary.x,
            y: self.boundary.y,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.north_west = Some(Box::new(RefCell::new(QuadTreeNode::new(nw, self.capacity))));

        let se = Aabb {
            x: self.boundary.x + self.boundary.width / 2.,
            y: self.boundary.y + self.boundary.height / 2.,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.south_east = Some(Box::new(RefCell::new(QuadTreeNode::new(se, self.capacity))));

        let sw = Aabb {
            x: self.boundary.x,
            y: self.boundary.y + self.boundary.height / 2.,
            height: self.boundary.height / 2.,
            width: self.boundary.width / 2.,
        };
        self.south_west = Some(Box::new(RefCell::new(QuadTreeNode::new(sw, self.capacity))));

    }

    fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let rect = Rect::new(
            self.boundary.x as i32,
            self.boundary.y as i32,
            self.boundary.width as u32,
            self.boundary.height as u32,
        );
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.draw_rect(rect)?;

        canvas.set_draw_color(Color::RGB(0, 0, 255));
        for p in self.values.iter() {
            let center = p.center();
            canvas.draw_point(
                sdl2::rect::Point::new(center.0 as i32,center.1 as i32)
            )?;
        }

        if let Some(qt) = self.north_west.as_ref() {
            qt.borrow().draw(canvas)?;
            println!("coucou");
        }

        if let Some(qt) = self.north_east.as_ref() {
            qt.borrow().draw(canvas)?;
            println!("coucou");
        }

        if let Some(qt) = self.south_west.as_ref() {
            qt.borrow().draw(canvas)?;
            println!("coucou");
        }

        if let Some(qt) = self.south_east.as_ref() {
            qt.borrow().draw(canvas)?;
            println!("coucou");
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct QuadTree {
    root: QuadTreeNode,

}

impl QuadTree {
    pub fn new(boundary: Aabb, capacity: usize) -> Self {
        Self {
            root: QuadTreeNode::new(boundary, capacity),
        }
    }

    pub fn insert(&mut self, value: Aabb) {
        self.root.insert(value)
    }

    pub fn query(&self, range: Aabb) -> Vec<Aabb> {
        let mut arr = Vec::new();
        self.root.query(range, &mut arr);
        arr
    }

    pub fn find_all_intersection(&self) -> Vec<(Aabb, Aabb)> {
        let mut arr = Vec::new();
        self.root.find_all_intersections(&mut arr);
        arr
    }



    pub fn draw(&self, canvas: &mut WindowCanvas) -> Result<(), String> {
        self.root.draw(canvas)
    }


}

#[cfg(test)]
mod test {
    use std::time::Duration;
    use rand::Rng;

    use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

    use super::*;

    #[test]
    fn quad_tree_test() {
        let boundary = Aabb {
            x: 0.,
            y: 0.,
            width: 500.,
            height: 500.,
        };
        let mut qt = QuadTree::new(boundary.clone(), 2);

        let bb1 = Aabb {
            x:0.,
            y:0.,
            width:500.,
            height:500.,
        };

        let bb2 = Aabb {
            x:0.,
            y:0.,
            width:100.,
            height:100.,
        };

        let bb3 = Aabb {
            x:260.,
            y:260.,
            width:100.,
            height:100.,
        };

        let bb4 = Aabb {
            x:240.,
            y:240.,
            width:100.,
            height:100.,
        };

        qt.insert(bb1);
        qt.insert(bb2);
        qt.insert(bb3);

        println!("{:#?}", qt);

        println!("{:#?}", qt.find_all_intersection())
    }

    const WIDTH: u32 = 500;
    const HEIGHT: u32 = 500;

    #[test]
    fn quad_tree_draw() -> Result<(), String> {
        let boundary = Aabb {
            x: 0.,
            y: 0.,
            width: 500.,
            height: 500.,
        };
        let mut qt = QuadTree::new(boundary.clone(), 2);

        let mut rng = rand::thread_rng();

        // for _ in 0..500 {
        //     qt.insert((rng.gen_range(0..WIDTH) as f32, rng.gen_range(0..HEIGHT) as f32));
        // }

        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window("rust verlet", WIDTH, HEIGHT)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump()?;

        'running: loop {
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
            canvas.present();

            qt.draw(&mut canvas)?;
            canvas.present();
            std::thread::sleep(Duration::from_millis(500));
        }

        Ok(())
    }
}
