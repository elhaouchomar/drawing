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
     fn color(&self)-> Color;
}
#[derive(Debug,Clone)]

pub struct Line {
    pub start: Point,
    pub end: Point,
}

#[derive(Debug,Clone,Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}


impl Line {
    pub fn new(p1: Point, p2:Point)-> Self{
        Self{
            start : p1, end: p2
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x1 = rng.gen_range(0..width);
        let y1 = rng.gen_range(0..height);
        let x2 = rng.gen_range(0..width);
        let y2 = rng.gen_range(0..height);

        Self {
            start: Point { x: x1, y: y1 },
            end: Point { x: x2, y: y2 }
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

            
            image.display(x.round() as i32, y.round() as i32, self.color());
        }
    }
    fn color(&self)-> Color{
       
         Color::rgba(0, 0, 255, 255)
       
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
        
        image.display(self.x, self.y, self.color());
    }
    fn color(&self) -> Color{
         Color::rgba(0, 0, 255, 255)
    }
}
// Triangle

pub struct Triangle{
    pub x : Point,
    pub y : Point,
    pub z : Point,
}

impl Triangle{
   pub fn new (p1:Point,p2: Point, p3:Point)-> Self{
        Self{
            x:p1, y : p2, z: p3
        }
    }
}
impl Drawable for Triangle{
    fn draw(&self,image: &mut Image){
      let l1 = Line::new(self.x,self.y);
      let l2 = Line::new(self.y, self.z);
      let l3 = Line::new(self.z, self.x);
      l1.draw(image);
      l2.draw(image);
      l3.draw(image);
    }
    fn color(&self)->Color{
        Color::rgba(0, 0, 255, 255)
    }
}

// rectangle
#[derive(Debug)]
pub struct Rectangle{
    p1 : Point,
    p2 :Point,
    p3: Point,
    p4: Point,
}
impl Rectangle{
    pub fn new(a : Point , b: Point)->Self{
        Self{
            p1:a,
            p2:b,
            p3:Point{x:a.x,y:b.y},
            p4:Point{x:b.x,y:a.y},
        }
    }
}
impl Drawable for Rectangle{
    fn draw(&self,image:&mut Image){
        let l1 = Line::new(self.p1,self.p3);
        let l2 = Line::new(self.p3,self.p2);
        let l3 = Line::new(self.p2,self.p4);
        let l4 = Line::new(self.p4,self.p1);
        l1.draw(image);
        l2.draw(image);
        l3.draw(image);
        l4.draw(image);

    }
        fn color(&self)->Color{
        Color::rgba(0, 0, 255, 255)
    }
}