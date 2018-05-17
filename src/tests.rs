#[cfg(test)]
mod tests {    
    
    use ::vectors::{Vector, Vector3, KVector3};

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = KVector3::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn v3_magnitude_squared() {
        let triangle = KVector3 { x : 3., y : 4., z: 0. };
        let mag_squared = triangle.magnitude_squared();
        assert_eq!(25., mag_squared);
    }

    #[test]
    fn v3_magnitude() {
        let triangle = KVector3 { x : 3., y : 4., z: 0. };
        let mag = triangle.magnitude();
        assert_eq!(5., mag);
    }

    #[test]
    fn cross_product() {        
        let product = KVector3::i_hat().cross(KVector3::j_hat());
        assert_eq!(KVector3::k_hat(), product);

    }

    #[test]
    fn cross_product_normal() {
        let a = KVector3 { x: 1., y: 2., z: 3. };
        let b = KVector3 { x: 4., y: 5., z: 6. };
        let normal = a.cross(b);        
        assert_eq!(normal, KVector3{ x: -3.,y: 6., z: -3. });
    }

    #[test]
    fn unit_vector() {
        let i_unit = KVector3{ x: 2., y: 0., z: 0. }.unit();
        assert_eq!(i_unit, KVector3::i_hat());
    }

    #[test]
    fn unit_triangle() {
        let triangle = KVector3 { x : 3., y : 4., z: 0. };
        let unit = triangle.unit();
        let dif = unit.sub(KVector3{ x: 3. / 5., y: 4. / 5., z: 0.}).magnitude_squared();
        assert_eq!(dif < 0.0000001, true);
    }

    #[test]
    fn vector_scale() {
        let scale = KVector3::i_hat().scale(3.);
        assert_eq!(scale, KVector3 { x: 3., y: 0., z: 0. });        
    }

    #[test]
    fn triangle_scale() {
        let triangle = KVector3 { x : 3., y : 4., z: 0. };
        let big = triangle.scale(2.);
        assert_eq!(triangle.x * 2., big.x);
        assert_eq!(triangle.y * 2., big.y);
        assert_eq!(triangle.z * 2., big.z);
    }

    #[test]
    fn triangle_add() {
        let triangle = KVector3{ x: 3., y: 4., z: 0. };
        let composed = KVector3::i_hat()
            .add(KVector3::i_hat())
            .add(KVector3::i_hat())
            .add(KVector3::j_hat())
            .add(KVector3::j_hat())
            .add(KVector3::j_hat())
            .add(KVector3::j_hat());
        assert_eq!(triangle, composed);
    }
}
