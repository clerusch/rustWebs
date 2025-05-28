use criterion::{criterion_group, criterion_main, Criterion};
use bitvec::prelude::*;
use rust_web::bitwisef2linalg::Mat2 as BitMat2;
use quizx::linalg::Mat2 as QuizxMat2;
use rand::Rng;
use std::time::Duration;

// Generate a random sparse matrix with given size and density
fn generate_random_matrix(size: usize, density: f64) -> (BitVec<usize>, Vec<Vec<u8>>) {
    let mut rng = rand::thread_rng();
    let mut bitvec = bitvec![0; size * size];
    let mut vec_mat = vec![vec![0u8; size]; size];
    
    for i in 0..size {
        for j in 0..size {
            if rng.gen_bool(density) {
                bitvec.set(i * size + j, true);
                vec_mat[i][j] = 1;
            }
        }
    }
    
    (bitvec, vec_mat)
}

// Convert a bitvec matrix to quizx matrix format
fn to_quizx_matrix(size: usize, bitvec: &BitVec<usize>) -> QuizxMat2 {
    let mut data = vec![vec![0u8; size]; size];
    for i in 0..size {
        for j in 0..size {
            if bitvec[i * size + j] {
                data[i][j] = 1;
            }
        }
    }
    QuizxMat2::new(data)
}

// Create a BitMat2 from a bitvector and dimensions
fn create_bitmat2(bitvec: &BitVec<usize>, rows: usize, cols: usize) -> BitMat2 {
    let mut mat = BitMat2::new(rows, cols);
    for i in 0..rows {
        for j in 0..cols {
            mat.set(i, j, bitvec[i * cols + j]);
        }
    }
    mat
}

fn bench_matrix_operations(c: &mut Criterion) {
    let sizes = [400];
    let density = 0.1; // 10% density for sparse matrices
    
    let mut group = c.benchmark_group("Matrix Operations");
    group.measurement_time(Duration::from_secs(10))  // Measure for 10 seconds per benchmark
        .warm_up_time(Duration::from_secs(3))       // Warm up for 3 seconds
        .sample_size(100);      
    
    for &size in &sizes {
        // Generate test data
        let (bitvec_data, _vec_data) = generate_random_matrix(size, density);
        
        // Create matrices
        let bitmat = create_bitmat2(&bitvec_data, size, size);
        let quizx_mat = to_quizx_matrix(size, &bitvec_data);
        
        // Clone for operations that consume the matrix
        let bitmat2 = bitmat.clone();
        let quizx_mat2 = quizx_mat.clone();
        
        // Benchmark matrix multiplication
        group.bench_function(
            &format!("bitwise_mat_mul_{}x{}", size, size),
            |b| b.iter(|| {
                let _ = bitmat.clone() * bitmat2.clone();
            })
        );
        
        group.bench_function(
            &format!("quizx_mat_mul_{}x{}", size, size),
            |b| b.iter(|| {
                let _ = quizx_mat.clone() * quizx_mat2.clone();
            })
        );
        
        // Note: quizx Mat2 doesn't implement Add, so we only test multiplication
    }
    
    group.finish();
}

criterion_group!(benches, bench_matrix_operations);
criterion_main!(benches);
