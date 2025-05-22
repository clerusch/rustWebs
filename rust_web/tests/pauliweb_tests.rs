// use rust_web::pauliweb2::{Pauli, PauliString, PauliError};
// // use quizx::circuit::Circuit;

// #[test]
// fn test_pauli_creation() {
//     // Test Pauli enum variants
//     assert_eq!(format!("{}", Pauli::I), "I");
//     assert_eq!(format!("{}", Pauli::X), "X");
//     assert_eq!(format!("{}", Pauli::Y), "Y");
//     assert_eq!(format!("{}", Pauli::Z), "Z");
// }

// #[test]
// fn test_pauli_string_creation() {
//     // Test creating a new PauliString
//     let n = 3;
//     let ps = PauliString::new(n);
//     assert_eq!(ps.to_string(), "III");
    
//     // Test setting Pauli operators
//     let mut ps = PauliString::new(3);
//     ps.set(0, Pauli::X).unwrap();
//     ps.set(1, Pauli::Y).unwrap();
//     ps.set(2, Pauli::Z).unwrap();
//     assert_eq!(ps.to_string(), "XYZ");
    
//     // Test out of bounds error
//     assert!(matches!(ps.set(3, Pauli::X), Err(PauliError::QubitOutOfBounds)));
// }

// #[test]
// fn test_pauli_multiplication() -> Result<(), PauliError> {
//     // Test multiplication of Pauli strings
//     let mut x = PauliString::new(1);
//     x.set(0, Pauli::X)?;
    
//     let mut y = PauliString::new(1);
//     y.set(0, Pauli::Y)?;
    
//     let mut z = PauliString::new(1);
//     z.set(0, Pauli::Z)?;
    
//     // Test X * Y = iZ
//     let xy = x.multiply(&y)?;
//     assert_eq!(xy.to_string(), "iZ");  // Includes phase 'i'
//     assert_eq!(xy.phase(), 1);  // i phase
    
//     // Test Y * Z = iX
//     let yz = y.multiply(&z)?;
//     assert_eq!(yz.to_string(), "iX");  // Includes phase 'i'
//     assert_eq!(yz.phase(), 1);  // i phase
    
//     // Test Z * X = iY
//     let zx = z.multiply(&x)?;
//     assert_eq!(zx.to_string(), "iY");  // Includes phase 'i'
//     assert_eq!(zx.phase(), 1);  // i phase
    
//     Ok(())
// }

// #[test]
// fn test_commutation() -> Result<(), PauliError> {
//     // Test commutation relations
//     let mut x = PauliString::new(1);
//     x.set(0, Pauli::X)?;
    
//     let mut z = PauliString::new(1);
//     z.set(0, Pauli::Z)?;
    
//     // X and Z anti-commute
//     assert!(!x.commutes_with(&z)?);
    
//     // X and I commute
//     let i = PauliString::new(1);
//     assert!(x.commutes_with(&i)?);
    
//     Ok(())
// }

// #[test]
// fn test_to_circuit() -> Result<(), PauliError> {
//     // Create a Pauli string
//     let mut ps = PauliString::new(3);
//     ps.set(0, Pauli::X)?;
//     ps.set(1, Pauli::Y)?;
//     ps.set(2, Pauli::Z)?;
    
//     // Convert to circuit
//     let circuit = ps.to_circuit();
    
//     // Verify the circuit has the correct number of qubits and gates
//     assert_eq!(circuit.num_qubits(), 3);
    
//     // The circuit should have 3 gates: X on qubit 0, Z on qubit 1, X on qubit 1, and Z on qubit 2
//     // (Y is decomposed into Z followed by X)
//     assert_eq!(circuit.num_gates(), 4);
    
//     // Convert to QASM to verify the gates
//     let qasm = circuit.to_qasm();
//     println!("Generated QASM: {}", qasm);
    
//     // Check for gates in the QASM output
//     // QASM uses lowercase gate names
//     assert!(qasm.contains("x q[0]"), "Expected x gate on qubit 0");
//     assert!(qasm.contains("z q[1]"), "Expected z gate on qubit 1");
//     assert!(qasm.contains("x q[1]"), "Expected x gate on qubit 1");
//     assert!(qasm.contains("z q[2]"), "Expected z gate on qubit 2");
    
//     Ok(())
// }
