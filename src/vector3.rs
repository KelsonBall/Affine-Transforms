use std::ops::{ Add, Sub, Mul, Neg };

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector3
{
    x : f32,
    y : f32,
    z : f32
}

impl Vector3
{
    pub fn new(x : f32, y : f32, z : f32) -> Vector3
    {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn newi(x : i32, y : i32, z : i32) -> Vector3
    {
        Vector3 { x: x as f32, y: y as f32, z: z as f32 }
    }
}

pub trait Vec3<T>
{
    fn magnitude_squared(&self) -> f32;
    fn magnitude(&self) -> f32;
    fn dot(&self, v : T) -> f32;
    fn cross(&self, v : T) -> T;
    fn unit(&self) -> T;

    fn v_add(&self, v : T) -> T;
    fn v_usub(&self) -> T;
    fn v_sub(&self, v : T) -> T;
    fn scale(&self, s : f32) -> T;
    fn round(&self) -> T;
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;

    fn zero() -> T;
    fn identity() -> T;
    fn i_hat() -> T;
    fn j_hat() -> T;
    fn k_hat() -> T;

    fn with_x(&self, x : f32) -> T;
    fn with_y(&self, y : f32) -> T;
    fn with_z(&self, z : f32) -> T;
}

impl Vec3<Vector3> for Vector3
{
    fn magnitude_squared(&self) -> f32
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn magnitude(&self) -> f32
    {
        self.magnitude_squared().sqrt()
    }

    fn dot(&self, v : Vector3) -> f32
    {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    fn cross(&self, v : Vector3) -> Vector3
    {
        Vector3 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    fn unit(&self) -> Vector3
    {
        self.scale(1.0 / self.magnitude())
    }

    fn v_add(&self, v : Vector3) -> Vector3
    {
        Vector3 { x : self.x + v.x, y : self.y + v.y, z : self.z + v.z }
    }

    fn v_usub(&self) -> Vector3
    {
        Vector3 { x : -self.x, y : -self.y, z : -self.z }
    }

    fn v_sub(&self, v : Vector3) -> Vector3
    {
        Vector3 { x : self.x - v.x, y : self.y - v.y, z : self.z - v.z }
    }

    fn scale(&self, s : f32) -> Vector3
    {
        Vector3 { x : self.x * s, y : self.y * s, z : self.z * s }
    }

    fn round(&self) -> Vector3
    {
        Vector3 { x : self.x.round(), y : self.y.round(), z : self.z.round() }
    }

    fn x(&self) -> f32 { self.x }
    fn y(&self) -> f32 { self.y }
    fn z(&self) -> f32 { self.z }

    fn zero() -> Vector3 { Vector3 { x: 0., y: 0., z: 0. } }
    fn identity() -> Vector3 { Vector3 { x: 1., y: 1., z: 1. } }
    fn i_hat() -> Vector3 { Vector3 { x: 1., y: 0., z: 0. } }
    fn j_hat() -> Vector3 { Vector3 { x: 0., y: 1., z: 0. } }
    fn k_hat() -> Vector3 { Vector3 { x: 0., y: 0., z: 1. } }

    fn with_x(&self, x : f32) -> Vector3 { Vector3 { x: x, y: self.y, z: self.z } }
    fn with_y(&self, y : f32) -> Vector3 { Vector3 { x: self.x, y: y, z: self.z } }
    fn with_z(&self, z : f32) -> Vector3 { Vector3 { x: self.x, y: self.y, z: z } }
}

impl Add for Vector3
{
    type Output = Vector3;
    fn add(self, v : Vector3)  -> Vector3
    {
        self.v_add(v)
    }
}

impl Sub for Vector3
{
    type Output = Vector3;
    fn sub(self, v : Vector3) -> Vector3
    {
        self.v_sub(v)
    }
}

impl Mul<f32> for Vector3
{
    type Output = Vector3;
    fn mul(self, s : f32) -> Vector3
    {
        self.scale(s)
    }
}

impl Mul<Vector3> for f32
{
    type Output = Vector3;
    fn mul(self, v : Vector3)  -> Vector3
    {
        v.scale(self)
    }
}

impl Neg for Vector3
{
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        self.v_usub()
    }
}


#[cfg(test)]
mod tests {

    use ::vector3::{Vec3, Vector3};

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = Vector3::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn v3_magnitude_squared() {
        let triangle = Vector3::newi(3, 4, 0);
        let mag_squared = triangle.magnitude_squared();
        assert_eq!(25., mag_squared);
    }

    #[test]
    fn v3_magnitude() {
        let triangle = Vector3::newi(3, 4, 0);
        let mag = triangle.magnitude();
        assert_eq!(5., mag);
    }

    #[test]
    fn cross_product() {
        let product = Vector3::i_hat().cross(Vector3::j_hat());
        assert_eq!(Vector3::k_hat(), product);

    }

    #[test]
    fn cross_product_normal() {
        let a = Vector3::newi(1, 2, 3);
        let b = Vector3::newi(4, 5, 6);
        let normal = a.cross(b);
        assert_eq!(normal, Vector3::newi(-3, 6, -3));
    }

    #[test]
    fn unit_vector() {
        let i_unit = Vector3::newi(2, 0, 0).unit();
        assert_eq!(i_unit, Vector3::i_hat());
    }

    #[test]
    fn unit_triangle() {
        let triangle = Vector3::newi(3, 4, 0);
        let unit = triangle.unit();
        let dif = (unit - Vector3::new(3.0/5.0, 4.0/5.0,0.0)).magnitude_squared();
        assert_eq!(dif < 0.0000001, true);
    }

    #[test]
    fn vector_scale() {
        let scale = Vector3::i_hat() * 3.;
        assert_eq!(scale, Vector3::newi(3, 0, 0));
    }

    #[test]
    fn triangle_scale() {
        let triangle = Vector3::newi(3, 4, 0);
        let big = triangle * 2.0;
        assert_eq!(triangle.x() * 2., big.x());
        assert_eq!(triangle.y() * 2., big.y());
        assert_eq!(triangle.z() * 2., big.z());
    }

    #[test]
    fn add_one_plus_one() {
        assert_eq!(Vector3::i_hat() + Vector3::i_hat(), Vector3::newi(2, 0, 0) );
    }

    #[test]
    fn triangle_add() {
        let triangle = Vector3::newi(3, 4, 0);
        let composed =
              Vector3::i_hat()
            + Vector3::i_hat()
            + Vector3::i_hat()
            + Vector3::j_hat()
            + Vector3::j_hat()
            + Vector3::j_hat()
            + Vector3::j_hat();
        assert_eq!(triangle, composed);
    }

    #[test]
    fn sub_one_minus_one() {
        assert_eq!(Vector3::i_hat() - Vector3::i_hat(), Vector3::zero());
    }

    #[test]
    fn unary_sub() {
        assert_eq!(-Vector3::i_hat(), Vector3::newi(-1, 0, 0));
    }
}