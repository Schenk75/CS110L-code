use std::ops::Add;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point {x, y}
    }
}

pub trait ComputeNorm {
    fn compute_norm(&self) -> f64 {
        0.0
    }
}

impl ComputeNorm for Option<u32> {}

impl ComputeNorm for Point {
    fn compute_norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl ComputeNorm for Vec<f64> {
    fn compute_norm(&self) -> f64 {
        self.iter().map(|x| {x * x}).sum::<f64>().sqrt()
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y)
    }
}

fn main() {
    let origin_point = Point::new(0.0, 0.0);
    let mut p = origin_point;
    println!("p: {:?} origin_point: {:?}", p, origin_point);
    println!("Are they equal? => {}\n", p == origin_point);

    p.x += 10.0;
    println!("p: {:?} origin_point: {:?}", p, origin_point);
    println!("Are they equal? => {}\n", p == origin_point);

    let some_opt: Option<u32> = Some(110);
    println!("Norm of some_opt: {}\n", some_opt.compute_norm());

    let lil_vec: Vec<f64> = vec![3.0, 4.0];
    println!("Norm of lil_vec: {}\n", lil_vec.compute_norm());
    
    println!("Norm of (3, 4) + origin_point: {}", (origin_point + Point::new(3.0, 4.0)).compute_norm());
}
