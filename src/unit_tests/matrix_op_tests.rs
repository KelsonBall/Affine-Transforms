

#[cfg(test)]
mod tests {    
    use ::vectors::{ AffineVector, KVector3 };
    use ::matrices::{ AffineMatrix, Primitives };
    use std::f64::consts::{ PI };

    const C : f64 = 0.5403023058681398; // cos(1)
    const S : f64 = 0.8414709848078965; // sin(1)

    const TOLERANCE : f64 = 0.0000000000000000001;

    macro_rules!  assert_aprox{
        ( $ left : expr , $ right : expr ) => (
        {
            match ( & ( $ left ) , & ( $ right ) ) {
                ( left_val , right_val ) => {
                    if ! ( (* left_val - * right_val).magnitude_squared() < TOLERANCE ) { panic!("assertion failed: `(left == right)` (left: `{:?}`, right: `{:?}`)", * left_val , * right_val ) } 
                } } 
        } )

    }

    #[test]
    fn inverse_affine_identity() {
        let identity = AffineMatrix::new(Primitives::Identity);
        let ident_inverse = identity.inverse();
        assert_eq!(identity, ident_inverse); // should be exact
    }

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
        assert_eq!(inverse, expected); // should be exact
    }

    #[test]
    fn inverse_rotation() {
        // create a rotation matrix for 1 radian about the Z axis
        let rotate = AffineMatrix::new(Primitives::RotationZ(1.));

        // create a matrix that undoes the rotation of 'rotate'
        let revert = rotate.inverse();

        // apply the transformation to the vector <1,0,0>
        let rotated = rotate.apply_vec3(KVector3::i_hat());        

        // assert that the result is <cos(1),sin(1),0>
        let expected = KVector3::new(C, S, 0.0);        
        assert_aprox!(rotated, expected);

        // use the 'revert' matrix to undo the rotation
        let returned = revert.apply_vec3(rotated);     

        // assert that the result is back to <1,0,0>, within a tolerance
        let i = KVector3::i_hat();
        assert_aprox!(returned, i);        
    }
    
    #[test]
    fn rotation_z_matrix() {
        // create a rotation matrix for 1 radian about the Z axis
        let rotate = AffineMatrix::new(Primitives::RotationZ(1.));

        assert_eq!(rotate.rvec(1), AffineVector::new( C ,-S , 0., 0.));
        assert_eq!(rotate.rvec(2), AffineVector::new( S , C , 0., 0.));
        assert_eq!(rotate.rvec(3), AffineVector::new( 0., 0., 1., 0.));
        assert_eq!(rotate.rvec(4), AffineVector::new( 0., 0., 0., 1.));
    }

    #[test]
    fn rotation_y_matrix() {
        // create a rotation matrix for 1 radian about the Y axis
        let rotate = AffineMatrix::new(Primitives::RotationY(1.));

        assert_eq!(rotate.rvec(1), AffineVector::new( C , 0. , S ,  0.));
        assert_eq!(rotate.rvec(2), AffineVector::new( 0., 1. , 0.,  0.));
        assert_eq!(rotate.rvec(3), AffineVector::new(-S , 0. , C ,  0.));
        assert_eq!(rotate.rvec(4), AffineVector::new( 0., 0. , 0.,  1.));
    }

    #[test]
    fn rotation_x_matrix() {
        // create a rotation matrix for 1 radian about the X axis
        let rotate = AffineMatrix::new(Primitives::RotationX(1.));

        assert_eq!(rotate.rvec(1), AffineVector::new( 1., 0., 0.,  0.));
        assert_eq!(rotate.rvec(2), AffineVector::new( 0., C ,-S ,  0.));
        assert_eq!(rotate.rvec(3), AffineVector::new( 0., S , C ,  0.));
        assert_eq!(rotate.rvec(4), AffineVector::new( 0., 0., 0.,  1.));
    }

    #[test]
    fn rotation_axis_tour() {
        // start at x axis
        let i = KVector3::i_hat();

        // rotate <1,0,0> 1/4 turn about the z axis to get <0,1,0>
        let j = AffineMatrix::new(Primitives::RotationZ(PI / 2.0)).apply_vec3(i).round();
        assert_eq!(j, KVector3::j_hat());

        // rotate <0,1,0> 1/4 turn about the x axis to get <0,0,1>
        let k = AffineMatrix::new(Primitives::RotationX(PI / 2.0)).apply_vec3(j).round();
        assert_eq!(k, KVector3::k_hat());

        // rotate <0,0,1> 1/4 turn about the y axis to get <1,0,0>
        let i2 = AffineMatrix::new(Primitives::RotationY(PI / 2.0)).apply_vec3(k).round();
        assert_eq!(i, i2);
    }
}