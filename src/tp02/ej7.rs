pub fn cantidad_de_mayores(arr: [i32; 10], limite: i32) -> i32 {
    arr.iter().filter(|&num| *num > limite).count() as i32
}

#[test]
fn test_cantidad_mayores() {
    let arreglo = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    assert_eq!(cantidad_de_mayores(arreglo, 5), 5);
}
