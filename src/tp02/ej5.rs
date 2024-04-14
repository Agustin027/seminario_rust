use std::vec;

pub fn duplicar_valores(arreglo: [f64; 5]) -> Vec<f64> {
    return arreglo.iter().map(|&num| num * 2.0).collect();
}
