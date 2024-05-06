pub fn reemplazar_pares(arr: &mut [i32; 5]) {
    arr.iter_mut().for_each(|num| {
        if *num % 2 == 0 {
            *num = -1
        }
    });
}

#[test]
fn test_reemplazar_pares() {
    let mut arreglo = [2, 4, 6, 8, 77];
    let resul = [-1, -1, -1, -1, 77];
    reemplazar_pares(&mut arreglo);
    assert_eq!(arreglo, resul);
}
