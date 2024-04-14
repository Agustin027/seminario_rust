pub fn suma_pares(arreglo: [i32; 10]) -> i32 {
    return arreglo.iter().filter(|&num| num % 2 == 0).sum();
}
