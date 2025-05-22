use std::ops::{Add, Mul};
use std::fmt;

/// An element of the field F2 (0 or 1)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum F2 {
    Zero,
    One,
}

impl F2 {
    pub fn from_u8(n: u8) -> Self {
        if n == 0 {
            F2::Zero
        } else {
            F2::One
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            F2::Zero => 0,
            F2::One => 1,
        }
    }
}

impl Add for F2 {
    type Output = F2;

    fn add(self, other: F2) -> F2 {
        match (self, other) {
            (F2::Zero, F2::Zero) => F2::Zero,
            (F2::Zero, F2::One) => F2::One,
            (F2::One, F2::Zero) => F2::One,
            (F2::One, F2::One) => F2::Zero,
        }
    }
}

impl Mul for F2 {
    type Output = F2;

    fn mul(self, other: F2) -> F2 {
        match (self, other) {
            (F2::Zero, _) => F2::Zero,
            (_, F2::Zero) => F2::Zero,
            (F2::One, F2::One) => F2::One,
        }
    }
}

impl fmt::Display for F2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            F2::Zero => write!(f, "0"),
            F2::One => write!(f, "1"),
        }
    }
}

/// A matrix over F2 (the field with 2 elements)
#[derive(Clone, Debug)]
pub struct Mat2 {
    /// The underlying data of the matrix, stored as a vector of rows
    pub data: Vec<Vec<F2>>,
}

impl Mat2 {
    /// Create a new matrix from raw data
    pub fn new(data: Vec<Vec<F2>>) -> Self {
        Mat2 { data }
    }

    /// Create a new matrix from raw u8 data
    pub fn from_u8(data: Vec<Vec<u8>>) -> Self {
        Mat2 {
            data: data.into_iter()
                .map(|row| row.into_iter().map(F2::from_u8).collect())
                .collect()
        }
    }

    /// Create an identity matrix of size n x n
    pub fn id(n: usize) -> Self {
        let mut data = vec![vec![F2::Zero; n]; n];
        for i in 0..n {
            data[i][i] = F2::One;
        }
        Mat2 { data }
    }

    /// Create a zero matrix of size m x n
    pub fn zeros(m: usize, n: usize) -> Self {
        Mat2 { data: vec![vec![F2::Zero; n]; m] }
    }

    /// Create a unit vector of size d with a 1 at position i
    pub fn unit_vector(d: usize, i: usize) -> Self {
        let mut data = vec![vec![F2::Zero]; d];
        data[i][0] = F2::One;
        Mat2 { data }
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.data.len()
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        if self.data.is_empty() {
            0
        } else {
            self.data[0].len()
        }
    }
    
    /// Get the value at the specified position
    pub fn get(&self, row: usize, col: usize) -> Option<F2> {
        self.data.get(row).and_then(|r| r.get(col)).copied()
    }
    
    /// Set the value at the specified position
    pub fn set(&mut self, row: usize, col: usize, value: F2) -> bool {
        if let Some(row_data) = self.data.get_mut(row) {
            if let Some(cell) = row_data.get_mut(col) {
                *cell = value;
                return true;
            }
        }
        false
    }
    
    /// Vertically stack this matrix with another matrix
    pub fn vstack(&self, other: &Mat2) -> Mat2 {
        if self.cols() != other.cols() {
            panic!("Cannot stack matrices with different number of columns");
        }
        
        let mut new_data = self.data.clone();
        new_data.extend_from_slice(&other.data);
        
        Mat2 { data: new_data }
    }

    /// Add row r0 to row r1
    pub fn row_add(&mut self, r0: usize, r1: usize) {
        let n = self.data[0].len();
        if r0 == r1 {
            // Adding a row to itself in F2 is always zero, so set to zero
            for i in 0..n {
                self.data[r1][i] = F2::Zero;
            }
        } else {
            let (row_a, row_b) = if r0 < r1 {
                let (top, bottom) = self.data.split_at_mut(r1);
                (&top[r0], &mut bottom[0])
            } else {
                let (top, bottom) = self.data.split_at_mut(r0);
                (&bottom[0], &mut top[r1])
            };
            for i in 0..n {
                if row_a[i] == F2::One {
                    row_b[i] = if row_b[i] == F2::One { F2::Zero } else { F2::One };
                }
            }
        }
    }

    /// Add column c0 to column c1
    pub fn col_add(&mut self, c0: usize, c1: usize) {
        for row in &mut self.data {
            if row[c0] == F2::One {
                row[c1] = if row[c1] == F2::One { F2::Zero } else { F2::One };
            }
        }
    }

    /// Swap rows r0 and r1
    pub fn row_swap(&mut self, r0: usize, r1: usize) {
        self.data.swap(r0, r1);
    }

