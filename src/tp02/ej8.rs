pub fn sumar_arreglos(arr1: [f32; 3], arr2: [f32; 3]) -> Vec<f32> {
    arr1.iter().zip(arr2.iter()).map(|(a, b)| a + b).collect()
}

#[test]
fn test_sumar_arreglos() {
    let arreglo1 = [1.1, 1.1, 1.1];
    let arreglo2 = [1.1, 1.1, 1.1];
    let result = [2.2, 2.2, 2.2];
    assert_eq!(sumar_arreglos(arreglo1, arreglo2), result);
}
