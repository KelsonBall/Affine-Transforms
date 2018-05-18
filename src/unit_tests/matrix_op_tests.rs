#[cfg(test)]
mod tests {    
    use ::vectors::{ AffineVector, KVector3 };
    use ::matrices::{ AffineMatrix, Cell, Primitives };

    fn c() -> f64 { 0.5403023058681398 }
    fn s() -> f64 { 0.8414709848078965 }

    #[test]
    fn inverse_perspective() {
        let perspective = AffineMatrix::from_row_major(
            vec![1., 0., 0., 0.,
                 0., 1., 0., 0.,
                 0., 0., 1., 0.,
                 0., 0., 1., 1.]);
        let inverse = perspective.inverse();
        let expected = AffineMatrix::from_row_major(
            vec![1., 0., 0., 0.,
                 0., 1., 0., 0.,
                 0., 0., 1., 0.,
                 0., 0.,-1., 1.]);
        assert_eq!(inverse, expected);
    }

    #[test]
    fn inverse_rotation() {
        let rotate = AffineMatrix::new(Primitives::RotationZ(1.));
        let revert = rotate.inverse();

        let rotated = rotate.apply_vec3(KVector3::i_hat());        
        let expected = KVector3::new(0.5403023058681398, 0.8414709848078965, 0.0);        
        assert_eq!(rotated, expected);

        let returned = revert.apply_vec3(rotated);        
        assert_eq!(returned, KVector3::i_hat());
    }
    
    #[test]
    fn rotation_z_matrix() {
        let rotate = AffineMatrix::new(Primitives::RotationZ(1.));

        assert_eq!(rotate.rvec(1), AffineVector::new(c(),-s(),  0.,  0.));
        assert_eq!(rotate.rvec(2), AffineVector::new(s(), c(),  0.,  0.));
        assert_eq!(rotate.rvec(3), AffineVector::new( 0.,  0.,  1.,  0.));
        assert_eq!(rotate.rvec(4), AffineVector::new( 0.,  0.,  0.,  1.));
    }
}