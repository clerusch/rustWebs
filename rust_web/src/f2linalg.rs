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
        if i < d {
            data[i][0] = F2::One;
        }
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
    
    /// Extract a submatrix from the matrix
    pub fn submatrix(&self, start_row: usize, start_col: usize, rows: usize, cols: usize) -> Option<Self> {
        if start_row + rows > self.rows() || start_col + cols > self.cols() {
            return None;
        }
        
        let mut result = Mat2::zeros(rows, cols);
        
        for i in 0..rows {
            for j in 0..cols {
                if let Some(val) = self.get(start_row + i, start_col + j) {
                    result.set(i, j, val);
                }
            }
        }
        
        Some(result)
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
    
    /// Horizontally stack this matrix with another matrix
    pub fn hstack(&self, other: &Mat2) -> Mat2 {
        if self.rows() != other.rows() {
            panic!("Cannot stack matrices with different number of rows");
        }
        
        let mut new_data = Vec::with_capacity(self.rows());
        
        for (row_self, row_other) in self.data.iter().zip(other.data.iter()) {
            let mut new_row = row_self.clone();
            new_row.extend_from_slice(row_other);
            new_data.push(new_row);
        }
        
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
    /// 
    /// # Arguments
    /// * `full_reduce` - If true, compute the full row-reduced form
    /// * `x` - Optional matrix to apply the same row operations to (x -> g * x)
    /// * `y` - Optional matrix to apply the inverse column operations to (y -> y * g^-1)
    /// * `blocksize` - Size of blocks for optimization (Patel/Markov/Hayes optimization)
    /// * `pivot_cols` - Output parameter that will contain the pivot columns
    pub fn gauss(&mut self, full_reduce: bool, mut x: Option<&mut Mat2>, mut y: Option<&mut Mat2>, blocksize: usize, pivot_cols: &mut Vec<usize>) -> usize {
        let rows = self.rows();
        let cols = self.cols();
        let mut rank = 0;
        
        // Process in blocks for optimization
        for sec in 0..(cols + blocksize - 1) / blocksize {
            let i0 = sec * blocksize;
            let i1 = std::cmp::min(cols, (sec + 1) * blocksize);
            
            // Process the current block
            for p in i0..i1 {
                // Find pivot row
                let mut pivot_row = None;
                for row in rank..rows {
                    if self.data[row][p] == F2::One {
                        pivot_row = Some(row);
                        break;
                    }
                }

                if let Some(pivot) = pivot_row {
                    // Swap rows if needed
                    if pivot != rank {
                        self.row_swap(rank, pivot);
                        if let Some(x_mat) = &mut x {
                            x_mat.row_swap(rank, pivot);
                        }
                        if let Some(y_mat) = &mut y {
                            y_mat.col_swap(pivot, rank);
                        }
                    }


                    // Eliminate other rows in the current block
                    for row in rank + 1..rows {
                        if self.data[row][p] == F2::One {
                            self.row_add(rank, row);
                            if let Some(x_mat) = &mut x {
                                x_mat.row_add(rank, row);
                            }
                            if let Some(y_mat) = &mut y {
                                y_mat.col_add(row, rank);
                            }
                        }
                    }
                    
                    pivot_cols.push(p);
                    rank += 1;
                    
                    if rank == rows {
                        return rank;
                    }
                }
            }
            
            // Eliminate duplicates in the current block
            let mut chunks = std::collections::HashMap::new();
            for row in rank..rows {
                let chunk: Vec<F2> = self.data[row][i0..i1].to_vec();
                if chunk.iter().any(|&x| x == F2::One) {
                    if let Some(&r) = chunks.get(&chunk) {
                        self.row_add(r, row);
                        if let Some(x_mat) = &mut x {
                            x_mat.row_add(r, row);
                        }
                        if let Some(y_mat) = &mut y {
                            y_mat.col_add(row, r);
                        }
                    } else {
                        chunks.insert(chunk, row);
                    }
                }
            }
        }
        
        // Full reduction if requested
        if full_reduce && !pivot_cols.is_empty() {
            let mut pivot_cols_rev = pivot_cols.clone();
            pivot_cols_rev.reverse();
            
            for &pcol in &pivot_cols_rev {
                // Find the pivot row for this column
                if let Some(pivot_row) = (0..rank).find(|&r| self.data[r][pcol] == F2::One) {
                    // Eliminate above the pivot
                    for row in 0..pivot_row {
                        if self.data[row][pcol] == F2::One {
                            self.row_add(pivot_row, row);
                            if let Some(x_mat) = &mut x {
                                x_mat.row_add(pivot_row, row);
                            }
                            if let Some(y_mat) = &mut y {
                                y_mat.col_add(row, pivot_row);
                            }
                        }
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
        
        let n = self.rows();
        
        // Special case for 2x2 matrices in F2
        if n == 2 {
            let a = self.data[0][0];
            let b = self.data[0][1];
            let c = self.data[1][0];
            let d = self.data[1][1];
            
            // Compute determinant
            let det = a * d + b * c;  // In F2, addition is XOR, so this is determinant
            
            if det == F2::Zero {
                return None;  // Not invertible
            }
            
            // For F2, the inverse is [d b; c a] / det, but since det=1 in F2, it's just [d b; c a]
            let inv = Mat2::from_u8(vec![
                vec![d as u8, b as u8],
                vec![c as u8, a as u8],
            ]);
            
            return Some(inv);
        }
        
        // General case for n x n matrices
        
        // Create augmented matrix [self | I]
        let mut aug = Mat2::zeros(n, 2 * n);
        for i in 0..n {
            for j in 0..n {
                aug.data[i][j] = self.data[i][j];
                aug.data[i][j + n] = if i == j { F2::One } else { F2::Zero };
            }
        }
        
        // Perform Gaussian elimination to get [I | inv]
        let mut pivot_cols = Vec::new();
        let rank = aug.gauss(true, None, None, 6, &mut pivot_cols);
        
        // If the matrix is not full rank, it's not invertible
        if rank < n {
            return None;
        }
        
        // Extract the inverse from the right half of the augmented matrix
        let mut inv = Mat2::zeros(n, n);
        for i in 0..n {
            for j in 0..n {
                inv.data[i][j] = aug.data[i][j + n];
            }
        }
        
        Some(inv)
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
        
        // Perform Gaussian elimination to get the matrix in reduced row echelon form
        let mut pivot_cols = Vec::new();
        m.gauss(true, None, None, 6, &mut pivot_cols);
        
        let cols = self.cols();
        let rank = pivot_cols.len();
        
        // If matrix is full rank, nullspace is empty
        if rank == cols {
            return Vec::new();
        }
        
        // Find non-pivot columns (free variables)
        let free_cols: Vec<usize> = (0..cols).filter(|&c| !pivot_cols.contains(&c)).collect();
        
        // For each free column, create a basis vector
        let mut basis = Vec::with_capacity(free_cols.len());
        
        for &free_col in &free_cols {
            // Start with a zero vector
            let mut vec = vec![F2::Zero; cols];
            // Set the free variable to 1
            vec[free_col] = F2::One;
            
            // Back-substitute to find the values of the pivot variables
            // We need to process rows in reverse order
            for i in (0..m.rows().min(rank)).rev() {
                // Find the pivot column for this row
                let pivot_col = match pivot_cols.get(i) {
                    Some(&col) => col,
                    None => continue,
                };
                
                // The value of the pivot variable is the sum of the products of the row elements
                // and the corresponding vector elements, for columns after the pivot
                let mut sum = F2::Zero;
                for j in (pivot_col + 1)..cols {
                    if m.data[i][j] == F2::One {
                        sum = sum + vec[j];
                    }
                }
                
                // The pivot variable is set to this sum
                if pivot_col < vec.len() {
                    vec[pivot_col] = sum;
                }
            }
            
            // Add the basis vector to the result
            basis.push(Mat2 { 
                data: vec.into_iter().map(|x| vec![x]).collect() 
            });
        }
        
        basis
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