    /// Swap columns c0 and c1
    pub fn col_swap(&mut self, c0: usize, c1: usize) {
        for row in &mut self.data {
            row.swap(c0, c1);
        }
    }

    /// Permute rows according to permutation p
    pub fn permute_rows(&mut self, p: &[usize]) {
        let mut new_data = vec![vec![F2::Zero; self.cols()]; self.rows()];
        for (i, &j) in p.iter().enumerate() {
            new_data[i] = self.data[j].clone();
        }
        self.data = new_data;
    }

    /// Permute columns according to permutation p
    pub fn permute_cols(&mut self, p: &[usize]) {
        let mut new_data = vec![vec![F2::Zero; self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for (j, &k) in p.iter().enumerate() {
                new_data[i][j] = self.data[i][k];
            }
        }
        self.data = new_data;
    }

    /// Compute the rank of the matrix using Gaussian elimination
    pub fn rank(&self) -> usize {
        let mut m = self.clone();
        m.gauss(false, None, None, 6, &mut vec![])
    }

    /// Perform Gaussian elimination
    pub fn gauss(&mut self, full_reduce: bool, mut x: Option<&mut Mat2>, mut y: Option<&mut Mat2>, blocksize: usize, pivot_cols: &mut Vec<usize>) -> usize {
        let rows = self.rows();
        let cols = self.cols();
        let mut pivot_row = 0;

        for sec in 0..(cols + blocksize - 1) / blocksize {
            let i0 = sec * blocksize;
            let i1 = std::cmp::min(cols, (sec + 1) * blocksize);

            // Search for duplicate chunks and eliminate them
            let mut chunks = std::collections::HashMap::new();
            for r in pivot_row..rows {
                let t: Vec<F2> = self.data[r][i0..i1].to_vec();
                if !t.iter().any(|&x| x != F2::Zero) {
                    continue;
                }
                if let Some(&prev_row) = chunks.get(&t) {
                    self.row_add(prev_row, r);
                    if let Some(ref mut x) = x {
                        x.row_add(prev_row, r);
                    }
                    if let Some(ref mut y) = y {
                        y.col_add(r, prev_row);
                    }
                } else {
                    chunks.insert(t, r);
                }
            }

            let mut p = i0;
            while p < i1 {
                for r0 in pivot_row..rows {
                    if self.data[r0][p] != F2::Zero {
                        if r0 != pivot_row {
                            self.row_add(r0, pivot_row);
                            if let Some(ref mut x) = x {
                                x.row_add(r0, pivot_row);
                            }
                            if let Some(ref mut y) = y {
                                y.col_add(pivot_row, r0);
                            }
                        }

                        for r1 in (pivot_row + 1)..rows {
                            if pivot_row != r1 && self.data[r1][p] != F2::Zero {
                                self.row_add(pivot_row, r1);
                                if let Some(ref mut x) = x {
                                    x.row_add(pivot_row, r1);
                                }
                                if let Some(ref mut y) = y {
                                    y.col_add(r1, pivot_row);
                                }
                            }
                        }
                        pivot_cols.push(p);
                        pivot_row += 1;
                        break;
                    }
                }
                p += 1;
            }
        }

        let rank = pivot_row;

        if full_reduce {
            let mut pivot_row = rank - 1;
            let mut pivot_cols1 = pivot_cols.clone();

            for sec in (0..(cols + blocksize - 1) / blocksize).rev() {
                let i0 = sec * blocksize;
                let i1 = std::cmp::min(cols, (sec + 1) * blocksize);

                // Search for duplicate chunks and eliminate them
                let mut chunks = std::collections::HashMap::new();
                for r in (0..=pivot_row).rev() {
                    let t: Vec<F2> = self.data[r][i0..i1].to_vec();
                    if !t.iter().any(|&x| x != F2::Zero) {
                        continue;
                    }
                    if let Some(&prev_row) = chunks.get(&t) {
                        self.row_add(prev_row, r);
                        if let Some(ref mut x) = x {
                            x.row_add(prev_row, r);
                        }
                        if let Some(ref mut y) = y {
                            y.col_add(r, prev_row);
                        }
                    } else {
                        chunks.insert(t, r);
                    }
                }

                while !pivot_cols1.is_empty() && i0 <= pivot_cols1[pivot_cols1.len() - 1] && pivot_cols1[pivot_cols1.len() - 1] < i1 {
                    let pcol = pivot_cols1.pop().unwrap();
                    for r in 0..pivot_row {
                        if self.data[r][pcol] != F2::Zero {
                            self.row_add(pivot_row, r);
                            if let Some(ref mut x) = x {
                                x.row_add(pivot_row, r);
                            }
                            if let Some(ref mut y) = y {
                                y.col_add(r, pivot_row);
                            }
                        }
                    }
                    if pivot_row > 0 {
                        pivot_row -= 1;
                    }
                }
            }
        }

        rank
    }

    /// Factorize the matrix into m0 * m1 where m0.cols() = m1.rows() = rank
    pub fn factor(&self) -> (Mat2, Mat2) {
        // Create identity matrix
        let mut m0 = Mat2::id(self.rows());
        
        // Copy of self
        let mut m1 = self.clone();
        
        // Produce m1 := g * m and m0 := g^-1
        let rank = m1.gauss(false, None, Some(&mut m0), 6, &mut vec![]);
        
        // Throw away zero rows in m1 and corresponding columns in m0
        let mut new_m0 = vec![vec![F2::Zero; rank]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..rank {
                new_m0[i][j] = m0.data[i][j];
            }
        }
        
        let mut new_m1 = vec![vec![F2::Zero; self.cols()]; rank];
        for i in 0..rank {
            new_m1[i] = m1.data[i].clone();
        }
        
        (Mat2 { data: new_m0 }, Mat2 { data: new_m1 })
    }

    /// Compute the inverse of the matrix if it exists
    pub fn inverse(&self) -> Option<Mat2> {
        if self.rows() != self.cols() {
            return None;
        }
        
        let mut m = self.clone();
        let mut inv = Mat2::id(self.rows());
        let rank = m.gauss(true, Some(&mut inv), None, 6, &mut vec![]);
        
        if rank < self.rows() {
            None
        } else {
            Some(inv)
        }
    }

    /// Solve the linear system M * x = b
    pub fn solve(&self, b: &Mat2) -> Option<Mat2> {
        let mut m = self.clone();
        let mut b1 = b.clone();
        let _rank = m.gauss(true, Some(&mut b1), None, 6, &mut vec![]);

        // Check for inconsistencies and set x to a particular solution
        let mut x = Mat2::zeros(m.cols(), 1);
        for i in 0..m.rows() {
            let mut got_pivot = false;
            for j in 0..m.cols() {
                if m.data[i][j] != F2::Zero {
                    got_pivot = true;
                    x.data[j][0] = b1.data[i][0];
                    break;
                }
            }
            // Zero LHS with non-zero RHS = no solutions
            if !got_pivot && b1.data[i][0] != F2::Zero {
                return None;
            }
        }
        Some(x)
    }

    /// Compute a basis for the nullspace of the matrix
    pub fn nullspace(&self, should_copy: bool) -> Vec<Mat2> {
        let mut m = if should_copy {
            self.clone()
        } else {
            self.clone() // We still need a copy to avoid mutating self
        };
        m.gauss(true, None, None, 6, &mut vec![]);
        
        let cols = self.cols();
        let mut nonpivots: Vec<usize> = (0..cols).collect();
        let mut pivots = Vec::new();
        
        for row in &m.data {
            for j in 0..cols {
                if row[j] != F2::Zero {
                    if let Some(pos) = nonpivots.iter().position(|&x| x == j) {
                        nonpivots.remove(pos);
                    }
                    if !pivots.contains(&j) {
                        pivots.push(j);
                    }
                    break;
                }
            }
        }
        
        let mut vectors = Vec::new();
        for &n in &nonpivots {
            let mut v = vec![F2::Zero; cols];
            v[n] = F2::One;
            for (row, &p) in m.data.iter().zip(&pivots) {
                if row[n] != F2::Zero && p < v.len() {
                    v[p] = F2::One;
                }
            }
            vectors.push(Mat2 { data: v.into_iter().map(|x| vec![x]).collect() });
        }
        
        vectors
    }
}

impl Add for Mat2 {
    type Output = Mat2;

    fn add(self, other: Mat2) -> Mat2 {
        let mut result = vec![vec![F2::Zero; self.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Mat2 { data: result }
    }
}

impl Mul for Mat2 {
    type Output = Mat2;

    fn mul(self, other: Mat2) -> Mat2 {
        let mut result = vec![vec![F2::Zero; other.cols()]; self.rows()];
        for i in 0..self.rows() {
            for j in 0..other.cols() {
                let mut sum = F2::Zero;
                for k in 0..self.cols() {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                result[i][j] = sum;
            }
        }
        Mat2 { data: result }
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Mat2) -> bool {
        if self.rows() != other.rows() || self.cols() != other.cols() {
            return false;
        }
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if self.data[i][j] != other.data[i][j] {
                    return false;
                }
            }
        }
        true
    }
}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            write!(f, "[ ")?;
            for val in row {
                write!(f, "{} ", val)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
} 

// Tests

#[cfg(test)]
mod tests {
    use super::*;
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

}
