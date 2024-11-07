use std::process::Output;

use faer::utils::constrained::Array;
use ndarray::arr1;
use ndarray::arr2;
use ndarray::Array1;

pub fn part_3_ndarray() {
    let inputs = arr1(&[1.0, 2.0, 3.0, 2.5]);
    let weights1 = [0.2, 0.8, -0.5, 1.0];
    let weights2 = [0.5, -0.91, 0.26, -0.5];
    let weights3 = [-0.26, -0.27, 0.17, 0.87];

    let weights = arr2(&[weights1, weights2, weights3]);
    let biases = arr1(&[2.0, 3.0, 0.5]);
    let output = weights.dot(&inputs) + biases;
    println!("{}", output);
}

// Matrix multiplication (dot product) requires:
// weights matrix: 3x4
// input vector: 4x1
// Result is 3x1 vector because:
// - Each row (3) of weights multiplies with input vector
// - Must have same number of columns (4) in weights as length of input vector
// weights.dot(input) finds dot product for each row in weights, with input then also adds
// corresponding bias to it.
