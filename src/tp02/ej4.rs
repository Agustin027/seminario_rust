pub fn cantidad_impares(arreglo: [i32; 3]) -> i32 {
    arreglo
        .iter()
        .filter(|&num| num % 2 != 0)
        .count()
        .try_into()
        .unwrap()
}

#[test]
fn test_cantidad_impares() {
    let arreglo = [1, 3, 2];
    assert_eq!(cantidad_impares(arreglo), 2);
}
