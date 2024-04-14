pub fn cantidad_de_cadenas_mayor_a(arreglo: [String; 5], limite: i32) -> i32 {
    return arreglo
        .iter()
        .filter(|cadena| cadena.len() > limite as usize)
        .count() as i32;
}
