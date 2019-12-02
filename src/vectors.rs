use std::ops::{ Add, Sub, Mul, Neg };

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector3
{
    x : f64,
    y : f64,
    z : f64,
}

impl Vector3 
{
    pub fn magnitude_squared(&self) -> f64 
    {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn magnitude(&self) -> f64 
    {
        self.magnitude_squared().sqrt()
    }  

    pub fn new(x : f64, y : f64, z : f64) -> Vector3 
    {
        Vector3 { x: x, y: y, z: z }
    }

    pub fn newi(x : i32, y : i32, z : i32) -> Vector3 
    {
        Vector3 { x: x as f64, y: y as f64, z: z as f64 }
    }

    pub fn dot(&self, v : Vector3) -> f64 
    {
        self.x * v.x + self.y * v.y + self.z * v.z        
    }

    pub fn cross(&self, v : Vector3) -> Vector3 
    {
        Vector3 {
            x : self.y * v.z - self.z * v.y,
            y : self.z * v.x - self.x * v.z,
            z : self.x * v.y - self.y * v.x
        }
    }

    pub fn unit(&self) -> Vector3 
    {
        self.scale(1.0 / self.magnitude())
    }

    pub fn v_add(&self, v : Vector3) -> Vector3 
    {
        Vector3 { x : self.x + v.x, y : self.y + v.y, z : self.z + v.z }
    }

    pub fn v_usub(&self) -> Vector3 
    {
        Vector3 { x : -self.x, y : -self.y, z : -self.z }
    }

    pub fn v_sub(&self, v : Vector3) -> Vector3 
    {
        Vector3 { x : self.x - v.x, y : self.y - v.y, z : self.z - v.z }
    }

    pub fn scale(&self, s : f64) -> Vector3 
    {
        Vector3 { x : self.x * s, y : self.y * s, z : self.z * s }
    }

    pub fn round(&self) -> Vector3 
    {
        Vector3 { x : self.x.round(), y : self.y.round(), z : self.z.round() }
    }

    pub fn x(&self) -> f64 { self.x }
    pub fn y(&self) -> f64 { self.y }
    pub fn z(&self) -> f64 { self.z }

    pub fn  zero() -> Vector3 { Vector3 { x: 0., y: 0., z: 0. } }
    pub fn identity() -> Vector3 { Vector3 { x: 1., y: 1., z: 1. } }
    pub fn i_hat() -> Vector3 { Vector3 { x: 1., y: 0., z: 0. } }
    pub fn j_hat() -> Vector3 { Vector3 { x: 0., y: 1., z: 0. } }
    pub fn k_hat() -> Vector3 { Vector3 { x: 0., y: 0., z: 1. } }

    pub fn with_x(&self, x : f64) -> Vector3 { Vector3 { x: x, y: self.y, z: self.z } }
    pub fn with_y(&self, y : f64) -> Vector3 { Vector3 { x: self.x, y: y, z: self.z } }
    pub fn with_z(&self, z : f64) -> Vector3 { Vector3 { x: self.x, y: self.y, z: z } }
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

impl Mul<f64> for Vector3 
{
    type Output = Vector3;
    fn mul(self, s : f64) -> Vector3 
    {
        self.scale(s)
    }
}

impl Mul<Vector3> for f64 
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

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
pub struct Vector4 
{
    x : f64,
    y : f64,
    z : f64,
    w : f64,
}

impl Vector4 
{

    pub fn magnitude_squared(&self) -> f64 {        
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }  

    pub fn dot(&self, v: Vector4) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z + self.w * v.w            
    }

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4 { Vector4 { x: x, y: y, z: z, w: w } }

    pub fn x(self) -> f64 { self.x }
    pub fn y(self) -> f64 { self.y }
    pub fn z(self) -> f64 { self.z }
    pub fn w(self) -> f64 { self.w }
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
    
    use ::vectors::Vector3;

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