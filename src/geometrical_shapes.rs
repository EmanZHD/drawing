use rand::prelude::*;
use raster::{ Color, Image };

pub trait Drawable {
    fn draw(&self, img: &mut Image);
    fn color() -> Color {
        let mut range = rand::rng();
        let r = range.random_range(0..=255);
        let g = range.random_range(0..=255);
        let b = range.random_range(0..=255);
        Color::rgb(r, g, b)
    }
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// ----------- Point -----------
#[derive(Debug, Clone)]
pub struct Point(i32, i32);

impl Point {
    pub fn new(v1: i32, v2: i32) -> Self {
        Point(v1, v2)
    }
    pub fn random(width: i32, height: i32) -> Point {
        // let mut range = rand::rng();
        let random_p1 = (rand::random::<u32>() % (width as u32)) as i32;
        let random_p2 = (rand::random::<u32>() % (height as u32)) as i32;
        // println!("{:?} {:?}", random_p1, random_p2);
        Point(random_p1, random_p2)
    }
}

impl Drawable for Point {
    fn draw(&self, img: &mut Image) {
        img.display(self.0, self.1, Point::color())
    }
}

// ----------- Line -----------
#[derive(Debug, Clone)]
pub struct Line(Point, Point);

impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Line(p1.clone(), p2.clone())
    }
    pub fn random(width: i32, height: i32) -> Line {
        Line(Point::random(width, height), Point::random(width, height))
    }
}

impl Drawable for Line {
    fn draw(&self, img: &mut Image) {
        drow_line(self.clone(), Point::color(), img)
    }
}

fn drow_line(line: Line, color: Color, img: &mut Image) {
    let x0 = line.0.0 as f64;
    let x1 = line.1.0 as f64;
    let y0 = line.0.1 as f64;
    let y1 = line.1.1 as f64;
    let dx = x1 - x0;
    let dy = y1 - y0;
    let step = dx.abs().max(dy.abs());

    let x_incr = dx / step;
    let y_incr = dy / step;

    let mut x = x0;
    let mut y = y0;

    for _ in 0..step as i32 {
        img.display(x.round() as i32, y.round() as i32, color.clone());
        x += x_incr;
        y += y_incr;
    }
}

// ----------- Triangle -----------
#[derive(Debug, Clone)]
pub struct Triangle(Point, Point, Point, Color);

impl Triangle {
    pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
        Triangle(p1.clone(), p2.clone(), p3.clone(), Color::rgb(255, 255, 255))
    }
}

impl Drawable for Triangle {
    fn draw(&self, img: &mut Image) {
        let color = Point::color();
        drow_line(Line::new(&self.0, &self.1), color.clone(), img);
        drow_line(Line::new(&self.1, &self.2), color.clone(), img);
        drow_line(Line::new(&self.2, &self.0), color.clone(), img);
    }
}

// ----------- Rectangle -----------
#[derive(Debug, Clone)]
pub struct Rectangle(Point, Point);

impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        Self(p1.clone(), p2.clone())
    }
}

impl Drawable for Rectangle {
    fn draw(&self, img: &mut Image) {
        println!("for rectangle ---> {:?}", self);
        let Point(x1, y1) = self.0;
        let Point(x2, y2) = self.1;

        let top_right = Point::new(x2, y1);
        let bottom_left = Point::new(x1, y2);

        let color = Point::color();
        drow_line(Line::new(&self.0, &top_right), color.clone(), img);
        drow_line(Line::new(&top_right, &self.1), color.clone(), img);
        drow_line(Line::new(&self.1, &bottom_left), color.clone(), img);
        drow_line(Line::new(&bottom_left, &self.0), color.clone(), img);
    }
}

// ----------- Circle -----------
#[derive(Debug, Clone)]
pub struct Circle {
    center: Point,
    radius: i32,
}
impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Circle {
            center: center.clone(),
            radius,
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut range = rand::rng();

        let center = Point::random(width, height);
        let max_radius = std::cmp::min(width, height);
        let radius = range.random_range(5..max_radius);

        Circle { center, radius }
    }

    // Bresenham's circle algorithm
    fn draw_circle_pixels(&self) -> Vec<(i32, i32)> {
        let mut pixels = Vec::new();
        let cx = self.center.0;
        let cy = self.center.1;
        let r = self.radius;

        let mut x = r;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            for &(dx, dy) in [
                (x, y),
                (y, x),
                (-y, x),
                (-x, y),
                (-x, -y),
                (-y, -x),
                (y, -x),
                (x, -y),
            ].iter() {
                pixels.push((cx + dx, cy + dy));
            }
            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            } else {
                x -= 1;
                err += 2 * (y - x + 1);
            }
        }
        pixels
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = Circle::color();
        let pixels = self.draw_circle_pixels();

        for (x, y) in pixels {
            image.display(x, y, color.clone());
        }
    }
}
