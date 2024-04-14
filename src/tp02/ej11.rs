pub fn multiplicar_valores(arr: &mut [i32; 5], factor: i32) {
    arr.iter_mut().for_each(|num| *num *= factor); // no uso .iter.map por que no modifica el arreglo original, sino que crea un nuevo vector con los valores modificados.
}
