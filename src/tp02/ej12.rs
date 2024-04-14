pub fn reemplazar_pares(arr: &mut [i32; 5]) {
    arr.iter_mut().for_each(|num| {
        if *num % 2 == 0 {
            *num = -1
        }
    });
}
