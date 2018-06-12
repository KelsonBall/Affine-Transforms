use std::ops::{ Add, Sub, Mul, Neg };

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector{
    x : f64,
    y : f64,
    z : f64,
}

impl Vector {

    pub fn magnitude_squared(&self) -> f64 {        
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }  

    pub fn new(x : f64, y : f64, z : f64) -> Vector {
        Vector { x: x, y: y, z: z }
    }

    pub fn newi(x : i32, y : i32, z : i32) -> Vector {
        Vector { x: x as f64, y: y as f64, z: z as f64 }
    }

    pub fn dot(&self, v : Vector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z        
    }

    pub fn cross(&self, v : Vector) -> Vector {
        Vector {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    pub fn unit(&self) -> Vector {
        self.scale(1.0 / self.magnitude())
    }

    pub fn v_add(&self, v : Vector) -> Vector {
        Vector { x : self.x + v.x, y : self.y + v.y, z : self.z + v.z }
    }

    pub fn v_usub(&self) -> Vector {
        Vector { x : -self.x, y : -self.y, z : -self.z }
    }

    pub fn v_sub(&self, v : Vector) -> Vector {
        Vector { x : self.x - v.x, y : self.y - v.y, z : self.z - v.z }
    }

    pub fn scale(&self, s : f64) -> Vector {
        Vector { x : self.x * s, y : self.y * s, z : self.z * s }
    }

    pub fn round(&self) -> Vector {
        Vector { x : self.x.round(), y : self.y.round(), z : self.z.round() }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn  zero() -> Vector { Vector { x: 0., y: 0., z: 0. } }
    pub fn identity() -> Vector { Vector { x: 1., y: 1., z: 1. } }
    pub fn i_hat() -> Vector { Vector { x: 1., y: 0., z: 0. } }
    pub fn j_hat() -> Vector { Vector { x: 0., y: 1., z: 0. } }
    pub fn k_hat() -> Vector { Vector { x: 0., y: 0., z: 1. } }

    pub fn with_x(&self, x : f64) -> Vector { Vector { x: x, y: self.y, z: self.z } }
    pub fn with_y(&self, y : f64) -> Vector { Vector { x: self.x, y: y, z: self.z } }
    pub fn with_z(&self, z : f64) -> Vector { Vector { x: self.x, y: self.y, z: z } }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, v : Vector)  -> Vector {
        self.v_add(v)
    }
}

impl Sub for Vector {
    type Output = Vector;
    fn sub(self, v : Vector) -> Vector {
        self.v_sub(v)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, s : f64) -> Vector {
        self.scale(s)
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, v : Vector)  -> Vector {
        v.scale(self)
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Vector {
        self.v_usub()
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct AffineVector {
    x : f64,
    y : f64,
    z : f64,
    w : f64,
}

impl AffineVector {

    pub fn magnitude_squared(&self) -> f64 {        
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }  

    pub fn dot(&self, v: AffineVector) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w            
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> AffineVector { AffineVector { x: x, y: y, z: z, w: w } }

    pub fn x(self) -> f64 { self.x }
    pub fn y(self) -> f64 { self.y }
    pub fn z(self) -> f64 { self.z }
    pub fn w(self) -> f64 { self.w }
}

impl Add for AffineVector {
    type Output = AffineVector;
    fn add(self, v : AffineVector)  -> AffineVector {
        AffineVector::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w)
    }
}

impl Sub for AffineVector {
    type Output = AffineVector;
    fn sub(self, v : AffineVector) -> AffineVector {
        AffineVector::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w)
    }
}

impl Neg for AffineVector {
    type Output = AffineVector;
    fn neg(self) -> AffineVector {
        AffineVector::new(-self.x, -self.y, -self.z, -self.w)
    }
}