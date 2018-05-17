use std::fmt;

pub struct KVector3{
    pub x : f64,
    pub y : f64,
    pub z : f64,
}

pub trait Vector {
    fn magnitude_squared(&self) -> f64;
    fn magnitude(&self) -> f64;        
}

pub trait Vector3 {
    fn dot(&self, v : KVector3) -> f64;
    fn cross(&self, v : KVector3) -> KVector3;
    fn unit(&self) -> KVector3;

    fn add(&self, v : KVector3) -> KVector3;
    fn sub(&self, v : KVector3) -> KVector3;
    fn scale(&self, s : f64) -> KVector3;

    fn zero() -> KVector3;
    fn i_hat() -> KVector3;
    fn j_hat() -> KVector3;
    fn k_hat() -> KVector3;
}

impl Vector for KVector3 {
    fn magnitude_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }    
}

impl Vector3 for KVector3 {
    fn dot(&self, v : KVector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z + v.z
    }

    fn cross(&self, v : KVector3) -> KVector3 {
        KVector3 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    fn unit(&self) -> KVector3 {
        self.scale(1.0 / self.magnitude())
    }

    fn add(&self, v : KVector3) -> KVector3 {
        KVector3 {
            x : self.x + v.x,
            y : self.y + v.y,
            z : self.z + v.z
        }
    }

    fn sub(&self, v : KVector3) -> KVector3 {
        KVector3 {
            x : self.x - v.x,
            y : self.y - v.y,
            z : self.z - v.z
        }
    }

    fn scale(&self, s : f64) -> KVector3 {
        KVector3 {
            x : self.x * s,
            y : self.y * s,
            z : self.z * s
        }
    }

    fn zero() -> KVector3 { KVector3 { x: 0., y: 0., z: 0. } }

    fn i_hat() -> KVector3 { KVector3 { x: 1., y: 0., z: 0. } }

    fn j_hat() -> KVector3 { KVector3 { x: 0., y: 1., z: 0. } }

    fn k_hat() -> KVector3 { KVector3 { x: 0., y: 0., z: 1. } }
}


impl PartialEq<KVector3> for KVector3 {
    fn eq(&self, other : &KVector3) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl fmt::Debug for KVector3 {
    fn fmt(&self, f : &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{},{},{}>", self.x, self.y, self.z)
    }
}
