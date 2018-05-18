use std::ops::{ Add, Sub, Mul, Neg };

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct KVector3{
    x : f64,
    y : f64,
    z : f64,
}

impl KVector3 {

    pub fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }  

    pub fn new(x : f64, y : f64, z : f64) -> KVector3 {
        KVector3 { x: x, y: y, z: z }
    }

    pub fn newi(x : i32, y : i32, z : i32) -> KVector3 {
        KVector3 { x: x as f64, y: y as f64, z: z as f64 }
    }

    pub fn dot(self, v : KVector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }

    pub fn cross(self, v : KVector3) -> KVector3 {
        KVector3 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    pub fn unit(self) -> KVector3 {
        self.scale(1.0 / self.magnitude())
    }

    pub fn v_add(self, v : KVector3) -> KVector3 {
        KVector3 { x : self.x + v.x, y : self.y + v.y, z : self.z + v.z }
    }

    pub fn v_usub(self) -> KVector3 {
        KVector3 { x : -self.x, y : -self.y, z : -self.z }
    }

    pub fn v_sub(self, v : KVector3) -> KVector3 {
        KVector3 { x : self.x - v.x, y : self.y - v.y, z : self.z - v.z }
    }

    pub fn scale(self, s : f64) -> KVector3 {
        KVector3 { x : self.x * s, y : self.y * s, z : self.z * s }
    }

    pub fn x(self) -> f64 { self.x }
    pub fn y(self) -> f64 { self.y }
    pub fn z(self) -> f64 { self.z }

    pub fn  zero() -> KVector3 { KVector3 { x: 0., y: 0., z: 0. } }
    pub fn identity() -> KVector3 { KVector3 { x: 1., y: 1., z: 1. } }
    pub fn i_hat() -> KVector3 { KVector3 { x: 1., y: 0., z: 0. } }
    pub fn j_hat() -> KVector3 { KVector3 { x: 0., y: 1., z: 0. } }
    pub fn k_hat() -> KVector3 { KVector3 { x: 0., y: 0., z: 1. } }

    pub fn with_x(self, x : f64) -> KVector3 { KVector3 { x: x, y: self.y, z: self.z } }
    pub fn with_y(self, y : f64) -> KVector3 { KVector3 { x: self.x, y: y, z: self.z } }
    pub fn with_z(self, z : f64) -> KVector3 { KVector3 { x: self.x, y: self.y, z: z } }
}

impl Add for KVector3 {
    type Output = KVector3;
    fn add(self, v : KVector3)  -> KVector3 {
        self.v_add(v)
    }
}

impl Sub for KVector3 {
    type Output = KVector3;
    fn sub(self, v : KVector3) -> KVector3 {
        self.v_sub(v)
    }
}

impl Mul<f64> for KVector3 {
    type Output = KVector3;
    fn mul(self, s : f64) -> KVector3 {
        self.scale(s)
    }
}

impl Mul<KVector3> for f64 {
    type Output = KVector3;
    fn mul(self, v : KVector3)  -> KVector3 {
        v.scale(self)
    }
}

impl Neg for KVector3 {
    type Output = KVector3;
    fn neg(self) -> KVector3 {
        self.v_usub()
    }
}