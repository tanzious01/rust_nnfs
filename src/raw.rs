use crate::supporting::*;

pub fn part1() {
    let input: Vec<i32> = vec![1, 2, 3];
    let weights: Vec<f32> = vec![0.2, 0.8, -0.5];
    let bias: i32 = 2;
    let output = input
        .iter()
        .zip(weights.iter())
        .map(|(a, b)| b * (*a as f32))
        .sum::<f32>()
        - bias as f32;
    println!("output: {:?}", output);
}
// each input has a specific weight, each neuron in first layer would take all the inputs, so that
// neuron has 3 inputs, but because its a singular neuron, it has 1 bias. Num inputs = Num weihts,
// num neuron = num bias.
//

pub fn part2() {
    // given 4 inputs, create code that finds the output value for 3 neurons with different weights
    // and biases., output)
    let inputs = vec![1.0, 2.0, 3.0, 2.5];
    let weights1 = vec![0.2, 0.8, -0.5, 1.0];
    let weights2 = vec![0.5, -0.91, 0.26, -0.5];
    let weights3 = vec![-0.26, -0.27, 0.17, 0.87];

    let bias1 = 2.0;
    let bias2 = 3.0;
    let bias3 = 0.5;

    let output1 = dot(&inputs, &weights1) + bias1;
    let output2 = dot(&inputs, &weights2) + bias2;
    let output3 = dot(&inputs, &weights3) + bias3;
    let output_vec = vec![output1, output2, output3];

    println!("part2 output: {:?}", output_vec);
}

pub fn part3() {
    //although i already made a dot product function in supporting.rs i am following the for loop
    //method found in the third video.
    let inputs = vec![1.0, 2.0, 3.0, 2.5];
    let weights1 = vec![0.2, 0.8, -0.5, 1.0];
    let weights2 = vec![0.5, -0.91, 0.26, -0.5];
    let weights3 = vec![-0.26, -0.27, 0.17, 0.87];

    let weights = vec![weights1, weights2, weights3];
    let biases = vec![2.0, 3.0, 0.5];

    let mut layered: Vec<f32> = Vec::new();
    let zipped_weights_biases: Vec<(_, _)> = weights.iter().zip(biases.iter()).collect();
    for (weight, bias) in zipped_weights_biases {
        let mut neuron_output = 0.0;
        for (ninput, weight) in inputs.iter().zip(weight.iter()) {
            neuron_output += (ninput * weight);
        }
        neuron_output += bias;
        layered.push(neuron_output);
    }

    println!("{:?}", layered);
}
