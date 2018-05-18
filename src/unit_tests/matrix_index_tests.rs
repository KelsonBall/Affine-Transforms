#[cfg(test)]
mod tests {        
    use ::matrices::{ AffineMatrix, Cell };

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
            assert_eq!(matrix[Cell::Row(i)], (i + 1) as f64); // values increment along row major order
            assert_eq!(matrix[Cell::Column(i)], ((i * 4 % 16) + (i / 4) + 1) as f64);
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
            assert_eq!(matrix[Cell::Column(i)], (i + 1) as f64); // values increment along column major order
            assert_eq!(matrix[Cell::Row(i)], ((i * 4 % 16) + (i / 4) + 1) as f64);
        }
    }
}