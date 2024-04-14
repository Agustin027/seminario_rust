pub fn cantidad_en_rango(arr: [i32; 5], inferior: i32, superior: i32) -> i32 {
    return arr
        .iter()
        .filter(|&x| *x >= inferior && *x <= superior)
        .count() as i32;
}
