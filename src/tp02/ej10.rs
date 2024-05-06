pub fn cantidad_de_cadenas_mayor_a(arreglo: [String; 3], limite: i32) -> i32 {
    return arreglo
        .iter()
        .filter(|cadena| cadena.len() > limite as usize)
        .count() as i32;
}

#[test]
fn test_cantidad_de_cadenas_mayor_a() {
    let arreglo = [
        "aaaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
        "aaaaaaaa".to_string(),
    ];

    assert_eq!(cantidad_de_cadenas_mayor_a(arreglo, 2), 3);
}
