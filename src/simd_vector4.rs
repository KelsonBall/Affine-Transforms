use ::vector4::{Vec4, Vector4};
use packed_simd::f32x4 as fvec;

pub fn to_vector4(a: &fvec) -> Vector4 {
    Vector4::new( a.extract(0), a.extract(1), a.extract(2), a.extract(3) )
}

pub fn to_simd_vector(a: &Vector4) -> fvec {
    fvec::new(a.x(), a.y(), a.z(), a.w())
}

impl Vec4<fvec> for fvec
{
    fn magnitude_squared(&self) -> f32 {
        (*self * *self).sum()
    }

    fn magnitude(&self) -> f32 {
        (*self * *self).sum().sqrt()
    }

    fn dot(&self, v: fvec) -> f32 {
        (*self * v).sum()
    }

    fn scale(&self, s : f32) -> fvec 
    {
        *self * s
    }

    fn v3_cross(&self, v: fvec) -> fvec { unimplemented!() }
    fn unit(&self) -> fvec { unimplemented!() }

    fn zero() -> fvec { fvec::new(0., 0., 0., 0.) }
    fn identity() -> fvec { fvec::new(1., 1., 1., 1.) }
    fn i_hat() -> fvec { fvec::new(1.0, 0.0, 0.0, 0.0) }
    fn j_hat() -> fvec { fvec::new(0.0, 1.0, 0.0, 0.0) }
    fn k_hat() -> fvec { fvec::new(0.0, 0.0, 1.0, 0.0) }
    fn w_hat() -> fvec { fvec::new(0.0, 0.0, 0.0, 1.0) }

    fn with_x(&self, x : f32) -> fvec { fvec::new(x, self.y(), self.z(), self.w()) }
    fn with_y(&self, y : f32) -> fvec { fvec::new(self.x(), y, self.z(), self.w()) }
    fn with_z(&self, z : f32) -> fvec { fvec::new(self.x(), self.y(), z, self.w()) }
    fn with_w(&self, w : f32) -> fvec { fvec::new(self.x(), self.y(), self.z(), w) }

    fn x(&self) -> f32 { self.extract(0) }
    fn y(&self) -> f32 { self.extract(1) }
    fn z(&self) -> f32 { self.extract(2) }
    fn w(&self) -> f32 { self.extract(3) }
}
