use bitvec::prelude::*;
use std::ops::{Add, Mul};
use std::fmt;

// Type aliases for better readability
type BitVecType = BitVec<usize, Lsb0>;

/// A matrix over F2 (the field with 2 elements) using bit-vectors for efficient storage
#[derive(Clone, Debug)]
pub struct Mat2 {
    rows: usize,
    cols: usize,
    data: Vec<BitVecType>, // Each BitVec represents a row
}

impl Mat2 {
    /// Create a new matrix from raw data (vector of rows, each as a BitVec)
    pub fn new(rows: usize, cols: usize) -> Self {
        let data = (0..rows)
            .map(|_| bitvec![0; cols])
            .collect();
        Self { rows, cols, data }
    }

    /// Create a new matrix from a 2D vector of u8 (0 or 1)
    pub fn from_u8(data: Vec<Vec<u8>>) -> Self {
        if data.is_empty() {
            return Self::new(0, 0);
        }
        let rows = data.len();
        let cols = data[0].len();
        
        let mut mat = Self::new(rows, cols);
        for (i, row) in data.into_iter().enumerate() {
            for (j, &val) in row.iter().enumerate() {
                mat.set(i, j, val != 0);
            }
        }
        mat
    }

    /// Create an identity matrix of size n x n
    pub fn id(n: usize) -> Self {
        let mut mat = Self::new(n, n);
        for i in 0..n {
            mat.set(i, i, true);
        }
        mat
    }

    /// Create a zero matrix of size m x n
    pub fn zeros(m: usize, n: usize) -> Self {
        Self::new(m, n)
    }

    /// Create a unit vector of size d with a 1 at position i
    pub fn unit_vector(d: usize, i: usize) -> Self {
        let mut mat = Self::zeros(1, d);
        mat.set(0, i, true);
        mat
    }

    /// Get the number of rows
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Get the number of columns
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Get the value at the specified position
    pub fn get(&self, row: usize, col: usize) -> bool {
        self.data[row][col]
    }

