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

    fn v3_cross(&self, v: fvec) -> fvec 
    {
        fvec::new(
            self.y() * v.z() - self.z() * v.y(),
            self.z() * v.x() - self.x() * v.z(),
            self.x() * v.y() - self.y() * v.x(),
            self.w()
        )
    }
    
    fn unit(&self) -> fvec { *self / self.magnitude() }

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

#[cfg(test)]
mod tests {
    use ::vector4::{Vec4};
    use packed_simd::f32x4 as fvec;

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = fvec::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn v3_magnitude_squared() {
        let triangle = fvec::new(3., 4., 0., 0.);
        let mag_squared = triangle.magnitude_squared();
        assert_eq!(25., mag_squared);
    }

    #[test]
    fn v3_magnitude() {
        let triangle = fvec::new(3., 4., 0., 0.);
        let mag = triangle.magnitude();
        assert_eq!(5., mag);
    }

    #[test]
    fn cross_product() {
        let product = fvec::i_hat().v3_cross(fvec::j_hat());
        assert_eq!(fvec::k_hat(), product);

    }

    #[test]
    fn cross_product_normal() {
        let a = fvec::new(1., 2., 3., 0.);
        let b = fvec::new(4., 5., 6., 0.);
        let normal = a.v3_cross(b);
        assert_eq!(normal, fvec::new(-3., 6., -3., 0.));
    }

    #[test]
    fn unit_vector() {
        let i_unit = fvec::new(2., 0., 0., 0.).unit();
        assert_eq!(i_unit, fvec::i_hat());
    }

    #[test]
    fn unit_triangle() {
        let triangle = fvec::new(3., 4., 0., 0.);
        let unit = triangle.unit();
        let dif = (unit - fvec::new(3.0/5.0, 4.0/5.0,0.0, 0.0)).magnitude_squared();
        assert_eq!(dif < 0.0000001, true);
    }

    #[test]
    fn vector_scale() {
        let scale = fvec::i_hat() * 3.;
        assert_eq!(scale, fvec::new(3., 0., 0., 0.));
    }

    #[test]
    fn triangle_scale() {
        let triangle = fvec::new(3., 4., 0., 0.);
        let big = triangle * 2.0;
        assert_eq!(triangle.x() * 2., big.x());
        assert_eq!(triangle.y() * 2., big.y());
        assert_eq!(triangle.z() * 2., big.z());
    }

    #[test]
    fn add_one_plus_one() {
        assert_eq!(fvec::i_hat() + fvec::i_hat(), fvec::new(2., 0., 0., 0.) );
    }

    #[test]
    fn triangle_add() {
        let triangle = fvec::new(3., 4., 0., 0.);
        let composed =
              fvec::i_hat()
            + fvec::i_hat()
            + fvec::i_hat()
            + fvec::j_hat()
            + fvec::j_hat()
            + fvec::j_hat()
            + fvec::j_hat();
        assert_eq!(triangle, composed);
    }

    #[test]
    fn sub_one_minus_one() {
        assert_eq!(fvec::i_hat() - fvec::i_hat(), fvec::zero());
    }

    #[test]
    fn unary_sub() {
        assert_eq!(-fvec::i_hat(), fvec::new(-1., 0., 0., 0.));
    }
}