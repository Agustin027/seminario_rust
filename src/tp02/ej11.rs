pub fn multiplicar_valores(arr: &mut [i32; 5], factor: i32) {
    arr.iter_mut().for_each(|num| *num *= factor); // no uso .iter.map por que no modifica el arreglo original, sino que crea un nuevo vector con los valores modificados.
}

#[test]
fn test_multiplicar_valores() {
    let mut arreglo = [1, 2, 3, 4, 5];
    let result = [5, 10, 15, 20, 25];
    multiplicar_valores(&mut arreglo, 5);
    assert_eq!(arreglo, result);
}
