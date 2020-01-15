use std::ops::{ Index, Mul };
use ::vector3::{ Vec3, Vector3 };
use ::vector4::Vec4;
use packed_simd::f32x4 as fvec;

/// Uniquely identifies a single scalar value of an Affine Matrix
pub enum Cell
{
    I1, J1, K1, W1,
    I2, J2, K2, W2,
    I3, J3, K3, W3,
    I4, J4, K4, W4,
    /// By row-major index
    Row(u8),
    /// By column-major index
    Column(u8),
}

impl Cell
{
    /// Converts any Cell identifier to column-major index
    pub fn to_column(&self) -> Cell
    {
        match self {
            &Cell::I1 => Cell::Column(0), &Cell::J1 => Cell::Column(4), &Cell::K1 => Cell::Column(8),  &Cell::W1 => Cell::Column(12),
            &Cell::I2 => Cell::Column(1), &Cell::J2 => Cell::Column(5), &Cell::K2 => Cell::Column(9),  &Cell::W2 => Cell::Column(13),
            &Cell::I3 => Cell::Column(2), &Cell::J3 => Cell::Column(6), &Cell::K3 => Cell::Column(10), &Cell::W3 => Cell::Column(14),
            &Cell::I4 => Cell::Column(3), &Cell::J4 => Cell::Column(7), &Cell::K4 => Cell::Column(14), &Cell::W4 => Cell::Column(15),
            &Cell::Column(i) => Cell::Column(i),
            &Cell::Row(i) => Cell::Column((i * 4 % 16) + (i / 4))
        }
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
/// A 4 dimensional matrix for representing linear transformations of 3D space
pub struct AffineMatrix
{
    i1 : f32, j1 : f32, k1 : f32, w1 : f32,
    i2 : f32, j2 : f32, k2 : f32, w2 : f32,
    i3 : f32, j3 : f32, k3 : f32, w3 : f32,
    i4 : f32, j4 : f32, k4 : f32, w4 : f32
}

impl AffineMatrix
{
    /// Recieves the 1st, 2nd, 3rd, or 4th column as an SIMD vector
    pub fn cvec(&self, column : u8) -> fvec
    {
        let start = (column - 1) * 4;
        fvec::new(
            self[Cell::Column(start + 0)],
            self[Cell::Column(start + 1)],
            self[Cell::Column(start + 2)],
            self[Cell::Column(start + 3)])
    }

    /// Recieves the 1st, 2nd, 3rd, or 4th row as an SIMD vector
    pub fn rvec(&self, row : u8) -> fvec
    {
        let start = (row - 1) * 4;
        fvec::new(
            self[Cell::Row(start + 0)],
            self[Cell::Row(start + 1)],
            self[Cell::Row(start + 2)],
            self[Cell::Row(start + 3)])
    }

    /// Applies matrix multiplication between this matrix and another
    pub fn multiply(&self, m : AffineMatrix) -> AffineMatrix
    {
        let c1 = self.cvec(1);
        let c2 = self.cvec(2);
        let c3 = self.cvec(3);
        let c4 = self.cvec(4);
        let r1 = m.rvec(1);
        let r2 = m.rvec(2);
        let r3 = m.rvec(3);
        let r4 = m.rvec(4);

        AffineMatrix {
            i1: c1.dot(r1), j1: c2.dot(r1), k1: c3.dot(r1), w1: c4.dot(r1),
            i2: c1.dot(r2), j2: c2.dot(r2), k2: c3.dot(r2), w2: c4.dot(r2),
            i3: c1.dot(r3), j3: c2.dot(r3), k3: c3.dot(r3), w3: c4.dot(r3),
            i4: c1.dot(r4), j4: c2.dot(r4), k4: c3.dot(r4), w4: c4.dot(r4),
        }
    }

    pub fn apply_affine(&self, a : fvec) -> fvec
    {
        fvec::new(self.rvec(1).dot(a),  self.rvec(2).dot(a), self.rvec(3).dot(a), self.rvec(4).dot(a))
    }

    pub fn apply_vec3(&self, v : Vector3) -> Vector3
    {
        let a = self.apply_affine(fvec::new(v.x(), v.y(), v.z(), 1.));
        Vector3::new(a.x(), a.y(), a.z())
    }

    pub fn inverse(&self) -> AffineMatrix
    {
        let m = self;

        let sa = fvec::new(m.i1, m.i1, m.i1, m.j1) * fvec::new(m.j2, m.k2, m.w2, m.k2);
        let sb = fvec::new(m.i2, m.i2, m.i2, m.j2) * fvec::new(m.j1, m.k1, m.w1, m.k1);
        let sv = sa - sb;
        let s0 = sv.extract(0);
        let s1 = sv.extract(1);
        let s2 = sv.extract(2);
        let s3 = sv.extract(3);

        let sca = fvec::new(m.j1, m.k1, m.k3, m.j3) * fvec::new(m.w2, m.w2, m.w4, m.w4);
        let scb = fvec::new(m.j2, m.k2, m.k4, m.j4) * fvec::new(m.w1, m.w1, m.w3, m.w3);
        let scv = sca - scb;
        let s4 = scv.extract(0);
        let s5 = scv.extract(1);
        let c5 = scv.extract(2);
        let c4 = scv.extract(3);

        let ca = fvec::new(m.j3, m.i3, m.i3, m.i3) * fvec::new(m.k4, m.w4, m.k4, m.j4);
        let cb = fvec::new(m.j4, m.i4, m.i4, m.i4) * fvec::new(m.k3, m.w3, m.k3, m.j3);
        let cv = ca - cb;
        let c3 = cv.extract(0);
        let c2 = cv.extract(1);
        let c1 = cv.extract(2);
        let c0 = cv.extract(3);

        let d = 1.0 / (s0 * c5 - s1 * c4 + s2 * c3 + s3 * c2 - s4 * c1 + s5 * c0);

        let c1v = (fvec::new(m.j2, -m.j1, m.j4, -m.j3) * fvec::new(c5, c5, s5, s5) + fvec::new(-m.k2, m.k1, -m.k4, m.k3) * fvec::new(c4, c4, s4, s4) + fvec::new(m.w2, -m.w1, m.w4, -m.w3) * fvec::new(c3, c3, s3, s3)) * d;
        let c2v = (fvec::new(-m.i2, m.i1, -m.i4, m.i3) * fvec::new(c5, c5, s5, s5) + fvec::new(m.k2, -m.k1, m.k4, -m.k3) * fvec::new(c2, c2, s2, s2) + fvec::new(-m.w2, m.w1, -m.w4, m.w3) * fvec::new(c1, c1, s1, s1)) * d;
        let c3v = (fvec::new( m.i2, -m.i1, m.i4, -m.i3) * fvec::new(c4, c4, s4, s4) + fvec::new(m.w2, -m.w1, m.w4, -m.w3) * fvec::new(c2, c2, s2, s2) + fvec::new(m.w2, -m.w1, m.w4, -m.w3) * fvec::new(c0, c0, s0, s0)) * d;
        let c4v = (fvec::new(-m.i2, m.i1, -m.i4, m.i3) * fvec::new(c3, c3, s3, s3) + fvec::new(-m.j2, m.j1, -m.j4, m.j3) * fvec::new(c1, c1, s1, s1) + fvec::new(-m.k2, m.k1, -m.k4, m.k3) * fvec::new(c0, c0, s0, s0)) * d;

        AffineMatrix {
            i1: c1v.extract(0), j1: c1v.extract(1), k1: c1v.extract(2), w1: c1v.extract(3),
            i2: c2v.extract(0), j2: c2v.extract(1), k2: c2v.extract(2), w2: c2v.extract(3),
            i3: c3v.extract(0), j3: c3v.extract(1), k3: c3v.extract(2), w3: c3v.extract(3),
            i4: c4v.extract(0), j4: c4v.extract(1), k4: c4v.extract(2), w4: c4v.extract(3),
        }
    }

    pub fn from_row_major(array : Vec<f32>) -> AffineMatrix
    {
        AffineMatrix {
            i1: array[0], j1: array[1], k1: array[2], w1: array[3],
            i2: array[4], j2: array[5], k2: array[6], w2: array[7],
            i3: array[8], j3: array[9], k3: array[10],w3: array[11],
            i4: array[12],j4: array[13],k4: array[14],w4: array[15],
        }
    }

    pub fn from_column_major(array : Vec<f32>) -> AffineMatrix
    {
        AffineMatrix {
            i1: array[0], j1: array[4], k1: array[8], w1: array[12],
            i2: array[1], j2: array[5], k2: array[9], w2: array[13],
            i3: array[2], j3: array[6], k3: array[10],w3: array[14],
            i4: array[3], j4: array[7], k4: array[11],w4: array[15],
        }
    }

    pub fn zero() -> AffineMatrix
    {
        AffineMatrix::from_row_major(vec![0.0;16])
    }

    pub fn identity() -> AffineMatrix
    {
        AffineMatrix {
            i1: 1., j1: 0., k1: 0., w1: 0.,
            i2: 0., j2: 1., k2: 0., w2: 0.,
            i3: 0., j3: 0., k3: 1., w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn translation(x : f32, y : f32, z : f32) -> AffineMatrix
    {
        AffineMatrix {
                i1: 1., j1: 0., k1: 0., w1: x ,
                i2: 0., j2: 1., k2: 0., w2: y ,
                i3: 0., j3: 0., k3: 1., w3: z ,
                i4: 0., j4: 0., k4: 0., w4: 1.,
            }
    }

    pub fn rotation_x(theta : f32) -> AffineMatrix
    {
        let c = theta.cos();
        let s = theta.sin();
        AffineMatrix {
            i1: 1., j1: 0., k1: 0., w1: 0.,
            i2: 0., j2: c , k2: -s, w2: 0.,
            i3: 0., j3: s , k3: c , w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn rotation_y(theta : f32) -> AffineMatrix
    {
        let c = theta.cos();
        let s = theta.sin();
        AffineMatrix {
            i1: c , j1: 0., k1: s , w1: 0.,
            i2: 0., j2: 1., k2: 0., w2: 0.,
            i3: -s, j3: 0., k3: c , w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn rotation_z(theta : f32) -> AffineMatrix
    {
        let c = theta.cos();
        let s = theta.sin();
        AffineMatrix {
            i1: c , j1: -s, k1: 0., w1: 0.,
            i2: s , j2: c , k2: 0., w2: 0.,
            i3: 0., j3: 0., k3: 1., w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn scale(x : f32, y : f32, z : f32) -> AffineMatrix
    {
        AffineMatrix {
            i1: x , j1: 0., k1: 0., w1: 0.,
            i2: 0., j2: y , k2: 0., w2: 0.,
            i3: 0., j3: 0., k3: z , w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn uniform_scale(s : f32) -> AffineMatrix
    {
        AffineMatrix {
            i1: s , j1: 0., k1: 0., w1: 0.,
            i2: 0., j2: s , k2: 0., w2: 0.,
            i3: 0., j3: 0., k3: s , w3: 0.,
            i4: 0., j4: 0., k4: 0., w4: 1.,
        }
    }

    pub fn transpose(&self) -> AffineMatrix
    {
        AffineMatrix {
            i1: self.i1, j1: self.i2, k1: self.i3, w1: self.i4,
            i2: self.j1, j2: self.j2, k2: self.j3, w2: self.j4,
            i3: self.k1, j3: self.k2, k3: self.k3, w3: self.k4,
            i4: self.w1, j4: self.w2, k4: self.w3, w4: self.w4
        }
    }

    pub fn as_row_major_vec(&self) -> Vec<f32>
    {
        vec![
            self.i1, self.j1, self.k1, self.w1,
            self.i2, self.j2, self.k2, self.w2,
            self.i3, self.j3, self.k3, self.w3,
            self.i4, self.j4, self.k4, self.w4
        ]
    }
}

impl Index<Cell> for AffineMatrix
{
    type Output = f32;
    fn index(&self, c : Cell) -> &f32
    {
        match c {
            Cell::I1 => &self.i1, Cell::I2 => &self.i2, Cell::I3 => &self.i3, Cell::I4 => &self.i4,
            Cell::J1 => &self.j1, Cell::J2 => &self.j2, Cell::J3 => &self.j3, Cell::J4 => &self.j4,
            Cell::K1 => &self.k1, Cell::K2 => &self.k2, Cell::K3 => &self.k3, Cell::K4 => &self.k4,
            Cell::W1 => &self.w1, Cell::W2 => &self.w2, Cell::W3 => &self.w3, Cell::W4 => &self.w4,
            Cell::Column(0) => &self.i1, Cell::Column(4) => &self.j1, Cell::Column(8) => &self.k1, Cell::Column(12) => &self.w1,
            Cell::Column(1) => &self.i2, Cell::Column(5) => &self.j2, Cell::Column(9) => &self.k2, Cell::Column(13) => &self.w2,
            Cell::Column(2) => &self.i3, Cell::Column(6) => &self.j3, Cell::Column(10) =>&self.k3, Cell::Column(14) => &self.w3,
            Cell::Column(3) => &self.i4, Cell::Column(7) => &self.j4, Cell::Column(11) =>&self.k4, Cell::Column(15) => &self.w4,
            Cell::Column(_) => panic!("Matrix Index out of bounds"),
            Cell::Row(0) => &self.i1, Cell::Row(1) => &self.j1, Cell::Row(2) => &self.k1, Cell::Row(3) => &self.w1,
            Cell::Row(4) => &self.i2, Cell::Row(5) => &self.j2, Cell::Row(6) => &self.k2, Cell::Row(7) => &self.w2,
            Cell::Row(8) => &self.i3, Cell::Row(9) => &self.j3, Cell::Row(10) =>&self.k3, Cell::Row(11) =>&self.w3,
            Cell::Row(12) =>&self.i4, Cell::Row(13) =>&self.j4, Cell::Row(14) =>&self.k4, Cell::Row(15) =>&self.w4,
            Cell::Row(_) => panic!("Matrix Index out of bounds"),
        }
    }
}

impl Index<i32> for AffineMatrix
{
    type Output = f32;
    fn index(&self, c : i32) -> &f32 {
        match c {
            0 => &self.i1, 4 => &self.i2, 8 => &self.i3, 12=> &self.i4,
            1 => &self.j1, 5 => &self.j2, 9 => &self.j3, 13=> &self.j4,
            2 => &self.k1, 6 => &self.k2, 10=> &self.k3, 14=> &self.k4,
            3 => &self.w1, 7 => &self.w2, 11=> &self.w3, 15=> &self.w4,
            _  => panic!("Matrix Index out of bounds")
        }
    }
}

impl Mul for AffineMatrix
{
    type Output = AffineMatrix;
    fn mul(self, m : AffineMatrix)  -> AffineMatrix {
        self.multiply(m)
    }
}

impl Mul<Vector3> for AffineMatrix
{
    type Output = Vector3;
    fn mul(self, v : Vector3) -> Vector3 {
        self.apply_vec3(v)
    }
}

impl Mul<fvec> for AffineMatrix
{
    type Output = fvec;
    fn mul(self, v : fvec) -> fvec {
        self.apply_affine(v)
    }
}

#[cfg(test)]
mod tests {
    use ::vector3::{Vec3, Vector3};
    use packed_simd::f32x4 as fvec;
    use ::matrices::{ AffineMatrix, Cell };
    use std::f32::consts::{ PI };
    use test::Bencher;

    const C : f32 = 0.5403023058681398; // cos(1)
    const S : f32 = 0.8414709848078965; // sin(1)

    const TOLERANCE : f32 = 0.000001;

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
        let identity = AffineMatrix::identity();
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
        // create a rotation Matrix for 1 radian about the Z axis
        let rotate = AffineMatrix::rotation_z(1.);

        // create a Matrix that undoes the rotation of 'rotate'
        let revert = rotate.inverse();

        // apply the transformation to the vector <1,0,0>
        let rotated = rotate.apply_vec3(Vector3::i_hat());

        // assert that the result is <cos(1),sin(1),0>
        let expected = Vector3::new(C, S, 0.0);
        assert_aprox!(rotated, expected);

        // use the 'revert' Matrix to undo the rotation
        let returned = revert.apply_vec3(rotated);

        // assert that the result is back to <1,0,0>, within a tolerance
        let i = Vector3::i_hat();
        assert_aprox!(returned, i);
    }

    #[bench]
    fn inverse_benchmark(b: &mut Bencher) {
        let mat = AffineMatrix::rotation_z(1.123) * AffineMatrix::translation(0.2, 0.3, 12.2) * AffineMatrix::rotation_y(-2.);
        b.iter(|| {
            let mut a = mat.inverse();
            for _ in 0..10_000 {
                a = mat.inverse();
            }
            a
        })
    }

    #[test]
    fn rotation_z_matrix() {
        // create a rotation Matrix for 1 radian about the Z axis
        let rotate = AffineMatrix::rotation_z(1.);

        assert_eq!(rotate.rvec(1), fvec::new( C ,-S , 0., 0.));
        assert_eq!(rotate.rvec(2), fvec::new( S , C , 0., 0.));
        assert_eq!(rotate.rvec(3), fvec::new( 0., 0., 1., 0.));
        assert_eq!(rotate.rvec(4), fvec::new( 0., 0., 0., 1.));
    }

    #[test]
    fn rotation_y_matrix() {
        // create a rotation Matrix for 1 radian about the Y axis
        let rotate = AffineMatrix::rotation_y(1.);

        assert_eq!(rotate.rvec(1), fvec::new( C , 0. , S ,  0.));
        assert_eq!(rotate.rvec(2), fvec::new( 0., 1. , 0.,  0.));
        assert_eq!(rotate.rvec(3), fvec::new(-S , 0. , C ,  0.));
        assert_eq!(rotate.rvec(4), fvec::new( 0., 0. , 0.,  1.));
    }

    #[test]
    fn rotation_x_matrix() {
        // create a rotation Matrix for 1 radian about the X axis
        let rotate = AffineMatrix::rotation_x(1.);

        assert_eq!(rotate.rvec(1), fvec::new( 1., 0., 0.,  0.));
        assert_eq!(rotate.rvec(2), fvec::new( 0., C ,-S ,  0.));
        assert_eq!(rotate.rvec(3), fvec::new( 0., S , C ,  0.));
        assert_eq!(rotate.rvec(4), fvec::new( 0., 0., 0.,  1.));
    }

    #[test]
    fn rotation_axis_tour() {
        // start at x axis
        let i = Vector3::i_hat();

        // rotate <1,0,0> 1/4 turn about the z axis to get <0,1,0>
        let j = AffineMatrix::rotation_z(PI / 2.0).apply_vec3(i).round();
        assert_eq!(j, Vector3::j_hat());

        // rotate <0,1,0> 1/4 turn about the x axis to get <0,0,1>
        let k = AffineMatrix::rotation_x(PI / 2.0).apply_vec3(j).round();
        assert_eq!(k, Vector3::k_hat());

        // rotate <0,0,1> 1/4 turn about the y axis to get <1,0,0>
        let i2 = AffineMatrix::rotation_y(PI / 2.0).apply_vec3(k).round();
        assert_eq!(i, i2);
    }

    #[test]
    fn translate() {
        // start at <1,0,0>
        let i = Vector3::i_hat();

        // move 'left' 1 unit and 'up' 1 unit
        let t = AffineMatrix::translation(-1., 1., 0.);

        let should_be_j = t.apply_vec3(i);

        assert_aprox!(should_be_j, Vector3::j_hat());
    }

    #[test]
    fn primitives_multiply_as_matrices() {

        let i = Vector3::i_hat();

        let t = AffineMatrix::rotation_z(PI / 2.0) * AffineMatrix::uniform_scale(2.0);

        let should_be_2j = t * i;

        assert_aprox!(should_be_2j, Vector3::j_hat() * 2.0);
    }

    #[test]
    fn row_major_ctor_with_ones() {
        let mut v = vec![1.;14];
        v.push(0.);
        v.push(2.);
        let matrix = AffineMatrix::from_row_major(v);
        for i in 0..14 {
            assert_eq!(matrix[Cell::Row(i)], 1.);
        }
        assert_eq!(matrix[Cell::Row(14)], 0.);
        assert_eq!(matrix[Cell::Row(15)], 2.);
    }

    #[test]
    fn row_major_ctor_with_incrementing() {
        let matrix = AffineMatrix::from_row_major(
            vec![1., 2., 3., 4.,
                 5., 6., 7., 8.,
                 9.,10.,11.,12.,
                 13.,14.,15.,16.]);

        for i in 0..16 {
            assert_eq!(matrix[Cell::Row(i)], (i + 1) as f32); // values increment along row major order
            assert_eq!(matrix[Cell::Column(i)], ((i * 4 % 16) + (i / 4) + 1) as f32);
        }
    }

    #[test]
    fn column_major_ctor_with_ones() {
        let mut v = vec![1.;14];
        v.push(0.);
        v.push(2.);
        let matrix = AffineMatrix::from_column_major(v);
        for i in 0..14 {
            assert_eq!(matrix[Cell::Column(i)], 1.);
        }
        assert_eq!(matrix[Cell::Column(14)], 0.);
        assert_eq!(matrix[Cell::Column(15)], 2.);
    }

    #[test]
    fn column_major_ctor_with_incrementing() {
        let matrix = AffineMatrix::from_column_major(
            vec![1., 2., 3., 4.,
                 5., 6., 7., 8.,
                 9.,10.,11.,12.,
                 13.,14.,15.,16.]);

        for i in 0..16 {
            assert_eq!(matrix[Cell::Column(i)], (i + 1) as f32); // values increment along column major order
            assert_eq!(matrix[Cell::Row(i)], ((i * 4 % 16) + (i / 4) + 1) as f32);
        }
    }
}