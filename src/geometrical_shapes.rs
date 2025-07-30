use crate::{Color, Image};
use rand::Rng;
use std::collections::HashSet;

// Trait for anything that can be displayed (draws a pixel at x, y with a color)
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Trait for drawable shapes
pub trait Drawable {
    fn draw(&self, image: &mut Image);

    fn color(&self) -> Color;
}

// Represents a point in 2D space
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // Generates a random point within image bounds
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
        let color = self.color();
        image.display(self.x, self.y, color);
    }
    fn color(&self) -> Color {
        get_random_color()
    }
}

// A line between two points
pub struct Line {
    pub start: Point,
    pub end: Point,
}

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self {
            start: *p1,
            end: *p2,
        }
    }

    // Random line from two random points
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
        let color = self.color();
        draw_line(image, &self.start, &self.end, &color);
    }
    fn color(&self) -> Color {
        get_random_color()
    }
}

// A circle with a center and radius
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

impl Circle {
    pub fn new(center: &Point, radius: i32) -> Self {
        Self {
            center: *center,
            radius,
        }
    }

    // Random circle positioned fully inside image
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let radius = rng.gen_range(10..300);
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        Self {
            center: Point { x, y },
            radius,
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let circumference = 2.0 * std::f32::consts::PI * self.radius as f32;
        let inc = 360.0 / circumference;
        let mut i: f32 = 0.0;

         let mut visited = HashSet::new();

        while i <= 360.0 {
            let angle = i.to_radians();
            let x = (self.center.x as f32 + self.radius as f32 * angle.cos()).round() as i32;
            let y = (self.center.y as f32 + self.radius as f32 * angle.sin()).round() as i32;

            //  only draw if this (x, y) wasn't drawn before
             if visited.insert((x, y)) {
                image.display(x, y, color.clone());
             }

            i += inc;
        }
    }
    fn color(&self) -> Color {
        get_random_color()
    }
}

// Rectangle represented by 4 corner points
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
    pub top_right: Point,
    pub bottom_left: Point,
}

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let x_min = p1.x.min(p2.x);
        let y_min = p1.y.min(p2.y);
        let x_max = p1.x.max(p2.x);
        let y_max = p1.y.max(p2.y);

        Self {
            top_left: Point { x: x_min, y: y_min },
            bottom_right: Point { x: x_max, y: y_max },
            top_right: Point { x: x_max, y: y_min },
            bottom_left: Point { x: x_min, y: y_max },
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        // Draw rectangle edges using four lines
        let color = self.color();
        draw_line(image, &self.top_left, &self.top_right, &color);
        draw_line(image, &self.top_right, &self.bottom_right, &color);
        draw_line(image, &self.bottom_right, &self.bottom_left, &color);
        draw_line(image, &self.bottom_left, &self.top_left, &color);
    }
    fn color(&self) -> Color {
        get_random_color()
    }
}

// A triangle with three points
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
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        // Draw edges between the 3 triangle vertices
        draw_line(image, &self.p1, &self.p2, &color);
        draw_line(image, &self.p2, &self.p3, &color);
        draw_line(image, &self.p3, &self.p1, &color);
    }
    fn color(&self) -> Color {
        get_random_color()
    }
}

// Draws a line between two points using linear interpolation
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

// Generates a random bright RGB color
pub fn get_random_color() -> Color {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(100..=255);
    let g = rng.gen_range(100..=255);
    let b = rng.gen_range(100..=255);
    Color::rgb(r, g, b)
}
