pub fn ordenar_nombres(arr: &mut [&str; 3]) {
    arr.sort()
}

#[test]
fn test_ordenar_nombres() {
    let mut arreglo = ["zzzzz", "aaaaa", "eeeee"];
    let resul = ["aaaaa", "eeeee", "zzzzz"];
    ordenar_nombres(&mut arreglo);
    println!("{:?}", arreglo);
    assert_eq!(arreglo, resul);
}
