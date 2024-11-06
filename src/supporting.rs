pub fn dot(input: &Vec<f32>, weight: Vec<f32>) -> f32 {
    let sum = input
        .iter()
        .zip(weight.iter())
        .map(|(a, b)| *a * b)
        .sum::<f32>();
    sum
}
