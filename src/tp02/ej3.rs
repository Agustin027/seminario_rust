pub fn suma_pares(arreglo: [i32; 2]) -> i32 {
    arreglo.iter().filter(|&num| num % 2 == 0).sum()
}

#[test]
fn test_suma_pares() {
    let arreglo = [4, 2];
    assert_eq!(suma_pares(arreglo), 6);
}
