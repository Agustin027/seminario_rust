pub fn cantidad_impares(arreglo: [i32; 10]) -> i32 {
    return arreglo
        .iter()
        .filter(|&num| num % 2 != 0)
        .count()
        .try_into()
        .unwrap();
}
