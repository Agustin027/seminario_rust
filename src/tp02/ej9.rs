pub fn cantidad_en_rango(arr: [i32; 5], inferior: i32, superior: i32) -> i32 {
    return arr
        .iter()
        .filter(|&x| *x >= inferior && *x <= superior)
        .count() as i32;
}

#[test]
fn test_cantidad_en_rango() {
    let arreglo = [1, 2, 3, 4, 5];
    let inf = 2;
    let sup = 5;
    assert_eq!(cantidad_en_rango(arreglo, inf, sup), 4);
}
