use std::io;

fn main() {
    const CADENA: &str = "AAAAAEAAAAA";

    println!("Ingrese un caracter");
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("Error al leer la entrada.");
    let caracter_ingresado: char = input.trim().parse().expect("Error");
    let mut cont :i32 = 0;
    
    for caracter in CADENA.chars() {
         if Some(caracter) == Some(caracter_ingresado) {
            cont+=1;
            
         }
    }
    
    println!("{}",cont);
}
