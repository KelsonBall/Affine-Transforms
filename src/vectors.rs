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
    pub fn new(x : f64, y : f64, z : f64) -> KVector3 {
        KVector3 { x: x, y: y, z: z }
    }

    pub fn newi(x : i32, y : i32, z : i32) -> KVector3 {
        KVector3 { x: x as f64, y: y as f64, z: z as f64 }
    }
}

pub trait Vector {
    fn magnitude_squared(self) -> f64;
    fn magnitude(self) -> f64;        
}

pub trait Vector3 {
    fn dot(self, v : KVector3) -> f64;
    fn cross(self, v : KVector3) -> KVector3;
    fn unit(self) -> KVector3;

    fn v_add(self, v : KVector3) -> KVector3;
    fn v_usub(self) -> KVector3;
    fn v_sub(self, v : KVector3) -> KVector3;
    fn scale(self, s : f64) -> KVector3;

    fn zero() -> KVector3;
    fn i_hat() -> KVector3;
    fn j_hat() -> KVector3;
    fn k_hat() -> KVector3;

    fn x(self) -> f64;
    fn y(self) -> f64;
    fn z(self) -> f64;
}

impl Vector for KVector3 {
    fn magnitude_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }    
}

impl Vector3 for KVector3 {
    fn dot(self, v : KVector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }

    fn cross(self, v : KVector3) -> KVector3 {
        KVector3 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    fn unit(self) -> KVector3 {
        self.scale(1.0 / self.magnitude())
    }

    fn v_add(self, v : KVector3) -> KVector3 {
        KVector3 { x : self.x + v.x, y : self.y + v.y, z : self.z + v.z }
    }

    fn v_usub(self) -> KVector3 {
        KVector3 { x : -self.x, y : -self.y, z : -self.z }
    }

    fn v_sub(self, v : KVector3) -> KVector3 {
        KVector3 { x : self.x - v.x, y : self.y - v.y, z : self.z - v.z }
    }

    fn scale(self, s : f64) -> KVector3 {
        KVector3 { x : self.x * s, y : self.y * s, z : self.z * s }
    }

    fn  zero() -> KVector3 { KVector3 { x: 0., y: 0., z: 0. } }

    fn i_hat() -> KVector3 { KVector3 { x: 1., y: 0., z: 0. } }

    fn j_hat() -> KVector3 { KVector3 { x: 0., y: 1., z: 0. } }

    fn k_hat() -> KVector3 { KVector3 { x: 0., y: 0., z: 1. } }

    fn x(self) -> f64 { self.x }
    fn y(self) -> f64 { self.y }
    fn z(self) -> f64 { self.z }
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