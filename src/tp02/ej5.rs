pub fn duplicar_valores(arreglo: [f64; 3]) -> Vec<f64> {
    arreglo.iter().map(|&num| num * 2.0).collect()
}

#[test]
fn test_duplicar_valores() {
    let arreglo = [1.1, 1.1, 1.1];
    let result = [2.2, 2.2, 2.2];
    assert_eq!(duplicar_valores(arreglo), result);
}
