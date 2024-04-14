pub fn longitud_de_cadenas(arreglo: [String; 5]) -> Vec<i32> {
    let mut res = vec![0; arreglo.len()];
    for i in 0..arreglo.len() {
        res[i] = arreglo[i].len() as i32;
    }

    return res;
}
