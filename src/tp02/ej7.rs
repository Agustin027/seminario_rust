pub fn cantidad_de_mayores(arr: [i32; 10], limite: i32) -> i32 {
    return arr.iter().filter(|&num| *num > limite).count() as i32;
}
