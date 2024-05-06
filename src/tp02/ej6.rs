pub fn longitud_de_cadenas(arreglo: [&str; 3]) -> Vec<i32> {
    let mut res = vec![0; arreglo.len()];
    for i in 0..arreglo.len() {
        res[i] = arreglo[i].len() as i32;
    }

    res
}

#[test]
fn test_longitud_cadenas() {
    let vec_arreglo = ["aaa", "aaaa", "aaaaa"];
    let result = [3, 4, 5];
    assert_eq!(longitud_de_cadenas(vec_arreglo), result);
}
