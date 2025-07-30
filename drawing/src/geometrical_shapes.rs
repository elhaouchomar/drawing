use crate::{Color, Image};
use rand::Rng;

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

pub trait Drawable {
    fn draw(&self, image: &mut Image);
}

// POINT
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
         let color = get_random_color();
        image.display(self.x, self.y, color);

    }
}

// LINE
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { start: p1, end: p2 }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let _rng = rand::thread_rng();
        Self {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
         let color = get_random_color();
        draw_line(image, &self.start, &self.end, &color);
    }
}

// CERCLE
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(10..50);
        let x = rng.gen_range(radius..(width - radius));
        let y = rng.gen_range(radius..(height - radius));
        Self {
            center: Point { x, y },
            radius,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
         let color = get_random_color();
                 let perimeter = (self.radius.pow(2) as f32) * std::f32::consts::PI;
        let inc = 360.0 / perimeter;
        let mut i: f32 = 0.0;
        while i <= 360.0 {
            let angle = (i as f32) * std::f32::consts::PI / 180.0;
            let x = (self.center.x as f32 + self.radius as f32 * angle.cos()) as i32;
            let y = (self.center.y as f32 + self.radius as f32 * angle.sin()) as i32;
            image.display(x, y, color.clone());


            i += inc;
        }
    }
}

// impl Drawable for Circle {
//     fn draw(&self, image: &mut Image) {
//         let color = self.color();
//         let perimeter = (self.radius.pow(2) as f32) * std::f32::consts::PI;
//         let inc = 360.0 / perimeter;
//         let mut i: f32 = 0.0;
//         while i <= 360.0 {
//             let ang = (i as f32) * std::f32::consts::PI / 180.0;
//             let x = ((self.radius as f32) * ang.cos() + (self.center.x as f32)).round() as i32;
//             let y = ((self.radius as f32) * ang.sin() + (self.center.y as f32)).round() as i32;
//             image.display(x, y, color.clone());
//             i += inc;
//         }
//     }
// }

// RECT
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub top_right: Point,
    pub bottom_left: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let x_min = p1.x.min(p2.x); // 50
        let y_min = p1.y.min(p2.y); // 50
        let mut x_max = p1.x.max(p2.x); // 150
        let y_max = p1.y.max(p2.y); // 150

        if (x_max - x_min).abs() == (y_max - y_min).abs() {
            x_max += 50; //TO MAKE IT RECT
        }

        Self {
            top_left: Point { x: x_min, y: y_min },    // 50 50
            bottom_right: Point { x: x_max, y: y_max }, // 150 150
            top_right: Point { x: x_max, y: y_min },   // 150 50
            bottom_left: Point { x: x_min, y: y_max }, // 50 150
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0..width);
        let y1 = rng.gen_range(0..height);
        let x2 = rng.gen_range(x1..width);
        let y2 = rng.gen_range(y1..height);
        Self::new(&Point::new(x1, y1), &Point::new(x2, y2))
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top = Line::new(self.top_left, self.top_right);
        let right = Line::new(self.top_right, self.bottom_right);
        let bottom = Line::new(self.bottom_right, self.bottom_left);
        let left = Line::new(self.bottom_left, self.top_left);      

        top.draw(image);
        right.draw(image);
        bottom.draw(image);
        left.draw(image);
    }
}


// TRIANGLE
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Self {
            p1: *p1,
            p2: *p2,
            p3: *p3,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let rng = rand::thread_rng();
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        let p3 = Point::random(width, height);
        Self::new(&p1, &p2, &p3)
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = get_random_color();
        draw_line(image, &self.p1, &self.p2, &color);
        draw_line(image, &self.p2, &self.p3, &color);
        draw_line(image, &self.p3, &self.p1, &color);
    }
}

// DRAW LINE
fn draw_line(image: &mut Image, start: &Point, end: &Point, color: &Color) {
    let dx = end.x - start.x;
    let dy = end.y - start.y;
    let steps = dx.abs().max(dy.abs());

    if steps == 0 {
        image.display(start.x, start.y, color.clone());
        return;
    }

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let x = start.x as f32 + t * dx as f32;
        let y = start.y as f32 + t * dy as f32;
        image.display(x.round() as i32, y.round() as i32, color.clone());

    }
}


pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(100..=255);
    let g = rng.gen_range(100..=255);
    let b = rng.gen_range(100..=255);
    Color::rgb(r, g, b)
}