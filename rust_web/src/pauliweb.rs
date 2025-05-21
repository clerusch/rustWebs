//! A Rust implementation of Pauli operator utilities for quantum computing.
//! Inspired by the Python `pauliweb` module but designed for Rust and `quizx`.

// use std::collections::HashMap;
use std::fmt;
// use num_complex::Complex64;
use quizx::circuit::*;
// use quizx::gate::*;
use thiserror::Error;

/// Represents a single-qubit Pauli operator (I, X, Y, Z)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Pauli {
    I,
    X,
    Y,
    Z,
}

/// Represents a Pauli string (tensor product of Pauli operators)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PauliString {
    ops: Vec<Pauli>,
    phase: i8,  // 0, 1, 2, 3 representing 1, i, -1, -i
}

/// Error type for Pauli operations
#[derive(Error, Debug)]
pub enum PauliError {
    #[error("Qubit index out of bounds")]
    QubitOutOfBounds,
    #[error("Incompatible Pauli string lengths")]
    IncompatibleLengths,
}

impl PauliString {
    /// Create a new Pauli string of identity operators
    pub fn new(n: usize) -> Self {
        PauliString {
            ops: vec![Pauli::I; n],
            phase: 0,
        }
    }

    /// Apply a Pauli operator to a specific qubit
    pub fn set(&mut self, qubit: usize, op: Pauli) -> Result<(), PauliError> {
        if qubit >= self.ops.len() {
            return Err(PauliError::QubitOutOfBounds);
        }
        self.ops[qubit] = op;
        Ok(())
    }
    
    /// Get the phase of the Pauli string (0, 1, 2, 3 representing 1, i, -1, -i)
    pub fn phase(&self) -> i8 {
        self.phase
    }

    /// Multiply two Pauli strings
    pub fn multiply(&self, other: &PauliString) -> Result<Self, PauliError> {
        if self.ops.len() != other.ops.len() {
            return Err(PauliError::IncompatibleLengths);
        }

        let mut result = self.clone();
        let mut phase_shift = 0;

        for (i, (a, b)) in self.ops.iter().zip(&other.ops).enumerate() {
            match (a, b) {
                (Pauli::I, _) => result.ops[i] = *b,
                (_, Pauli::I) => (),
                (Pauli::X, Pauli::X) | (Pauli::Y, Pauli::Y) | (Pauli::Z, Pauli::Z) => {
                    result.ops[i] = Pauli::I;
                }
                (Pauli::X, Pauli::Y) => {
                    result.ops[i] = Pauli::Z;
                    phase_shift += 1;
                }
                (Pauli::Y, Pauli::Z) => {
                    result.ops[i] = Pauli::X;
                    phase_shift += 1;
                }
                (Pauli::Z, Pauli::X) => {
                    result.ops[i] = Pauli::Y;
                    phase_shift += 1;
                }
                (Pauli::Y, Pauli::X) => {
                    result.ops[i] = Pauli::Z;
                    phase_shift -= 1;
                }
                (Pauli::Z, Pauli::Y) => {
                    result.ops[i] = Pauli::X;
                    phase_shift -= 1;
                }
                (Pauli::X, Pauli::Z) => {
                    result.ops[i] = Pauli::Y;
                    phase_shift -= 1;
                }
            }
        }

        result.phase = (self.phase + other.phase + phase_shift).rem_euclid(4);
        Ok(result)
    }

    /// Check if two Pauli strings commute
    pub fn commutes_with(&self, other: &PauliString) -> Result<bool, PauliError> {
        if self.ops.len() != other.ops.len() {
            return Err(PauliError::IncompatibleLengths);
        }

        let mut anticommute_count = 0;

        for (a, b) in self.ops.iter().zip(&other.ops) {
            match (a, b) {
                (Pauli::I, _) | (_, Pauli::I) => (),
                (Pauli::X, Pauli::Y) | (Pauli::Y, Pauli::X) => anticommute_count += 1,
                (Pauli::X, Pauli::Z) | (Pauli::Z, Pauli::X) => anticommute_count += 1,
                (Pauli::Y, Pauli::Z) | (Pauli::Z, Pauli::Y) => anticommute_count += 1,
                _ => (),
            }
        }

        Ok(anticommute_count % 2 == 0)
    }

    /// Convert to a circuit using quizx gates
    pub fn to_circuit(&self) -> Circuit {
        let mut circuit = Circuit::new(self.ops.len());
        
        for (i, pauli) in self.ops.iter().enumerate() {
            match pauli {
                Pauli::X => {
                    circuit.add_gate("x", vec![i]);  // Use lowercase 'x' for QASM compatibility
                }
                Pauli::Y => {
                    // Y = iXZ, so we need to implement this as Z followed by X
                    circuit.add_gate("z", vec![i]);  // Use lowercase 'z' for QASM compatibility
                    circuit.add_gate("x", vec![i]);  // Use lowercase 'x' for QASM compatibility
                }
                Pauli::Z => {
                    circuit.add_gate("z", vec![i]);  // Use lowercase 'z' for QASM compatibility
                }
                Pauli::I => (),
            }
        }

        circuit
    }
}

impl fmt::Display for Pauli {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Pauli::I => "I",
                Pauli::X => "X",
                Pauli::Y => "Y",
                Pauli::Z => "Z",
            }
        )
    }
}

impl fmt::Display for PauliString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let phase_str = match self.phase {
            0 => "",
            1 => "i",
            2 => "-",
            3 => "-i",
            _ => unreachable!(),
        };
        
        write!(f, "{}", phase_str)?;
        for pauli in &self.ops {
            write!(f, "{}", pauli)?;
        }
        Ok(())
    }
}

/// Example usage
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pauli_multiplication() -> Result<(), PauliError> {
        // Test X * Y = iZ
        let mut x = PauliString::new(1);
        x.set(0, Pauli::X)?;
        
        let mut y = PauliString::new(1);
        y.set(0, Pauli::Y)?;
        
        let result = x.multiply(&y)?;
        assert_eq!(result.to_string(), "iZ");
        
        Ok(())
    }

    #[test]
    fn test_commutation() -> Result<(), PauliError> {
        let mut x = PauliString::new(2);
        x.set(0, Pauli::X)?;
        
        let mut z = PauliString::new(2);
        z.set(1, Pauli::Z)?;
        
        // X ⊗ I and I ⊗ Z should commute
        assert!(x.commutes_with(&z)?);
        
        Ok(())
    }
}