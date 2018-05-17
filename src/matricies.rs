use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Index };
use ::vectors;

struct AffineMatrix {
    i1 : f64, j1 : f64, k1 : f64, w1 : f64, 
    i2 : f64, j2 : f64, k2 : f64, w2 : f64, 
    i3 : f64, j3 : f64, k3 : f64, w3 : f64, 
    i4 : f64, j4 : f64, k4 : f64, w4 : f64
}


pub trait Matrix4 {
    fn column_major(&self) -> Vec<f64>;
    fn row_major(&self) -> Vec<f64>;

    fn add(&self, m : Matrix4) -> Matrix4;
    fn sub(&self, m : Matrix4) -> Matrix4;
    fn sub(&self) -> Matrix4;
    fn mult(&self, m : Matrix4) -> Matrix4;
    fn mult(&self, v : Vector3) -> Matrix4;
    fn inverse()
}

impl Index