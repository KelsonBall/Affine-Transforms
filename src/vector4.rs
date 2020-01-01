use std::ops::{ Add, Sub, Mul, Neg };

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector4
{
    x : f32,
    y : f32,
    z : f32,
    w : f32,
}

pub trait Vec4<T>
{
    fn magnitude(&self) -> f32;
    fn magnitude_squared(&self) -> f32;
    fn dot(&self, v: T) -> f32;
    fn scale(&self, s : f32) -> T;
    fn v3_cross(&self, v: T) -> T;
    fn unit(&self) -> T;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
    fn w(&self) -> f32;

    fn zero() -> T;
    fn identity() -> T;
    fn i_hat() -> T; 
    fn j_hat() -> T; 
    fn k_hat() -> T; 
    fn w_hat() -> T; 
    fn with_x(&self, x : f32) -> T;
    fn with_y(&self, y : f32) -> T;
    fn with_z(&self, z : f32) -> T;
    fn with_w(&self, w : f32) -> T;
}

impl Vector4
{
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 { Vector4 { x: x, y: y, z: z, w: w } }
    pub fn newi(x: i32, y: i32, z: i32, w: i32) -> Vector4 { Vector4 { x: x as f32, y: y as f32, z: z as f32, w: w as f32 } }
}

impl Vec4<Vector4> for Vector4
{
    fn magnitude_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    fn magnitude(&self) -> f32 {
        self.magnitude_squared().sqrt()
    }

    fn dot(&self, v: Vector4) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w
    }

    fn scale(&self, s : f32) -> Vector4 
    {
        Vector4 { x: self.x * s, y : self.y * s, z: self.z * s, w: self.w * s }
    }

    fn v3_cross(&self, v : Vector4) -> Vector4
    {
        Vector4 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x,
            w: self.w
        }
    }

    fn unit(&self) -> Vector4 { self.scale(1.0 / self.magnitude()) }

    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }
    fn w(&self) -> f32 { self.w }
    
    fn zero() -> Vector4 { Vector4 { x: 0., y: 0., z: 0., w: 0. } }
    fn identity() -> Vector4 { Vector4 { x: 1., y: 1., z: 1., w: 1. } }
    fn i_hat() -> Vector4 { Vector4 { x: 1.0, y: 0.0, z: 0.0, w: 0.0 } }
    fn j_hat() -> Vector4 { Vector4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 } }
    fn k_hat() -> Vector4 { Vector4 { x: 0.0, y: 0.0, z: 1.0, w: 0.0 } }
    fn w_hat() -> Vector4 { Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 } }

    fn with_x(&self, x : f32) -> Vector4 { Vector4 { x: x, y: self.y, z: self.z, w: self.w } }
    fn with_y(&self, y : f32) -> Vector4 { Vector4 { x: self.x, y: y, z: self.z, w: self.w } }
    fn with_z(&self, z : f32) -> Vector4 { Vector4 { x: self.x, y: self.y, z: z, w: self.w } }
    fn with_w(&self, w : f32) -> Vector4 { Vector4 { x: self.x, y: self.y, z: self.z, w: w } }
}

impl Add for Vector4
{
    type Output = Vector4;
    fn add(self, v : Vector4)  -> Vector4 {
        Vector4::new(self.x + v.x, self.y + v.y, self.z + v.z, self.w + v.w)
    }
}

impl Sub for Vector4
{
    type Output = Vector4;
    fn sub(self, v : Vector4) -> Vector4 {
        Vector4::new(self.x - v.x, self.y - v.y, self.z - v.z, self.w - v.w)
    }
}

impl Mul<f32> for Vector4
{
    type Output = Vector4;
    fn mul(self, s : f32) -> Vector4
    {
        self.scale(s)
    }
}

impl Mul<Vector4> for f32
{
    type Output = Vector4;
    fn mul(self, v : Vector4)  -> Vector4
    {
        v.scale(self)
    }
}

impl Neg for Vector4
{
    type Output = Vector4;
    fn neg(self) -> Vector4
    {
        Vector4::new(-self.x, -self.y, -self.z, -self.w)
    }
}


#[cfg(test)]
mod tests {

    use ::vector4::{Vec4, Vector4};

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = Vector4::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn v3_magnitude_squared() {
        let triangle = Vector4::newi(3, 4, 0, 0);
        let mag_squared = triangle.magnitude_squared();
        assert_eq!(25., mag_squared);
    }

    #[test]
    fn v3_magnitude() {
        let triangle = Vector4::newi(3, 4, 0, 0);
        let mag = triangle.magnitude();
        assert_eq!(5., mag);
    }

    #[test]
    fn cross_product() {
        let product = Vector4::i_hat().v3_cross(Vector4::j_hat());
        assert_eq!(Vector4::k_hat(), product);

    }

    #[test]
    fn cross_product_normal() {
        let a = Vector4::newi(1, 2, 3, 0);
        let b = Vector4::newi(4, 5, 6, 0);
        let normal = a.v3_cross(b);
        assert_eq!(normal, Vector4::newi(-3, 6, -3, 0));
    }

    #[test]
    fn unit_vector() {
        let i_unit = Vector4::newi(2, 0, 0, 0).unit();
        assert_eq!(i_unit, Vector4::i_hat());
    }

    #[test]
    fn unit_triangle() {
        let triangle = Vector4::newi(3, 4, 0, 0);
        let unit = triangle.unit();
        let dif = (unit - Vector4::new(3.0/5.0, 4.0/5.0,0.0, 0.0)).magnitude_squared();
        assert_eq!(dif < 0.0000001, true);
    }

    #[test]
    fn vector_scale() {
        let scale = Vector4::i_hat() * 3.;
        assert_eq!(scale, Vector4::newi(3, 0, 0, 0));
    }

    #[test]
    fn triangle_scale() {
        let triangle = Vector4::newi(3, 4, 0, 0);
        let big = triangle * 2.0;
        assert_eq!(triangle.x() * 2., big.x());
        assert_eq!(triangle.y() * 2., big.y());
        assert_eq!(triangle.z() * 2., big.z());
    }

    #[test]
    fn add_one_plus_one() {
        assert_eq!(Vector4::i_hat() + Vector4::i_hat(), Vector4::newi(2, 0, 0, 0) );
    }

    #[test]
    fn triangle_add() {
        let triangle = Vector4::newi(3, 4, 0, 0);
        let composed =
              Vector4::i_hat()
            + Vector4::i_hat()
            + Vector4::i_hat()
            + Vector4::j_hat()
            + Vector4::j_hat()
            + Vector4::j_hat()
            + Vector4::j_hat();
        assert_eq!(triangle, composed);
    }

    #[test]
    fn sub_one_minus_one() {
        assert_eq!(Vector4::i_hat() - Vector4::i_hat(), Vector4::zero());
    }

    #[test]
    fn unary_sub() {
        assert_eq!(-Vector4::i_hat(), Vector4::newi(-1, 0, 0, 0));
    }
}