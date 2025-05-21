use rust_web::f2linalg::{F2, Mat2};

#[test]
fn test_f2_arithmetic() {
    // Test addition
    assert_eq!(F2::Zero + F2::Zero, F2::Zero);
    assert_eq!(F2::Zero + F2::One, F2::One);
    assert_eq!(F2::One + F2::Zero, F2::One);
    assert_eq!(F2::One + F2::One, F2::Zero); // 1 + 1 = 0 (mod 2)

    // Test multiplication
    assert_eq!(F2::Zero * F2::Zero, F2::Zero);
    assert_eq!(F2::Zero * F2::One, F2::Zero);
    assert_eq!(F2::One * F2::Zero, F2::Zero);
    assert_eq!(F2::One * F2::One, F2::One);
}

#[test]
fn test_matrix_addition() {
    // Test addition of matrices
    let a = Mat2::from_u8(vec![
        vec![1, 0, 1],
        vec![0, 1, 1],
    ]);
    let b = Mat2::from_u8(vec![
        vec![1, 1, 0],
        vec![1, 0, 1],
    ]);
    let expected = Mat2::from_u8(vec![
        vec![0, 1, 1], // 1+1=0, 0+1=1, 1+0=1
        vec![1, 1, 0], // 0+1=1, 1+0=1, 1+1=0
    ]);
    assert_eq!(a + b, expected);
}

#[test]
fn test_matrix_multiplication() {
    // Test multiplication of matrices
    let a = Mat2::from_u8(vec![
        vec![1, 0, 1],
        vec![0, 1, 1],
    ]);
    let b = Mat2::from_u8(vec![
        vec![1, 1],
        vec![0, 1],
        vec![1, 0],
    ]);
    let expected = Mat2::from_u8(vec![
        vec![0, 1], // (1*1 + 0*0 + 1*1) mod 2 = 0, (1*1 + 0*1 + 1*0) mod 2 = 1
        vec![1, 1], // (0*1 + 1*0 + 1*1) mod 2 = 1, (0*1 + 1*1 + 1*0) mod 2 = 1
    ]);
    assert_eq!(a * b, expected);
}

#[test]
fn test_matrix_rank() {
    // Test rank computation
    let m = Mat2::from_u8(vec![
        vec![1, 0, 1],
        vec![0, 1, 1],
        vec![1, 1, 0],
    ]);
    assert_eq!(m.rank(), 2); // Third row is sum of first two rows

    let m = Mat2::from_u8(vec![
        vec![1, 0, 0],
        vec![0, 1, 0],
        vec![0, 0, 1],
    ]);
    assert_eq!(m.rank(), 3); // Full rank matrix
}

#[test]
fn test_matrix_inverse() {
    // Test matrix inversion
    let m = Mat2::from_u8(vec![
        vec![1, 1],
        vec![0, 1],
    ]);
    let inv = m.inverse().unwrap();
    let expected = Mat2::from_u8(vec![
        vec![1, 1],
        vec![0, 1],
    ]);
    assert_eq!(inv, expected);
    assert_eq!(m * inv, Mat2::id(2)); // M * M^-1 = I

    // Test non-invertible matrix
    let m = Mat2::from_u8(vec![
        vec![1, 1],
        vec![1, 1],
    ]);
    assert!(m.inverse().is_none());
}

#[test]
fn test_matrix_solve() {
    // Test solving linear system
    let m = Mat2::from_u8(vec![
        vec![1, 1],
        vec![0, 1],
    ]);
    let b = Mat2::from_u8(vec![
        vec![1],
        vec![0],
    ]);
    let x = m.solve(&b).unwrap();
    let expected = Mat2::from_u8(vec![
        vec![1],
        vec![0],
    ]);
    assert_eq!(x, expected);
    assert_eq!(m * x, b); // M * x = b

    // Test system with no solution
    let m = Mat2::from_u8(vec![
        vec![1, 1],
        vec![1, 1],
    ]);
    let b = Mat2::from_u8(vec![
        vec![1],
        vec![0],
    ]);
    assert!(m.solve(&b).is_none());
}

#[test]
fn test_matrix_nullspace() {
    // Test nullspace computation
    let m = Mat2::from_u8(vec![
        vec![1, 1, 0],
        vec![0, 1, 1],
    ]);
    let nullspace = m.nullspace(true);
    assert_eq!(nullspace.len(), 1); // Dimension of nullspace should be 1
    let v = &nullspace[0];
    assert_eq!(m * v.clone(), Mat2::zeros(2, 1)); // M * v = 0

    // Test matrix with trivial nullspace
    let m = Mat2::from_u8(vec![
        vec![1, 0],
        vec![0, 1],
    ]);
    let nullspace = m.nullspace(true);
    assert_eq!(nullspace.len(), 0); // Dimension of nullspace should be 0
}

#[test]
fn test_matrix_factorization() {
    // Test matrix factorization
    let m = Mat2::from_u8(vec![
        vec![1, 1, 0],
        vec![0, 1, 1],
        vec![1, 0, 1],
    ]);
    let (m0, m1) = m.factor();
    assert_eq!(m0.cols(), m1.rows()); // Dimensions should match
    assert_eq!(m0.rows(), m.rows()); // m0 should have same number of rows as m
    assert_eq!(m1.cols(), m.cols()); // m1 should have same number of columns as m
    assert_eq!(m0 * m1, m); // m0 * m1 should equal m
} 