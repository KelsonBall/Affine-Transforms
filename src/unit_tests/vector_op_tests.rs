#[cfg(test)]
mod tests {    
    
    use ::vectors::Vector;

    #[test]
    fn unit_magnitude_squared() {
        let mag_squared = Vector::i_hat().magnitude_squared();
        assert_eq!(1., mag_squared);
    }

    #[test]
    fn v3_magnitude_squared() {
        let triangle = Vector::newi(3, 4, 0);
        let mag_squared = triangle.magnitude_squared();
        assert_eq!(25., mag_squared);
    }

    #[test]
    fn v3_magnitude() {
        let triangle = Vector::newi(3, 4, 0);
        let mag = triangle.magnitude();
        assert_eq!(5., mag);
    }

    #[test]
    fn cross_product() {        
        let product = Vector::i_hat().cross(Vector::j_hat());
        assert_eq!(Vector::k_hat(), product);

    }

    #[test]
    fn cross_product_normal() {
        let a = Vector::newi(1, 2, 3);
        let b = Vector::newi(4, 5, 6);
        let normal = a.cross(b);        
        assert_eq!(normal, Vector::newi(-3, 6, -3));
    }

    #[test]
    fn unit_vector() {
        let i_unit = Vector::newi(2, 0, 0).unit();
        assert_eq!(i_unit, Vector::i_hat());
    }

    #[test]
    fn unit_triangle() {
        let triangle = Vector::newi(3, 4, 0);
        let unit = triangle.unit();
        let dif = (unit - Vector::new(3.0/5.0, 4.0/5.0,0.0)).magnitude_squared();
        assert_eq!(dif < 0.0000001, true);
    }

    #[test]
    fn vector_scale() {
        let scale = Vector::i_hat() * 3.;
        assert_eq!(scale, Vector::newi(3, 0, 0));        
    }

    #[test]
    fn triangle_scale() {
        let triangle = Vector::newi(3, 4, 0);
        let big = triangle * 2.0;
        assert_eq!(triangle.x() * 2., big.x());
        assert_eq!(triangle.y() * 2., big.y());
        assert_eq!(triangle.z() * 2., big.z());
    }

    #[test]
    fn add_one_plus_one() {
        assert_eq!(Vector::i_hat() + Vector::i_hat(), Vector::newi(2, 0, 0) );
    }

    #[test]
    fn triangle_add() {
        let triangle = Vector::newi(3, 4, 0);
        let composed = 
              Vector::i_hat()
            + Vector::i_hat()
            + Vector::i_hat()
            + Vector::j_hat()
            + Vector::j_hat()
            + Vector::j_hat()
            + Vector::j_hat();
        assert_eq!(triangle, composed);
    }

    #[test]
    fn sub_one_minus_one() {
        assert_eq!(Vector::i_hat() - Vector::i_hat(), Vector::zero());
    }

    #[test]
    fn unary_sub() {
        assert_eq!(-Vector::i_hat(), Vector::newi(-1, 0, 0));
    }
}
