pub fn sumar_arreglos(arr1: [f32; 10], arr2: [f32; 10]) -> Vec<f32> {
    return arr1.iter().zip(arr2.iter()).map(|(a, b)| a + b).collect();
}
