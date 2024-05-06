use std::io;
pub fn main() {
    let numero_f: f64 = 10.0;
    let mut input = String::new();
    println!("Ingresa un numero");
    io::stdin().read_line(&mut input).expect("error");

    let numero_f2: f64 = input.trim().parse().expect("error");

    println!("Resultado de la multiplicación: {}", numero_f * numero_f2);
    println!("Resultado de la división: {}", numero_f / numero_f2);
    println!("Resultado de la suma: {}", numero_f + numero_f2);
    println!("Resultado de la resta: {}", numero_f - numero_f2);
}