    /// Set the value at the specified position
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        self.data[row].set(col, value);
    }

    /// Vertically stack this matrix with another matrix
    pub fn vstack(&self, other: &Self) -> Self {
        assert_eq!(self.cols, other.cols, "Matrices must have same number of columns for vstack");
        let mut new_data = self.data.clone();
        new_data.extend_from_slice(&other.data);
        Self {
            rows: self.rows + other.rows,
            cols: self.cols,
            data: new_data,
        }
    }

    /// Horizontally stack this matrix with another matrix
    pub fn hstack(&self, other: &Self) -> Self {
        assert_eq!(self.rows, other.rows, "Matrices must have same number of rows for hstack");
        let mut new_data = Vec::with_capacity(self.rows);
        
        for i in 0..self.rows {
            let mut new_row = self.data[i].clone();
            new_row.extend_from_bitslice(&other.data[i]);
            new_data.push(new_row);
        }
        
        Self {
            rows: self.rows,
            cols: self.cols + other.cols,
            data: new_data,
        }
    }

    /// Add row r0 to row r1 (r1 = r1 + r0)
    /// Uses bitwise XOR for efficient F2 addition
    #[inline]
    pub fn row_add(&mut self, r0: usize, r1: usize) {
        if r0 == r1 {
            return; // Adding a row to itself in F2 is a no-op
        }
        // Create temporary copy of r0 to avoid borrow checker issues
        let row0 = self.data[r0].clone();
        self.data[r1] ^= &row0;
    }

    /// Add column c0 to column c1 (c1 = c1 + c0)
    /// Optimized to use a single XOR operation per row
    #[inline]
    pub fn col_add(&mut self, c0: usize, c1: usize) {
        if c0 == c1 {
            return; // Adding a column to itself in F2 is a no-op
        }
        for row in &mut self.data {
            // Use split_at_mut to get mutable references to both bits
            let (left, right) = row.split_at_mut(c0 + 1);
            let bit_c0 = left[c0];
            let bit_c1 = right[c1 - c0 - 1];
            row.set(c1, bit_c1 ^ bit_c0);
        }
    }

    /// Swap rows r0 and r1
    pub fn row_swap(&mut self, r0: usize, r1: usize) {
        self.data.swap(r0, r1);
    }

    /// Swap columns c0 and c1
    pub fn col_swap(&mut self, c0: usize, c1: usize) {
        for row in &mut self.data {
            let tmp = row[c0];
            // Use split_at_mut to get mutable references to both bits
            let bit = row[c1];
            row.set(c0, bit);
            row.set(c1, tmp);
        }
    }

    /// Compute the rank of the matrix using Gaussian elimination
    pub fn rank(&self) -> usize {
        let mut mat = self.clone();
        mat.gauss(false, None, None, 0, &mut Vec::new())
    }

    /// Perform Gaussian elimination with optimizations
    pub fn gauss(
        &mut self,
        full_reduce: bool,
        mut x: Option<&mut Self>,
        mut _y: Option<&mut Self>, // Not used in this implementation
        _blocksize: usize,         // For future optimization
        pivot_cols: &mut Vec<usize>,
    ) -> usize {
        let m = self.rows();
        let n = self.cols();
        let mut rank = 0;

        pivot_cols.clear();
        pivot_cols.reserve(m.min(n));


        for col in 0..n {
            // Find pivot row using iterator for better performance
            if let Some(pivot_row) = (rank..m).find(|&row| self.get(row, col)) {
                pivot_cols.push(col);

                // Swap rows if needed
                if pivot_row != rank {
                    self.row_swap(rank, pivot_row);
                    if let Some(ref mut x_mat) = x {
                        x_mat.row_swap(rank, pivot_row);
                    }
                }


                // Collect rows to process first to avoid borrowing issues
                let rows_to_process: Vec<usize> = (0..m)
                    .filter(|&r| r != rank && self.get(r, col))
                    .collect();
                
                // Process the rows
                for &row in &rows_to_process {
                    // Use row_add which now handles borrowing correctly
                    self.row_add(rank, row);
                    if let Some(ref mut x_mat) = x {
                        x_mat.row_add(rank, row);
                    }
                }

                rank += 1;


                if rank == m {
                    break;
                }
            } else if full_reduce {
                // Full reduction: clear above the pivot
                // This is the hot path, optimized for performance
                for row in 0..rank {
                    unsafe {
                        // SAFETY: We've already checked that row < rank < m
                        // and col < n in the outer loop
                        let row_ptr = self.data.as_mut_ptr().add(row);
                        if (*row_ptr)[col] { // Simplified bounds-checked access
                            (*row_ptr) ^= &self.data[rank];
                            if let Some(x_mat) = x.as_deref_mut() {
                                let x_row = x_mat.data.as_mut_ptr().add(row);
                                (*x_row) ^= &x_mat.data[rank];
                            }
                        }
                    }
                }
            }
        }

        rank
    }

    /// Compute a basis for the nullspace of the matrix
    pub fn nullspace(&self, _should_copy: bool) -> Vec<Self> {
        let mut mat = self.clone();
        let mut pivot_cols = Vec::new();
        let rank = mat.gauss(true, None, None, 0, &mut pivot_cols);
        let n = self.cols();

        if rank == n {
            return Vec::new();
        }

        // Find free variables (columns without pivots)
        let mut free_vars = Vec::with_capacity(n - rank);
        let mut pivot_iter = pivot_cols.iter().peekable();
        
        for col in 0..n {
            if let Some(&&pivot) = pivot_iter.peek() {
                if pivot == col { // Compare values directly
                    pivot_iter.next();
                    continue;
                }
            }
            free_vars.push(col);
        }

        // Generate basis vectors for the nullspace
        let mut basis = Vec::with_capacity(free_vars.len());
        
        for &free_var in &free_vars {
            let mut vec = Self::zeros(1, n);
            vec.set(0, free_var, true);
            
            // Back substitution
            for (row, &pivot_col) in pivot_cols.iter().enumerate().rev() {
                if free_var > pivot_col && mat.get(row, free_var) {
                    vec.set(0, pivot_col, true);
                }
            }
            
            basis.push(vec);
        }
        
        basis
    }

    /// Convert matrix to a vector of vectors of u8 (0 or 1)
    pub fn to_u8_vec(&self) -> Vec<Vec<u8>> {
        self.data
            .iter()
            .map(|row| row.iter().map(|b| if *b { 1 } else { 0 }).collect())
            .collect()
    }
}

