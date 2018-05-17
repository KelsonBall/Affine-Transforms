use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Index };
use ::vectors;

pub enum Cell {
    I1, I2, I3, I4,
    J1, J2, J3, J4,
    K1, K2, K3, K4,
    W1, W2, W3, W4,
    Row(i8),
    Column(i8),
}

pub enum Primitives {
    Zero,
    Identity,    
    Translation(f64,f64,f64),
    RotationX(f64),
    RotationY(f64),
    RotationZ(f64),
    Scale(f64,f64,f64),
    UniformScale(f64),
}

pub struct AffineMatrix {
    i1 : f64, j1 : f64, k1 : f64, w1 : f64, 
    i2 : f64, j2 : f64, k2 : f64, w2 : f64, 
    i3 : f64, j3 : f64, k3 : f64, w3 : f64, 
    i4 : f64, j4 : f64, k4 : f64, w4 : f64
}

impl AffineMatrix {
    pub fn from_row_major(array : Vec<f64>) -> AffineMatrix {
        AffineMatrix {
            i1 : array[0], j1 : array[1], k1 : array[2], w1 : array[3],
            i2 : array[4], j2 : array[5], k2 : array[6], w2 : array[7],
            i3 : array[8], j3 : array[9], k3 : array[10],w3 : array[11],
            i4 : array[12],j4 : array[13],k4 : array[14],w4 : array[15],
        }
    }

    pub fn from_column_major(array : Vec<f64>) -> AffineMatrix {
        AffineMatrix {
            i1 : array[0], j1 : array[4], k1 : array[8], w1 : array[12],
            i2 : array[1], j2 : array[5], k2 : array[9], w2 : array[13],
            i3 : array[2], j3 : array[6], k3 : array[10],w3 : array[14],
            i4 : array[3], j4 : array[7], k4 : array[11],w4 : array[15],
        }
    }

    pub fn new(primitive : Primitives) -> AffineMatrix {
        match primitive {
            Primitives::Zero => AffineMatrix::from_row_major(vec![0.0;16]),
            Primitives::Identity => AffineMatrix {
                i1 : 1., j1 : 0., k1 : 0., w1 : 0.,
                i2 : 0., j2 : 1., k2 : 0., w2 : 0.,
                i3 : 0., j3 : 0., k3 : 1., w3 : 0.,
                i4 : 0., j4 : 0., k4 : 0., w4 : 1.,
            },
            Primitives::Translation(x,y,z) => panic!("Not implemented"),
            Primitives::RotationX(theta) => panic!("Not implemented"),
            Primitives::RotationY(theta) => panic!("Not implemented"),
            Primitives::RotationZ(theta) => panic!("Not implemented"),
            Primitives::Scale(x,y,z) => panic!("Not implemented"),
            Primitives::UniformScale(s) => panic!("Not implemented"),
        }
    }
}
// pub trait Matrix4 {
//     fn column_major(&self) -> Vec<f64>;
//     fn row_major(&self) -> Vec<f64>;

//     fn add(&self, m : Matrix4) -> Matrix4;
//     fn sub(&self, m : Matrix4) -> Matrix4;
//     fn sub(&self) -> Matrix4;
//     fn mult(&self, m : Matrix4) -> Matrix4;
//     fn mult(&self, v : Vector3) -> Matrix4;
//     fn inverse()
// }

impl Index<Cell> for AffineMatrix {
    type Output = f64;
    fn index(&self, c : Cell) -> &f64 {
        match c {
            Cell::I1 => &self.i1, Cell::I2 => &self.i2, Cell::I3 => &self.i3, Cell::I4 => &self.i4, 
            Cell::J1 => &self.j1, Cell::J2 => &self.j2, Cell::J3 => &self.j3, Cell::J4 => &self.j4, 
            Cell::K1 => &self.k1, Cell::K2 => &self.k2, Cell::K3 => &self.k3, Cell::K4 => &self.k4, 
            Cell::W1 => &self.w1, Cell::W2 => &self.w2, Cell::W3 => &self.w3, Cell::W4 => &self.w4,
            Cell::Column(0) => &self.i1, Cell::Column(1) => &self.i2, Cell::Column(2) => &self.i3, Cell::Column(3) => &self.i4, 
            Cell::Column(4) => &self.j1, Cell::Column(5) => &self.j2, Cell::Column(6) => &self.j3, Cell::Column(7) => &self.j4, 
            Cell::Column(8) => &self.k1, Cell::Column(9) => &self.k2, Cell::Column(10) =>&self.k3, Cell::Column(11) =>&self.k4, 
            Cell::Column(12) =>&self.w1, Cell::Column(13) =>&self.w2, Cell::Column(14) =>&self.w3, Cell::Column(15) =>&self.w4, 
            Cell::Column(_) => panic!("Matrix Index out of bounds"),
            Cell::Row(0) => &self.i1, Cell::Row(4) => &self.i2, Cell::Row(8) => &self.i3, Cell::Row(12) =>&self.i4, 
            Cell::Row(1) => &self.j1, Cell::Row(5) => &self.j2, Cell::Row(9) => &self.j3, Cell::Row(13) =>&self.j4, 
            Cell::Row(2) => &self.k1, Cell::Row(6) => &self.k2, Cell::Row(10) =>&self.k3, Cell::Row(14) =>&self.k4, 
            Cell::Row(3) => &self.w1, Cell::Row(7) => &self.w2, Cell::Row(11) =>&self.w3, Cell::Row(15) =>&self.w4, 
            Cell::Row(_) => panic!("Matrix Index out of bounds"), 
        }
    }
}