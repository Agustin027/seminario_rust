use std::io;
pub fn main() {
    let numeroF: f64 = 10.0;
    let mut input = String::new();
    println!("Ingresa un numero");
    io::stdin().read_line(&mut input).expect("error");

    let numeroF2: f64 = input.trim().parse().expect("error");

    println!("Resultado de la multiplicación: {}", numeroF * numeroF2);
    println!("Resultado de la división: {}", numeroF / numeroF2);
    println!("Resultado de la suma: {}", numeroF + numeroF2);
    println!("Resultado de la resta: {}", numeroF - numeroF2);
}