impl Add for Mat2 {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        assert_eq!(self.rows, other.rows, "Matrices must have same number of rows for addition");
        assert_eq!(self.cols, other.cols, "Matrices must have same number of columns for addition");
        
        for (row_self, row_other) in self.data.iter_mut().zip(other.data.iter()) {
            *row_self ^= row_other;
        }
        
        self
    }
}

impl Mul for Mat2 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(self.cols, other.rows, "Incompatible matrix dimensions for multiplication");
        
        let mut result = Self::new(self.rows, other.cols);
        
        // Optimized matrix multiplication using bitwise operations
        for i in 0..self.rows {
            for k in 0..self.cols {
                // Skip zero elements (common in sparse matrices)
                if self.get(i, k) {
                    for j in 0..other.cols {
                        // result[i][j] ^= (self[i][k] & other[k][j])
                        // Since self[i][k] is true, this simplifies to:
                        if other.get(k, j) {
                            unsafe {
                                // SAFETY: i and j are within bounds due to loop ranges
                                let row = result.data.get_unchecked_mut(i);
                                let val = row[j];
                                row.set(j, !val);
                            }
                        }
                    }
                }
            }
        }
        
        result
    }
}

impl PartialEq for Mat2 {
    fn eq(&self, other: &Self) -> bool {
        if self.rows != other.rows || self.cols != other.cols {
            return false;
        }
        
        self.data == other.data
    }
}

impl Eq for Mat2 {}

impl fmt::Display for Mat2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.data {
            for bit in row.iter() { // No need to destructure BitRef
                write!(f, "{} ", if *bit { '1' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_matrix_creation() {
        let mat = Mat2::from_u8(vec![
            vec![1, 0, 1],
            vec![0, 1, 0],
        ]);
        
        assert_eq!(mat.rows(), 2);
        assert_eq!(mat.cols(), 3);
        assert_eq!(mat.get(0, 0), true);
        assert_eq!(mat.get(0, 1), false);
        assert_eq!(mat.get(1, 2), false);
    }
    
    #[test]
    fn test_addition() {
        let a = Mat2::from_u8(vec![
            vec![1, 0, 1],
            vec![0, 1, 0],
        ]);
        
        let b = Mat2::from_u8(vec![
            vec![1, 1, 0],
            vec![1, 1, 1],
        ]);
        
        let c = a + b;
        assert_eq!(c.get(0, 0), false);
        assert_eq!(c.get(0, 1), true);
        assert_eq!(c.get(1, 0), true);
        assert_eq!(c.get(1, 1), false);
    }
    
    #[test]
    fn test_multiplication() {
        let a = Mat2::from_u8(vec![
            vec![1, 0, 1],
            vec![0, 1, 1],
        ]);
        
        let b = Mat2::from_u8(vec![
            vec![1, 0],
            vec![1, 1],
            vec![0, 1],
        ]);
        
        let c = a * b;
        assert_eq!(c.rows(), 2);
        assert_eq!(c.cols(), 2);
        assert_eq!(c.get(0, 0), true);
        assert_eq!(c.get(0, 1), true);
        assert_eq!(c.get(1, 0), true);
        assert_eq!(c.get(1, 1), false);
    }
    
    #[test]
    fn test_rank() {
        let mat = Mat2::from_u8(vec![
            vec![1, 0, 1],
            vec![0, 1, 1],
            vec![1, 1, 0],
        ]);
        
        assert_eq!(mat.rank(), 2);
    }
    
    #[test]
    fn test_nullspace() {
        let mat = Mat2::from_u8(vec![
            vec![1, 1, 0],
            vec![1, 0, 1],
        ]);
        
        let nullspace = mat.nullspace(false);
        assert_eq!(nullspace.len(), 1);
        
        let vec = &nullspace[0];
        assert_eq!(vec.get(0, 0), true);
        assert_eq!(vec.get(0, 1), true);
        assert_eq!(vec.get(0, 2), true);
    }
}
