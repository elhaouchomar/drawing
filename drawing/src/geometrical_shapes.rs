use rand::Rng;
//creating two traits:
//Drawable ; contains the methods draw and color;
use raster::{Color, Image};
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
pub trait Drawable {
    //draw
    fn draw(&self, image: &mut Image);

    //color
    // fn color(&self, _: &mut Image);
}

pub struct Line {
    pub start: Point,
    pub end: Point,
}

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Line {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0..width);
        let y1 = rng.gen_range(0..height);
        let x2 = rng.gen_range(0..width);
        let y2 = rng.gen_range(0..height);

        Self {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 },
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;

        let steps = dx.abs().max(dy.abs());

        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let x = self.start.x as f32 + t * dx as f32;
            let y = self.start.y as f32 + t * dy as f32;

            let white = Color::rgba(255, 255, 255, 255);
            image.display(x.round() as i32, y.round() as i32, white);
        }
    }
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x: x, y: y }
    }
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Self { x, y }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let red = Color::rgba(0, 0, 255, 255);
        image.display(self.x, self.y, red);
    }
}
//displayable ; contains the method display
